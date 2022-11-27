use std::error::Error;
use tokio::test;

#[test(flavor = "multi_thread", worker_threads = 1)]
async fn creates_session() -> Result<(), Box<dyn Error>>{
    let session = apple_ble::session::Session::new().await;
    assert!(session.is_ok());
    Ok(())
}