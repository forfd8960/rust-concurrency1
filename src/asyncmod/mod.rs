pub async fn long_run() -> Result<(), String> {
    println!("running in async long_run");
    Ok(())
}

pub async fn long_run1() -> Result<(), String> {
    println!("running in async long_run1");
    Ok(())
}

pub async fn long_run2() -> Result<(), String> {
    println!("running in async long_run2");
    Ok(())
}

pub async fn wait_async() -> Result<(), String> {
    let block = async {
        let _ = long_run().await?;
        let _ = long_run1().await;
        let _ = long_run2().await;
        Ok::<(), String>(())
    };

    // await used in async function to wait the async block or the async function done.
    block.await?;

    println!("all async fn done");
    Ok(())
}
