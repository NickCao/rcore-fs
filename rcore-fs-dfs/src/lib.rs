use core::any::Any;
use rcore_fs::vfs::*;
use std::string::String;
use std::sync::Arc;
pub mod local;
pub mod remote;

pub struct DFS {
    nid: usize,
    store: Arc<dyn FileSystem>,
}

impl FileSystem for DFS {
    fn sync(&self) -> Result<()> {
        // FIXME
        self.store.sync()
    }

    fn root_inode(&self) -> Arc<dyn INode> {
        // FIXME
        local::DLocalNode::new(self.nid, self.store.root_inode())
    }

    fn info(&self) -> FsInfo {
        // FIXME
        self.store.info()
    }
}

impl DFS {
    pub fn new(nid: usize, rid: usize, store: Arc<dyn FileSystem>) -> Arc<DFS> {
        Arc::new(DFS { nid, store })
    }
}
