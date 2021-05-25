use std::sync::atomic::AtomicU32;
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
    str::FromStr,
    sync::Arc,
    time::Instant,
};

use crate::Result;

use threadpool::ThreadPool;
use tokio::{
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
    task::JoinHandle,
};

use crate::Opt;

pub async fn execute_async(opt: &Opt) {
    tokio(dbg!(opt)).await.expect("tokio failed");
}
async fn tokio(opt: &Opt) -> Result<()> {
    // Create the runtime
    let start_time = Instant::now();
    create_files(opt.number_of_files)?;
    let max_count = opt.max_count;
    let number_of_files = opt.number_of_files;
    let counter: Arc<AtomicU32> = Arc::new(AtomicU32::new(0));
    let mut tasks: Vec<JoinHandle<()>> = Vec::new();
    for i in 0..number_of_files {
        let c = counter.clone();
        let file = PathBuf::from_str(format!("/tmp/{}.txt", i % opt.number_of_files).as_str())?;
        let v = tokio::spawn(async move {
            let mut file = tokio::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(file.as_path())
                .await
                .expect("failed to open");
            while c.fetch_add(1, std::sync::atomic::Ordering::SeqCst) < max_count {
                file.seek(SeekFrom::Start(0)).await.expect("failed to seek");
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .await
                    .expect("failed to read");
                file.seek(SeekFrom::Start(0)).await.expect("failed to seek");
                let number = contents.parse::<u32>().expect("failed to parse");
                let number = number + 1;
                file.write_all(number.to_string().as_bytes())
                    .await
                    .expect("failed to write");
            }
        });
        tasks.push(v);
    }
    println!(
        "Duration--: {}",
        Instant::now().duration_since(start_time).as_millis()
    );
    futures::future::join_all(tasks).await;
    println!(
        "Duration: {}",
        Instant::now().duration_since(start_time).as_millis()
    );
    validate(opt.number_of_files, opt.max_count)?;
    delete_files(opt.number_of_files)?;
    Ok(())
}

pub fn execute_thread(opt: &Opt) {
    thread(&opt).expect("thread failed");
}

fn thread(opt: &Opt) -> Result<()> {
    let thread_pool = ThreadPool::new(opt.number_of_tasks);
    let start_time = Instant::now();
    create_files(opt.number_of_files)?;
    let max_count = opt.max_count;
    let number_of_files = opt.number_of_files;
    let counter: Arc<AtomicU32> = Arc::new(AtomicU32::new(0));
    for i in 0..number_of_files {
        let c = counter.clone();
        thread_pool.execute(move || {
            let file = PathBuf::from_str(format!("/tmp/{}.txt", i).as_str()).expect("path failed");
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(file)
                .expect("failed to open");
            while c.fetch_add(1, std::sync::atomic::Ordering::SeqCst) < max_count {
                file.seek(SeekFrom::Start(0)).expect("failed to seek");
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("failed to read");
                file.seek(SeekFrom::Start(0)).expect("failed to seek");
                let number = contents.parse::<u32>().expect("failed to parse");
                let number = number + 1;
                file.write_all(number.to_string().as_bytes())
                    .expect("failed to write");
            }
        });
    }

    thread_pool.join();
    println!(
        "Duration: {}",
        Instant::now().duration_since(start_time).as_millis()
    );
    validate(opt.number_of_files, opt.max_count)?;
    delete_files(opt.number_of_files)?;
    Ok(())
}

fn validate(number_of_files: u32, max_count: u32) -> Result<()> {
    let mut number = 0u32;
    for i in 0..number_of_files {
        let path = PathBuf::from_str(format!("/tmp/{}.txt", i).as_str())?;
        let mut file = File::open(path).expect("failed to open");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("failed to read");
        let number_in_file = contents.parse::<u32>().expect("failed to parse");
        number += number_in_file;
    }
    eprintln!(
        "{} | Actual: {}, Expected: {}",
        if number == max_count {
            "SUCCESS"
        } else {
            "FAILURE"
        },
        number,
        max_count,
    );
    Ok(())
}

fn create_files(number_of_files: u32) -> Result<()> {
    for i in 0..number_of_files {
        let path = PathBuf::from_str(format!("/tmp/{}.txt", i).as_str())?;
        let mut file = File::create(&path)?;
        file.write_all(b"0")?;
    }
    Ok(())
}

fn delete_files(number_of_files: u32) -> std::io::Result<()> {
    for i in 0..number_of_files {
        fs::remove_file(format!("/tmp/{}.txt", i))?;
    }
    Ok(())
}
