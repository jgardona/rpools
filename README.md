# rpools

<p align="center">
  <a href="https://crates.io/crates/rpools">
    <img src="https://img.shields.io/crates/v/rpools.svg" alt="Crates.io"/>
  </a>
  <a href="https://crates.io/crates/rpools">
    <img src="https://img.shields.io/crates/d/rpools" alt="Crates.io Downloads"/>
  </a>
  <img src="https://img.shields.io/badge/rust-stable-orange" alt="Rust Stable"/>
  <a href="https://opensource.org/licenses/MIT">
    <img src="https://img.shields.io/crates/l/rpools.svg" alt="License"/>
  </a>
  <a href="https://github.com/jgardona/rpools/actions/workflows/rust.yml">
    <img src="https://github.com/jgardona/rpools/actions/workflows/rust.yml/badge.svg" alt="GitHub Actions Workflow Status"/>
  </a>
</p>


A minimalist rust workerpool implementation that uses channels to synchronize the jobs. It can spawn a fixed number of worker threads, that waits for a job queue.


* Use
```rust
 use rpools::pool::WorkerPool;
 use std::sync::mpsc::channel;
 use std::sync::{Arc, Mutex};

 let n_workers = 4;
 let n_jobs = 8;
 let pool = WorkerPool::new(n_workers);

 let (tx, rx) = channel();
 let atx = Arc::new(Mutex::new(tx));
 for _ in 0..n_jobs {
     let atx = atx.clone();
     pool.execute(move|| {
         let tx = atx.lock().unwrap();

            // a long task goes here
            // send results to channel (use it to sync the pool with the parent thread)

         tx.send(1).expect("channel will be there waiting for the pool");
     });
 }

 assert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), 8);
```

* Test

```shell
$ cargo test
```
