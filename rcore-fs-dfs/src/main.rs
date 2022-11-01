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
    let mut tp1 = LoopbackTransport::new(0, 2, 3000).unwrap();
    let mut tp2 = LoopbackTransport::new(1, 2, 3000).unwrap();
    tp1.send(1, "hello".as_bytes()).unwrap();
    let mut buf = vec![0u8; 4096];
    let n = tp2.recv(&mut buf).unwrap();
    println!("{:?}", &buf[..n]);
}
