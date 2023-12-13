//! ## Sync
//!
//! This module has data structures used to synchronize
//! threads. WaitGroup is used to make a thread to wait
//! others.
//!
//! ### Examples
//! ```
//! use rpools::pool::WorkerPool;
//! use std::sync::{atomic::AtomicUsize, atomic::Ordering, Arc, Mutex};
//! use rpools::sync::WaitGroup;
//!
//! let njobs = 20;
//! let nworkers = 3;
//! let pool = WorkerPool::new(nworkers);
//! let atomic = Arc::new(AtomicUsize::new(0));
//! let wg = WaitGroup::default();
//!
//! // send the jobs to the pool
//! for _ in 0..njobs {
//!     let wg = wg.clone();
//!     let atomic = atomic.clone();
//!     pool.execute(move || {
//!         atomic.fetch_add(1, Ordering::Relaxed);
//!         drop(wg);
//!     });
//! }
//!
//! // wait for the pool finnishes
//! wg.wait();
//! assert_eq!(njobs, atomic.load(Ordering::Relaxed));
//! ```

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Condvar, Mutex,
};

/// A data struct to store a counter, a mutex and a condvar.
/// It is responsible and serves as semaphore to synchronize threads.
#[derive(Default)]
struct Wg {
    counter: AtomicUsize,
    mu: Mutex<bool>,
    condvar: Condvar,
}

/// A public wrapper above Wg. This data structure is responsible
/// to do the logics of the semaphore, block the target thread and
/// wait for signals to continue processing.
#[derive(Default)]
pub struct WaitGroup(Arc<Wg>);

impl WaitGroup {
    /// Blocks the current thread and waits until counter becomes 0. If
    /// counter is 0, start processing again.
    pub fn wait(&self) {
        let mut mutex = self.0.mu.lock().expect("Cant get the lock");
        loop {
            if self.0.counter.load(Ordering::Relaxed) == 0 {
                break;
            }
            mutex = self
                .0
                .condvar
                .wait(mutex)
                .expect("Cant block the current thread");
        }
    }
}

/// Implements Clone for WaitGroup
impl Clone for WaitGroup {
    /// For each clone of this struct, increments the
    /// counter in one.
    fn clone(&self) -> Self {
        self.0.counter.fetch_add(1, Ordering::Relaxed);
        Self(self.0.clone())
    }
}

/// Implements Drop for WaitGroup
impl Drop for WaitGroup {
    /// When a shared reference goes out of scope,
    /// decrement the counter in one.
    fn drop(&mut self) {
        self.0.counter.fetch_sub(1, Ordering::Relaxed);
        self.0.condvar.notify_one();
    }
}

#[cfg(test)]
mod mod_wait_group_tests {
    use super::WaitGroup;

    #[test]
    fn test_if_zero_count_must_not_block() {
        let wg = WaitGroup::default();
        wg.wait();
    }
}
