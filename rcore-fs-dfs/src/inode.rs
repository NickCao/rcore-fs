use crate::transport::{self, Transport};
use core::any::Any;
use rcore_fs::vfs::*;
use serde::{Deserialize, Serialize};
use std::string::String;
use std::sync::Arc;
use std::usize;

const MAX_INODE_SIZE: usize = 4096;

pub struct DINode {
    trans: Arc<dyn Transport>,
    nid: u64,
    bid: u64,
}

#[derive(Serialize, Deserialize)]
pub enum DFileType {
    File,
    Dir,
    SymLink,
    CharDevice,
    BlockDevice,
    NamedPipe,
    Socket,
}

#[derive(Serialize, Deserialize)]
pub struct DMetadata {
    pub type_: DFileType,
    pub mode: u16,
}

impl DINode {
    pub fn new(trans: Arc<dyn Transport>, nid: u64, bid: u64) -> Arc<Self> {
        let mut buf = vec![0u8; MAX_INODE_SIZE];
        if trans.get(nid, bid, &mut buf).is_err() {
            trans
                .set(
                    nid,
                    bid,
                    &bincode::serialize(&DMetadata {
                        mode: 0o0755,
                        type_: DFileType::Dir,
                    })
                    .unwrap(),
                )
                .unwrap();
        }
        Arc::new(Self { trans, nid, bid })
    }
}

impl rcore_fs::vfs::INode for DINode {
    /*
       Local operations
    */

    fn read_at(&self, offset: usize, buf: &mut [u8]) -> Result<usize> {
        unimplemented!()
    }

    fn write_at(&self, offset: usize, buf: &[u8]) -> Result<usize> {
        unimplemented!()
    }

    fn sync_all(&self) -> Result<()> {
        unimplemented!()
    }

    fn sync_data(&self) -> Result<()> {
        unimplemented!()
    }

    fn resize(&self, len: usize) -> Result<()> {
        unimplemented!()
    }

    fn mmap(&self, area: MMapArea) -> Result<()> {
        unimplemented!()
    }

    fn io_control(&self, cmd: u32, data: usize) -> Result<usize> {
        unimplemented!()
    }

    fn get_entry(&self, id: usize) -> Result<String> {
        Err(FsError::EntryNotFound)
    }

    fn poll(&self) -> Result<PollStatus> {
        unimplemented!()
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn create(&self, name: &str, type_: FileType, mode: u32) -> Result<Arc<dyn INode>> {
        unimplemented!()
    }

    /*
       Local operations requiring special handling
    */

    fn metadata(&self) -> Result<Metadata> {
        let mut buf = vec![0u8; MAX_INODE_SIZE];
        let n = self.trans.get(self.nid, self.bid, &mut buf).unwrap();
        let meta: DMetadata = bincode::deserialize(&buf[..n]).unwrap();
        Ok(Metadata {
            dev: 0,
            inode: 0,
            size: 0,
            blk_size: 0,
            blocks: 0,
            atime: Timespec { sec: 0, nsec: 0 },
            mtime: Timespec { sec: 0, nsec: 0 },
            ctime: Timespec { sec: 0, nsec: 0 },
            type_: match meta.type_ {
                DFileType::Dir => FileType::Dir,
                _ => unreachable!(),
            },
            mode: meta.mode,
            nlinks: 1,
            uid: 0,
            gid: 0,
            rdev: 0,
        })
    }

    fn set_metadata(&self, metadata: &Metadata) -> Result<()> {
        unimplemented!()
    }

    /*
       Remote operations
    */

    fn find(&self, name: &str) -> Result<Arc<dyn INode>> {
        unimplemented!()
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
