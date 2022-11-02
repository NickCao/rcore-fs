use crate::transport::Transport;
use rcore_fs::vfs::*;
use std::sync::Arc;
use std::sync::Mutex;

pub mod inode;
pub mod transport;

pub struct DFS {
    trans: Arc<dyn Transport>,
    store: Arc<dyn FileSystem>,
}

impl FileSystem for DFS {
    fn sync(&self) -> Result<()> {
        // FIXME
        self.store.sync()
    }

    fn root_inode(&self) -> Arc<dyn INode> {
        // FIXME
        inode::DINode::new(self.trans.clone(), 0, 0)
    }

    fn info(&self) -> FsInfo {
        // FIXME
        self.store.info()
    }
}

impl DFS {
    pub fn new(trans: Arc<dyn Transport>, store: Arc<dyn FileSystem>) -> Arc<DFS> {
        Arc::new(DFS { trans, store })
    }
}
