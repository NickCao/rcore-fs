use core::any::Any;
use rcore_fs::vfs::*;
use std::string::String;
use std::sync::Arc;

pub struct DINode {
    nid: u64,
    bid: u64,
}

impl DINode {
    pub fn new(nid: u64, bid: u64) -> Arc<Self> {
        Arc::new(Self { nid, bid })
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
        unimplemented!()
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
        unimplemented!()
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
