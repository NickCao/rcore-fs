use core::any::Any;
use rcore_fs::vfs::*;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::string::String;
use std::sync::{Arc, Weak};
use std::sync::{Mutex, MutexGuard};

#[macro_use]
extern crate log;

pub struct DFS {
    nid: usize,
    store: Arc<dyn FileSystem>,
}

pub struct DLocalNode {
    node: Arc<dyn INode>,
}

impl DLocalNode {
    fn new(node: Arc<dyn INode>) -> Arc<Self> {
        Arc::new(Self { node })
    }
}

pub struct DRemoteNode {
    node: Arc<dyn INode>,
}

impl DRemoteNode {
    fn new(node: Arc<dyn INode>) -> Arc<Self> {
        Arc::new(Self { node })
    }
}

impl FileSystem for DFS {
    fn sync(&self) -> Result<()> {
        // FIXME
        self.store.sync()
    }

    fn root_inode(&self) -> Arc<dyn INode> {
        // FIXME
        DLocalNode::new(self.store.root_inode())
    }

    fn info(&self) -> FsInfo {
        // FIXME
        self.store.info()
    }
}

impl DFS {
    pub fn new(nid: usize, store: Arc<dyn FileSystem>) -> Arc<DFS> {
        Arc::new(DFS { nid, store })
    }
}

impl INode for DLocalNode {
    /*
       Local operations
    */

    fn read_at(&self, offset: usize, buf: &mut [u8]) -> Result<usize> {
        self.node.read_at(offset, buf)
    }

    fn write_at(&self, offset: usize, buf: &[u8]) -> Result<usize> {
        self.node.write_at(offset, buf)
    }

    fn sync_all(&self) -> Result<()> {
        self.node.sync_all()
    }

    fn sync_data(&self) -> Result<()> {
        self.node.sync_data()
    }

    fn resize(&self, len: usize) -> Result<()> {
        self.node.resize(len)
    }

    fn mmap(&self, area: MMapArea) -> Result<()> {
        self.node.mmap(area)
    }

    fn io_control(&self, cmd: u32, data: usize) -> Result<usize> {
        self.node.io_control(cmd, data)
    }

    fn get_entry(&self, id: usize) -> Result<String> {
        self.node.get_entry(id)
    }

    fn poll(&self) -> Result<PollStatus> {
        self.node.poll()
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn create(&self, name: &str, type_: FileType, mode: u32) -> Result<Arc<dyn INode>> {
        // when creating a new inode in a local directory
        // the new inode is still local
        assert_ne!(type_, FileType::Shadow);
        Ok(DLocalNode::new(self.node.create(name, type_, mode)?))
    }

    /*
       Local operations requiring special handling
    */

    fn metadata(&self) -> Result<Metadata> {
        // TODO: could use some metadata rewrites
        self.node.metadata()
    }

    fn set_metadata(&self, metadata: &Metadata) -> Result<()> {
        // TODO: could use some metadata rewrites
        self.node.set_metadata(metadata)
    }

    /*
       Remote operations
    */

    fn find(&self, name: &str) -> Result<Arc<dyn INode>> {
        let node = self.node.find(name)?;
        Ok(match node.metadata()?.type_ {
            // for shadow inodes, we create a remote node
            FileType::Shadow => DRemoteNode::new(node),
            // for others, we create a local node
            _ => DLocalNode::new(node),
        })
    }

    fn link(&self, name: &str, other: &Arc<dyn INode>) -> Result<()> {
        match other.metadata()?.type_ {
            // TODO: figure out how to link shadow inodes
            FileType::Shadow => unimplemented!(),
            _ => self.node.link(name, other),
        }
    }

    fn unlink(&self, name: &str) -> Result<()> {
        // FIXME: handle the case when the link is remote
        self.node.unlink(name)
    }

    fn move_(&self, old_name: &str, target: &Arc<dyn INode>, new_name: &str) -> Result<()> {
        match target.metadata()?.type_ {
            // TODO: figure out how to move to remote
            FileType::Shadow => unimplemented!(),
            _ => self.node.move_(old_name, target, new_name),
        }
    }

    fn fs(&self) -> Arc<dyn FileSystem> {
        // FIXME
        self.node.fs()
    }
}

impl INode for DRemoteNode {
    /*
       Local operations
    */

    fn read_at(&self, _offset: usize, _buf: &mut [u8]) -> Result<usize> {
        unimplemented!()
    }

    fn write_at(&self, _offset: usize, _buf: &[u8]) -> Result<usize> {
        unimplemented!()
    }

    fn sync_all(&self) -> Result<()> {
        unimplemented!()
    }

    fn sync_data(&self) -> Result<()> {
        unimplemented!()
    }

    fn resize(&self, _len: usize) -> Result<()> {
        unimplemented!()
    }

    fn mmap(&self, _area: MMapArea) -> Result<()> {
        unimplemented!()
    }

    fn io_control(&self, _cmd: u32, _data: usize) -> Result<usize> {
        unimplemented!()
    }

    fn get_entry(&self, _id: usize) -> Result<String> {
        unimplemented!()
    }

    fn poll(&self) -> Result<PollStatus> {
        unimplemented!()
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn create(&self, _name: &str, _type_: FileType, _mode: u32) -> Result<Arc<dyn INode>> {
        unimplemented!()
    }

    /*
       Local operations requiring special handling
    */

    fn metadata(&self) -> Result<Metadata> {
        unimplemented!()
    }

    fn set_metadata(&self, _metadata: &Metadata) -> Result<()> {
        unimplemented!()
    }

    /*
       Remote operations
    */

    fn find(&self, _name: &str) -> Result<Arc<dyn INode>> {
        unimplemented!()
    }

    fn link(&self, _name: &str, _other: &Arc<dyn INode>) -> Result<()> {
        unimplemented!()
    }

    fn unlink(&self, _name: &str) -> Result<()> {
        unimplemented!()
    }

    fn move_(&self, _old_name: &str, _target: &Arc<dyn INode>, _new_name: &str) -> Result<()> {
        unimplemented!()
    }

    fn fs(&self) -> Arc<dyn FileSystem> {
        unimplemented!()
    }
}
