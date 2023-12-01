//! ## rpools
//!
//! This module contains constructs for dealing with concurrent tasks. It can spawn
//! any number of worker threads and sync them with other channels.
//!
//! ## Examples
//!
//! ### Synchronized with other channels
//!
//! ```
//! use rpools::pool::WorkerPool;
//! use std::sync::mpsc::channel;
//! use std::sync::{Arc, Mutex};
//!
//! let n_workers = 4;
//! let n_jobs = 8;
//! let pool = WorkerPool::new(n_workers);
//!
//! let (tx, rx) = channel();
//! let atx = Arc::new(Mutex::new(tx));
//! for _ in 0..n_jobs {
//!     let atx = atx.clone();
//!     pool.execute(move|| {
//!         let tx = atx.lock().unwrap();
//!         tx.send(1).expect("channel will be there waiting for the pool");
//!     });
//! }
//!
//! assert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), 8);
//!```

// Imports and makes pool public.
pub mod pool;
