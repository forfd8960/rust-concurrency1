use rust_concurrency_programming::shared_state::concurrent_counter;

fn main() {
    let value = concurrent_counter();
    println!("value: {}", value);
}
