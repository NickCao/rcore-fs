extern crate alloc;

use crate::transport::Transport;
use alloc::string::String;
use alloc::string::ToString;
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use bincode::config::legacy;
use bincode::serde::{decode_from_slice, encode_to_vec};
use core::any::Any;
use rcore_fs::vfs::*;
use serde::{Deserialize, Serialize};

const MAX_INODE_SIZE: usize = 4096;
const BLOCK_SIZE: usize = 512;

pub struct DINode {
    trans: Arc<dyn Transport>,
    nid: u64,
    bid: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DFileType {
    File,
    Dir,
    SymLink,
    CharDevice,
    BlockDevice,
    NamedPipe,
    Socket,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DMetadata {
    pub type_: DFileType,
    pub mode: u16,
    pub entries: Vec<(String, (u64, u64))>,
    pub blocks: Vec<(u64, u64)>,
    pub size: usize,
}

impl DINode {
    pub fn new(trans: Arc<dyn Transport>, nid: u64, bid: u64) -> Arc<Self> {
        let mut buf = vec![0u8; MAX_INODE_SIZE];
        if nid == 0 && bid == 0 && trans.get(nid, bid, &mut buf).is_err() {
            trans
                .set(
                    nid,
                    bid,
                    &bincode::serde::encode_to_vec(
                        &DMetadata {
                            mode: 0o777,
                            type_: DFileType::Dir,
                            entries: vec![],
                            blocks: vec![],
                            size: 0,
                        },
                        legacy(),
                    )
                    .unwrap(),
                )
                .unwrap();
        }
        Arc::new(Self { trans, nid, bid })
    }

    pub fn read<V>(&self, f: impl FnOnce(&DMetadata) -> V) -> Result<V> {
        let mut buf = vec![0u8; MAX_INODE_SIZE];
        let n = self.trans.get(self.nid, self.bid, &mut buf).unwrap();
        let (meta, _): (DMetadata, _) = decode_from_slice(&buf[..n], legacy()).unwrap();
        Ok(f(&meta))
    }

    pub fn modify<V>(&self, f: impl FnOnce(&mut DMetadata) -> V) -> Result<V> {
        let mut buf = vec![0u8; MAX_INODE_SIZE];
        let n = self.trans.get(self.nid, self.bid, &mut buf).unwrap();
        let (mut meta, _): (DMetadata, _) = decode_from_slice(&buf[..n], legacy()).unwrap();
        let v = f(&mut meta);
        self.trans
            .set(self.nid, self.bid, &encode_to_vec(&meta, legacy()).unwrap())
            .unwrap();
        Ok(v)
    }
}

impl rcore_fs::vfs::INode for DINode {
    /*
       Local operations
    */

    fn read_at(&self, offset: usize, dbuf: &mut [u8]) -> Result<usize> {
        let mut buf = vec![0u8; MAX_INODE_SIZE];
        let n = self.trans.get(self.nid, self.bid, &mut buf).unwrap();
        let (meta, _): (DMetadata, _) =
            bincode::serde::decode_from_slice(&buf[..n], bincode::config::legacy()).unwrap();
        let blk = offset.div_floor(BLOCK_SIZE);
        let off = offset % BLOCK_SIZE;
        if blk > meta.blocks.len() {
            return Err(FsError::InvalidParam);
        }
        let (bnid, bbid) = meta.blocks[blk];
        let len = self.trans.get(bnid, bbid, &mut buf).unwrap();
        assert_eq!(len, BLOCK_SIZE);
        let avail = if (len - off) < dbuf.len() {
            len - off
        } else {
            dbuf.len()
        };
        dbuf[..avail].copy_from_slice(&buf[off..off + avail]);
        Ok(avail)
    }

    fn write_at(&self, offset: usize, dbuf: &[u8]) -> Result<usize> {
        let mut buf = vec![0u8; MAX_INODE_SIZE];
        let n = self.trans.get(self.nid, self.bid, &mut buf).unwrap();
        let (mut meta, _): (DMetadata, _) =
            bincode::serde::decode_from_slice(&buf[..n], bincode::config::legacy()).unwrap();
        let blk = offset.div_floor(BLOCK_SIZE);
        let off = offset % BLOCK_SIZE;
        while meta.blocks.len() <= blk {
            let bb = self.trans.next();
            self.trans.set(self.nid, bb, &[0u8; BLOCK_SIZE]).unwrap();
            meta.blocks.push((self.nid, bb));
        }

        let (bnid, bbid) = meta.blocks[blk];
        let len = self.trans.get(bnid, bbid, &mut buf).unwrap();
        assert_eq!(len, BLOCK_SIZE);
        let avail = if (len - off) < dbuf.len() {
            len - off
        } else {
            dbuf.len()
        };
        buf[off..off + avail].copy_from_slice(&dbuf[..avail]);
        self.trans.set(bnid, bbid, &buf[..BLOCK_SIZE]).unwrap();
        self.trans
            .set(
                self.nid,
                self.bid,
                &bincode::serde::encode_to_vec(&meta, bincode::config::legacy()).unwrap(),
            )
            .unwrap();
        Ok(avail)
    }

    fn sync_all(&self) -> Result<()> {
        Ok(())
    }

    fn sync_data(&self) -> Result<()> {
        Ok(())
    }

    fn resize(&self, len: usize) -> Result<()> {
        self.modify(|mut meta| meta.size = len)
    }

    fn mmap(&self, area: MMapArea) -> Result<()> {
        unimplemented!()
    }

    fn io_control(&self, cmd: u32, data: usize) -> Result<usize> {
        unimplemented!()
    }

    fn get_entry(&self, id: usize) -> Result<String> {
        log::debug!("get_entry: {}", id);

        let mut buf = vec![0u8; MAX_INODE_SIZE];
        let n = self.trans.get(self.nid, self.bid, &mut buf).unwrap();
        let (meta, _): (DMetadata, _) =
            bincode::serde::decode_from_slice(&buf[..n], bincode::config::legacy()).unwrap();

        if meta.type_ != DFileType::Dir {
            return Err(FsError::NotDir);
        }

        match id {
            0 => Ok(".".to_string()),
            1 => Ok("..".to_string()),
            id => {
                if let Some(ent) = meta.entries.iter().nth(id - 2) {
                    Ok(ent.0.to_string())
                } else {
                    Err(FsError::EntryNotFound)
                }
            }
        }
    }

    fn poll(&self) -> Result<PollStatus> {
        unimplemented!()
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn create(&self, name: &str, type_: FileType, mode: u32) -> Result<Arc<dyn INode>> {
        let nid = self.nid;
        let bid = self.trans.next();
        self.trans
            .set(
                nid,
                bid,
                &bincode::serde::encode_to_vec(
                    &DMetadata {
                        mode: mode as u16,
                        type_: match type_ {
                            FileType::Dir => DFileType::Dir,
                            FileType::File => DFileType::File,
                            _ => unimplemented!(),
                        },
                        entries: vec![],
                        blocks: vec![],
                        size: 0,
                    },
                    bincode::config::legacy(),
                )
                .unwrap(),
            )
            .unwrap();

        let mut buf = vec![0u8; MAX_INODE_SIZE];
        let n = self.trans.get(self.nid, self.bid, &mut buf).unwrap();
        let (mut meta, _): (DMetadata, _) =
            bincode::serde::decode_from_slice(&buf[..n], bincode::config::legacy()).unwrap();
        meta.entries.push((name.to_string(), (nid, bid)));
        self.trans
            .set(
                self.nid,
                self.bid,
                &bincode::serde::encode_to_vec(&meta, bincode::config::legacy()).unwrap(),
            )
            .unwrap();
        Ok(DINode::new(self.trans.clone(), nid, bid))
    }

    /*
       Local operations requiring special handling
    */

    fn metadata(&self) -> Result<Metadata> {
        log::debug!("metadata: {} {}", self.nid, self.bid);

        let mut buf = vec![0u8; MAX_INODE_SIZE];
        let n = self.trans.get(self.nid, self.bid, &mut buf).unwrap();
        let (mut meta, _): (DMetadata, _) =
            bincode::serde::decode_from_slice(&buf[..n], bincode::config::legacy()).unwrap();

        Ok(Metadata {
            dev: 0,
            inode: self.bid as usize, // synth a better inode number
            size: 0,
            blk_size: 0,
            blocks: meta.blocks.len(),
            atime: Timespec { sec: 0, nsec: 0 },
            mtime: Timespec { sec: 0, nsec: 0 },
            ctime: Timespec { sec: 0, nsec: 0 },
            type_: match meta.type_ {
                DFileType::Dir => FileType::Dir,
                DFileType::File => FileType::File,
                _ => unreachable!(),
            },
            mode: meta.mode,
            nlinks: 1,
            uid: 0,
            gid: 0,
            rdev: 1,
        })
    }

    fn set_metadata(&self, metadata: &Metadata) -> Result<()> {
        Ok(())
    }

    /*
       Remote operations
    */

    fn find(&self, name: &str) -> Result<Arc<dyn INode>> {
        log::debug!("find: {}", name);

        let mut buf = vec![0u8; MAX_INODE_SIZE];
        let n = self.trans.get(self.nid, self.bid, &mut buf).unwrap();
        let (mut meta, _): (DMetadata, _) =
            bincode::serde::decode_from_slice(&buf[..n], bincode::config::legacy()).unwrap();

        if meta.type_ != DFileType::Dir {
            return Err(FsError::NotDir);
        }

        match name {
            "." => Ok(DINode::new(self.trans.clone(), self.nid, self.bid)),
            ".." => Ok(DINode::new(self.trans.clone(), 0, 0)), // FIXME
            name => {
                if let Some(ent) = meta.entries.iter().find(|(n, _)| n == name) {
                    Ok(DINode::new(self.trans.clone(), ent.1 .0, ent.1 .1))
                } else {
                    Err(FsError::EntryNotFound)
                }
            }
        }
    }

    fn link(&self, name: &str, other: &Arc<dyn INode>) -> Result<()> {
        unimplemented!()
    }

    fn unlink(&self, name: &str) -> Result<()> {
        unimplemented!()
    }

    fn move_(&self, old_name: &str, target: &Arc<dyn INode>, new_name: &str) -> Result<()> {
        unimplemented!()
    }

    fn fs(&self) -> Arc<dyn FileSystem> {
        unimplemented!()
    }
}
