use rcore_fs::vfs::*;
use std::sync::Arc;

pub mod inode;
pub mod transport;

pub struct DFS {
    nid: u64,
    store: Arc<dyn FileSystem>,
}

impl FileSystem for DFS {
    fn sync(&self) -> Result<()> {
        // FIXME
        self.store.sync()
    }

    fn root_inode(&self) -> Arc<dyn INode> {
        // FIXME
        inode::DINode::new(0, 0)
    }

    fn info(&self) -> FsInfo {
        // FIXME
        self.store.info()
    }
}

impl DFS {
    pub fn new(nid: u64, store: Arc<dyn FileSystem>) -> Arc<DFS> {
        Arc::new(DFS { nid, store })
    }
}
