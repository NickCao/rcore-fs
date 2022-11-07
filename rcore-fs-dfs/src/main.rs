fn main() {
    #[cfg(feature = "std")]
    {
        use dfs::transport::loopback::LoopbackTransport;
        use rcore_fs_dfs as dfs;
        use rcore_fs_fuse::fuse::VfsFuse;
        use rcore_fs_ramfs as ramfs;
        use std::{path::PathBuf, sync::Arc};
        use structopt::StructOpt;

        #[derive(Debug, StructOpt)]
        struct Opt {
            /// Target directory
            #[structopt(parse(from_os_str))]
            dir: PathBuf,
            #[structopt()]
            idx: usize,
        }

        simple_logger::SimpleLogger::new().init().unwrap();

        let opt = Opt::from_args();
        let store = ramfs::RamFS::new();
        let trans = LoopbackTransport::new(opt.idx as u64, 1, 3000).unwrap();
        let fs = dfs::DFS::new(Arc::new(trans), store);
        fuse::mount(VfsFuse::new(fs), &opt.dir, &[]).expect("failed to mount fs");
    }
}
