#![cfg_attr(not(feature = "std"), no_std)]
#![feature(int_roundings)]
extern crate alloc;

use crate::transport::Transport;
use alloc::sync::Arc;
use rcore_fs::vfs::*;

pub mod inode;
pub mod transport;

/// Distributed filesystem
pub struct DFS {
    trans: Arc<dyn Transport>,
}

impl FileSystem for DFS {
    fn sync(&self) -> Result<()> {
        Ok(())
    }

    fn root_inode(&self) -> Arc<dyn INode> {
        inode::DINode::new(self.trans.clone(), 0, 0)
    }

    fn info(&self) -> FsInfo {
        FsInfo {
            bsize: 0,
            frsize: 0,
            blocks: 0,
            bfree: 0,
            bavail: 0,
            files: 0,
            ffree: 0,
            namemax: 0,
        }
    }
}

impl DFS {
    /// create DFS from transport
    pub fn new(trans: Arc<dyn Transport>) -> Arc<DFS> {
        Arc::new(DFS { trans })
    }
}
