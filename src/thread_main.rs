use perf_test::{IoType, Opt};
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
    match opt.io_type {
        IoType::Small => perf_test::small_io::execute_thread(&opt),
        IoType::Big => perf_test::big_io::execute_thread(&opt),
    }
}
