use core::any::Any;
use rcore_fs::vfs::*;
use std::string::String;
use std::sync::Arc;

pub struct DRemoteNode {
    node: Arc<dyn INode>,
}

impl DRemoteNode {
    pub fn new(node: Arc<dyn INode>) -> Arc<Self> {
        Arc::new(Self { node })
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
