use rust_concurrency_programming::shared_state::{concurrent_counter, concurrent_counter1};

fn main() {
    let value = concurrent_counter();
    println!("value: {}", value);

    println!("concurrent counter: {}", concurrent_counter1(100))
}
