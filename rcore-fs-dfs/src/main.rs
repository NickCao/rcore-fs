use dfs::transport::{LoopbackTransport, Transport};
use rcore_fs::vfs::FileSystem;
use rcore_fs_dfs as dfs;
use rcore_fs_fuse::fuse::VfsFuse;
use rcore_fs_ramfs as ramfs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Target directory
    #[structopt(parse(from_os_str))]
    dir: PathBuf,
}

fn main() {
    /*
    let opt = Opt::from_args();
    let store = ramfs::RamFS::new();
    let fs = dfs::DFS::new(0, 0, store);
    fuse::mount(VfsFuse::new(fs), &opt.dir, &[]).expect("failed to mount fs");
    */
}
