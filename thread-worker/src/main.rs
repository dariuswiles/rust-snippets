//! Multi-threading demo
//!
//! One worker thread is created per physical CPU, and each pulls incoming work from a shared
//! channel, "works" on it and returns results in a separate channel. The work consists of
//! returning a string that includes data sent in the job.
//!
//! The code could be improved by:
//! 1. moving the higher level code out of `main()`, probably into a thread pool object that is
//!    responsible for setting up and managing worker threads; and
//! 2. making the worker more generic so it can perform a *_given_* task passed in as a closure,
//!    rather than having it hard-coded into the worker code.
//!
//! The Rust book has example code that does exactly this:
//! https://doc.rust-lang.org/book/ch20-03-graceful-shutdown-and-cleanup.html

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

const JOB_COUNT: usize = 20;

#[derive(Clone, Debug)]
struct Job {
    data: String,
}

struct Worker {}

impl Worker {
    pub fn new(
        id: usize,
        job_assignment_rx: Arc<Mutex<mpsc::Receiver<Option<Job>>>>,
        result_tx: mpsc::Sender<String>,
    ) -> Self {
        println!("Creating new worker");

        let _ = Some(thread::spawn(move || {
            println!("Worker {} starting", id);

            loop {
                let job = job_assignment_rx.lock().unwrap().recv().unwrap();

                match job {
                    Some(j) => {
                        println!("Worker {} received new job containing data: '{}'", id, j.data);
                        thread::sleep(Duration::from_millis(100));
                        result_tx
                            .send(format!("Worker {} completed job with data '{}'", id, j.data))
                            .unwrap_or_else(|_| panic!("Worker {} unable to send results", id));
                    }
                    None => {
                        break;
                    }
                }
            }
        }));

        Self {}
    }
}

fn main() {
    // Benchmark begin
    let benchmark_start = SystemTime::now();

    let thread_count = num_cpus::get_physical();
    println!("Found {} physical cores", thread_count);

    let mut workers = Vec::new();
    let (work_assignment_tx, work_assignment_rx) = mpsc::channel();
    let (results_channel_tx, results_channel_rx) = mpsc::channel();

    let arc_work_assignment_rx = Arc::new(Mutex::new(work_assignment_rx));

    // Create one worker for each physical CPU core
    for i in 0..thread_count {
        workers.push(Worker::new(
            i,
            arc_work_assignment_rx.clone(),
            results_channel_tx.clone(),
        ));
    }

    // Send all jobs via a channel. Each job will be retrieved by the next free worker.
    for i in 0..JOB_COUNT {
        let new_job = Job {
            data: format!("Job #{} data", i),
        };
        work_assignment_tx
            .send(Some(new_job))
            .expect("Failed to send");
    }

    // Collect the results from all the jobs. Although this code sends and retrieves jobs in
    // separate loops, this can be interleaved.
    for _ in 0..JOB_COUNT {
        let r = results_channel_rx.recv().expect("Failed to receive");
        println!("Received completed job data: {}", r);
    }

    for _ in workers {
        println!("Terminating a worker");
        work_assignment_tx.send(None).expect("Failed to send");
    }

    // Benchmark end
    match benchmark_start.elapsed() {
        Ok(elapsed) => {
            println!(
                "Benchmark: run time was {} milliseconds",
                elapsed.as_millis()
            );
        }
        Err(e) => {
            println!(
                "Benchmark: run time cannot be calculated due to error: {:?}",
                e
            );
        }
    }
}
