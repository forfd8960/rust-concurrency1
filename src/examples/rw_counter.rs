use rust_concurrency_programming::shared_state::concurrent_read_write;

fn main() {
    println!("read write counter: {}", concurrent_read_write());
}
