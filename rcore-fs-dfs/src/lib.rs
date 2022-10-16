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

pub struct DNode {
    node: Arc<dyn INode>,
}

impl DNode {
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
        DNode::new(self.store.root_inode())
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

impl INode for DNode {
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

    fn create(&self, name: &str, type_: FileType, mode: u32) -> Result<Arc<dyn INode>> {
        // FIXME
        let node = self.node.create(name, type_, mode);
        match node {
            Ok(node) => Ok(Arc::new(DNode { node })),
            err => err,
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

    fn find(&self, name: &str) -> Result<Arc<dyn INode>> {
        // FIXME
        self.node.find(name)
    }

    fn fs(&self) -> Arc<dyn FileSystem> {
        // FIXME
        self.node.fs()
    }
}
