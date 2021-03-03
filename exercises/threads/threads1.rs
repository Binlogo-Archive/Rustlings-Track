// threads1.rs
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a hint.
// This program should wait until all the spawned threads have finished before exiting.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;


fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            status_shared.lock().unwrap().jobs_completed += 1;
        }
    });
    while status.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }

    let mut completed_threads = 0;
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        completed_threads += 1;
    }

    if completed_threads != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }
    
}
