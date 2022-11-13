//! Distributed filesystem
//!
//! Distributed filesystems has been around for a long time,
//! from the 9pfs protocol to NFS, GlusterFS and CEPH.
//! However many of these implementations are deeply tied
//! with specific networking or storage systems, limiting
//! their application and deployment. And they are mostly
//! designed from ground up, missing out from the recent
//! advancements in filesystem design.
//!
//! Our goal is to have a common abstraction of the
//! networking and storage systems, on which distributed
//! filesytems can build upon, together with a reference
//! implementation that showcases it's feasibility.

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
