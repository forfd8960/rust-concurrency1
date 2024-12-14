use std::{
    sync::{Arc, Mutex, RwLock},
    thread,
};

#[derive(Debug, Clone)]
pub struct Counter {
    pub count: i64,
}

impl Counter {
    pub fn new(init: i64) -> Self {
        Self { count: init }
    }
}

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

pub fn concurrent_counter1(limit: i64) -> i64 {
    let counter = Arc::new(Mutex::new(Counter::new(0)));

    println!(
        "counter before concurrent access: {:?}",
        counter.lock().unwrap()
    );

    let handles: Vec<_> = (0..limit)
        .map(|_| {
            let inner = counter.clone();
            thread::spawn(move || {
                let mut counter = inner.lock().unwrap();
                counter.count += 1;
            })
        })
        .collect();

    handles.into_iter().for_each(|h| h.join().unwrap());
    let x = counter.lock().unwrap();
    println!("counter after concurrent access: {:?}", x);
    x.count
}

pub fn concurrent_read_write() -> i64 {
    let rw_counter = Arc::new(RwLock::new(Counter::new(0)));

    println!(
        "rw_counter before concurrent access: {:?}",
        rw_counter.read().unwrap()
    );

    let read_handles: Vec<_> = (0..10)
        .map(|_| {
            let inner = rw_counter.clone();
            thread::spawn(move || {
                let counter = inner.read().unwrap();
                println!("read counter: {:?}", counter);
            })
        })
        .collect();

    let write_handles: Vec<_> = (0..10)
        .map(|_| {
            let inner = rw_counter.clone();
            thread::spawn(move || {
                let mut counter = inner.write().unwrap();
                println!("write counter: {:?}", counter);
                counter.count += 1;
            })
        })
        .collect();

    read_handles.into_iter().for_each(|h| h.join().unwrap());
    write_handles.into_iter().for_each(|h| h.join().unwrap());

    let x = rw_counter.read().unwrap();
    println!("rw_counter after concurrent access: {:?}", x);
    x.count
}
