use rust_concurrency_programming::asyncmod::wait_async;

// `async fn` define an async function, means there are async code in the function.
#[tokio::main]
async fn main() {
    match wait_async().await {
        Ok(_) => println!("success"),
        Err(e) => println!("failed: {}", e),
    }
}
