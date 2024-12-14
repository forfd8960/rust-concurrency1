use std::{
    sync::{Arc, Mutex},
    thread,
};

// Mutex is an abbreviation for mutual exclusion, as in a mutex allows only one thread to access some data at any given time
// Arc: Atomic Reference counter, can be used multiple threads.
pub fn concurrent_counter() -> i32 {
    let counter = Arc::new(Mutex::new(0));

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let inner = counter.clone();
            thread::spawn(move || {
                let mut num = inner.lock().unwrap();
                *num += 1;
            })
        })
        .collect();

    handles.into_iter().for_each(|h| h.join().unwrap());
    let x = *counter.lock().unwrap();
    x
}
