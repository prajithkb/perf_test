use perf_test::{IoType, Opt};
use structopt::StructOpt;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    let opt = Opt::from_args();
    match opt.io_type {
        IoType::Small => perf_test::small_io::execute_async(&opt).await,
        IoType::Big => perf_test::big_io::execute_async(&opt).await,
    }
}
