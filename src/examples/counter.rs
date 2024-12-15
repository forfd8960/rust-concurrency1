use rust_concurrency_programming::{
    concurrent_collection::thread_safe_counter,
    shared_state::{concurrent_counter, concurrent_counter1},
};

fn main() {
    let value = concurrent_counter();
    println!("value: {}", value);

    println!("concurrent counter: {}", concurrent_counter1(100));

    /*
    increase data in thread: 0
    increase data in thread: 1
    increase data in thread: 2
    increase data in thread: 3
    increase data in thread: 4
    increase data in thread: 5
    increase data in thread: 6
    increase data in thread: 7
    increase data in thread: 8
    increase data in thread: 9
    thread safe conuter: 10
    */
    println!("thread safe conuter: {}", thread_safe_counter(10));
}
