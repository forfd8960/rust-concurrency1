use std::thread;

pub fn start_thread() {
    // start a new thread in current thread
    let handle = thread::spawn(|| {
        println!("Hello from Thread");
    });

    // wait until the new thread to finish
    handle.join().unwrap()
}

pub fn start_thread_with_value() {
    // start a new thread in current thread
    let handle = thread::spawn(|| {
        println!("Hello from Thread");
        200
    });

    // wait until the new thread to finish
    match handle.join() {
        Ok(v) => println!("thread value: {}", v),
        Err(_) => println!("thread panic"),
    }
}

pub fn start_2_thread() {
    let h1 = thread::spawn(|| {
        println!("working on thread1");
    });

    let h2 = thread::spawn(|| {
        println!("working on thread2");
    });

    h1.join().unwrap();
    h2.join().unwrap()
}
