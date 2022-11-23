use std::error::Error;

use apple_ble::advertisements::{Advertisable, AirDropAdvertisementData};
use tokio::test;

#[test(flavor = "multi_thread", worker_threads = 1)]
async fn creates_session() -> Result<(), Box<dyn Error>>{
    let session = apple_ble::session::Session::new().await;
    assert!(session.is_ok());
    apple_ble::advertisements::AirDropAdvertisement::register(&session.unwrap(), &Some(AirDropAdvertisementData{
        apple_id: Some("john.doe@example.com".to_string()),
        phone: Some("+15552368".to_string()),
        email: None,
        email2: None
    })).await?;
    Ok(())
}