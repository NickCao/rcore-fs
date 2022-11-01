use core::any::Any;
use rcore_fs::vfs::*;
use std::string::String;
use std::sync::Arc;

pub struct DLocalNode {
    nid: usize,
    node: Arc<dyn INode>,
}

impl DLocalNode {
    pub fn new(nid: usize, node: Arc<dyn INode>) -> Arc<Self> {
        Arc::new(Self { nid, node })
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
        Ok(DLocalNode::new(
            self.nid,
            self.node.create(name, type_, mode)?,
        ))
    }

    /*
       Local operations requiring special handling
    */

    fn metadata(&self) -> Result<Metadata> {
        let meta = self.node.metadata()?;
        // only support low inode numbers
        assert_eq!(meta.inode & 0xffffffff00000000, 0);
        Ok(Metadata {
            // concat inode number with nodeid
            inode: (meta.inode & 0x00000000ffffffff) + (self.nid << 32),
            ..meta
        })
    }

    fn set_metadata(&self, metadata: &Metadata) -> Result<()> {
        self.node.set_metadata(metadata)
    }

    /*
       Remote operations
    */

    fn find(&self, name: &str) -> Result<Arc<dyn INode>> {
        let node = self.node.find(name)?;
        Ok(match node.metadata()?.type_ {
            // for shadow inodes, we create a remote node
            // for others, we create a local node
            _ => DLocalNode::new(self.nid, node),
        })
    }

    fn link(&self, name: &str, other: &Arc<dyn INode>) -> Result<()> {
        unimplemented!()
    }

    fn unlink(&self, name: &str) -> Result<()> {
        // FIXME: handle the case when the link is remote
        self.node.unlink(name)
    }

    fn move_(&self, old_name: &str, target: &Arc<dyn INode>, new_name: &str) -> Result<()> {
        match target.metadata()?.type_ {
            // TODO: figure out how to move to remote
            _ => self.node.move_(old_name, target, new_name),
        }
    }

    fn fs(&self) -> Arc<dyn FileSystem> {
        // FIXME
        self.node.fs()
    }
}
