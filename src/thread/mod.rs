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

pub fn start_multiple_threads() {
    let handles: Vec<_> = (0..10)
        .map(|i| thread::spawn(move || println!("Hello from thread: {}", i)))
        .collect();

    handles.into_iter().for_each(|f| f.join().unwrap());
}

pub fn build_thread() {
    let builder = thread::Builder::new()
        .name("my thread".to_string())
        .stack_size(32 * 1024); // 32 bytes

    let handler = builder
        .spawn(|| {
            println!("from builder thread");
        })
        .unwrap();

    handler.join().unwrap()
}

pub fn current_thread() {
    let cur_thread = thread::current();
    println!(
        "current thread: {:?}, {:?}",
        cur_thread.id(),
        cur_thread.name()
    );

    let builder = thread::Builder::new()
        .name("new thread".to_string())
        .stack_size(32 * 1024); // 32 bytes

    let handler = builder
        .spawn(|| {
            println!("from builder thread");
            let cur_thread = thread::current();
            println!(
                "current thread: {:?}, {:?}",
                cur_thread.id(),
                cur_thread.name()
            );
        })
        .unwrap();

    handler.join().unwrap()
}
