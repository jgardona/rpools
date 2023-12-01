use std::{sync::mpsc, sync::Arc, sync::Mutex};

use rpools::pool;

#[test]
fn pool_should_synchronize_sender_and_receiver_and_fold_results() {
    let nworkers = 4;
    let njobs = 8;

    let pool = pool::WorkerPool::new(nworkers);

    let (tx, rx) = mpsc::channel();
    let atx = Arc::new(Mutex::new(tx));
    for _ in 0..njobs {
        let atx = atx.clone();
        pool.execute(move || {
            let tx = atx.lock().unwrap();

            // a long task goes here
            // send results to channel (use it to sync the pool with the parent thread)

            tx.send(1).expect("channel waiting for pool");
        });
    }

    assert_eq!(rx.iter().take(njobs).fold(0, |a, b| a + b), njobs);
}
