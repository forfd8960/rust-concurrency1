use rust_concurrency_programming::thread::{
    build_thread, current_thread, start_2_thread, start_multiple_threads, start_thread,
    start_thread_with_value,
};

fn main() {
    println!("Rust Concurrency");
    start_thread();
    start_thread_with_value();
    start_2_thread();

    start_multiple_threads();
    build_thread();
    current_thread();
}
