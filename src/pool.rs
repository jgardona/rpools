//! ## Pool
//!
//! With this module, we are able to synchronize channels,
//! start jobs, wait for workers, and many others concurrent
//! tasks are made easy.

use std::{
    fmt::Display,
    sync::{mpsc, Arc, Mutex},
    thread,
};

// Basic types for concurrent tasks
type Job = Box<dyn FnOnce() + Send + Sync + 'static>;
type JobReceiver = Arc<Mutex<mpsc::Receiver<Job>>>;
type Handle = thread::JoinHandle<()>;

/// Implements a continuous pool of rust threads thats doesn't stops
/// unless it gets out of scope.
///
/// ### Examples
/// 
/// let njobs = 20;
/// let nworkers = 3;
/// let pool = pool::WorkerPool::new(nworkers);
/// let atomic = Arc::new(AtomicUsize::new(0));
/// let wg = WaitGroup::default();
/// 
/// // send the jobs to the pool
/// for _ in 0..njobs {
///     let wg = wg.clone();
///     let atomic = atomic.clone();
///     pool.execute(move || {
///         atomic.fetch_add(1, Ordering::Relaxed);
///         drop(wg);
///     });
/// }
/// 
/// // wait for the pool finnishes
/// wg.wait();
/// assert_eq!(njobs, atomic.load(Ordering::Relaxed));
pub struct WorkerPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl WorkerPool {
    /// Constructs a new WorkerPool of size x.
    ///
    /// **size**: usize - Is the number of workers in WorkerPool object. \
    /// **returns**: a WorkerPool object.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpools::pool::WorkerPool;
    ///
    /// let pool = WorkerPool::new(3);
    ///
    /// assert_eq!("workers[] = (id: 0)(id: 1)(id: 2)", pool.to_string());
    /// ```
    pub fn new(size: usize) -> WorkerPool {
        let (tx, rx) = mpsc::channel();
        let mut workers = Vec::<Worker>::with_capacity(size);
        let rec = Arc::new(Mutex::new(rx));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&rec)));
        }

        WorkerPool {
            workers,
            sender: tx,
        }
    }

    /// Executes a job. The job is moved to closure, as this function is FnOnce. \
    ///
    /// **f**: A FnOnce closure hosted by a Box smart pointer.
    /// ## Examples
    ///
    /// ```
    /// use rpools::pool::WorkerPool;
    /// use std::sync::mpsc;
    /// use std::sync::{Arc, Mutex};
    ///
    /// let njobs = 20;
    /// let nworkers = 10;
    ///
    /// let pool = WorkerPool::new(nworkers);
    /// let (tx, rx) = mpsc::channel();
    ///
    /// let atx = Arc::new(Mutex::new(tx));
    ///
    /// for _ in 0 .. njobs {
    ///     let atx = atx.clone();
    ///     pool.execute(move || {
    ///         let tx = atx.lock().unwrap();
    ///         tx.send(1).unwrap();
    ///     });
    /// }
    ///
    /// let sum = rx.iter().take(njobs).sum();
    /// assert_eq!(njobs, sum);
    /// ```
    pub fn execute<J>(&self, f: J)
    where
        J: FnOnce() + Send + Sync + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).expect("Cant send job");
    }
}

// Implements Display for WorkerPool. This is usefull as we can able
// to compare and make unit tests more easily.
impl Display for WorkerPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        for i in &self.workers {
            buffer.push_str(&i.to_string());
        }
        write!(f, "workers[] = {}", buffer)
    }
}

// A structure that holds an id and thread handle.
//
// id: usize - An id for worker indentification.\
// handle: JoinHandle<()> - a handle that has a working thread.
struct Worker {
    id: usize,
    _handle: Handle,
}

impl Worker {
    // Constructs a new Worker.
    //
    // id: usize - Worker identificator.
    // handle: JoinHandle<()> - a thread handle.
    fn new(id: usize, handle: JobReceiver) -> Worker {
        let handle = thread::spawn(move || loop {
            let job = match handle.lock().expect("Cant acquire lock").recv() {
                Ok(data) => data,
                Err(_) => continue,
            };

            job();
        });

        Worker {
            id,
            _handle: handle,
        }
    }
}

// Implements Display for Worker as this simplifys test writing.
impl Display for Worker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(id: {})", self.id,)
    }
}

// This sections are the beginning of workerpool module unit tests.
#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn worker_should_return_new() {
        let (_, rx) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(rx));
        let w = Worker::new(1, Arc::clone(&receiver));
        assert_eq!("(id: 1)", w.to_string());
    }

    #[test]
    fn workerpool_should_return_new() {
        let expected = "workers[] = (id: 0)(id: 1)(id: 2)".to_string();
        let pool = WorkerPool::new(3);
        assert_eq!(expected.to_string(), pool.to_string());
    }

    #[test]
    fn workerpool_should_execute_job_succeed() {
        let pool = WorkerPool::new(1);
        for _ in 0..10000 {
            pool.execute(|| {
                let _sum = 3 + 1;
            });
        }
    }
}
