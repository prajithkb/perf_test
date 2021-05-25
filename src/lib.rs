use std::{error::Error, fmt::Display};

use structopt::{clap::arg_enum, StructOpt};
pub mod big_io;
pub mod load_test_thread;
pub mod small_io;

pub type Result<T> = std::result::Result<T, Box<dyn Send + Sync + Error>>;

arg_enum! {
    #[derive(Debug)]
    pub enum IoType {
        Big,
        Small,
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opt {
    // The number of tasks
    #[structopt(short = "t", long = "tasks")]
    pub number_of_tasks: usize,
    // The number of files
    #[structopt(short = "f", long = "files")]
    pub number_of_files: u32,
    // The max count
    #[structopt(short = "c", long = "count")]
    pub max_count: u32,
    // The max count
    #[structopt(short = "i", long = "io_type")]
    pub io_type: IoType,
}

impl Display for Opt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "number_of_tasks={}/files={}/count={}",
            self.number_of_tasks, self.number_of_files, self.max_count
        ))
    }
}
