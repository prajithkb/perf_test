use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use perf_test::{Opt, small_io::execute_thread};
use structopt::StructOpt;
pub fn thread_benchmark(c: &mut Criterion) {
    let opt = dbg!(Opt::from_iter(&[
        "thread_perf",
        "--count",
        "1000",
        "--files",
        "100",
        "--tasks",
        "1000",
    ]));
    c.bench_with_input(BenchmarkId::new("thread", &opt), &opt, |b, s| {
        b.iter(|| execute_thread(s));
    });
}

// pub fn async_benchmark(c: &mut Criterion) {
//     let opt = dbg!(Opt::from_iter(&[
//         "thread_perf",
//         "--count",
//         "1000",
//         "--files",
//         "100",
//         "--tasks",
//         "1000",
//     ]));
//     c.bench_with_input(BenchmarkId::new("thread", &opt), &opt, |b, s| {
//         b.iter(|| execute_thread(s));
//     });
// }

criterion_group!(benches, thread_benchmark);
criterion_main!(benches);
