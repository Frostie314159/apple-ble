use apple_ble::advertisements::Advertisable;
use std::error::Error;
use tokio::test;

#[test(flavor = "multi_thread", worker_threads = 1)]
async fn register_airdrop_advertisement() -> Result<(), Box<dyn Error>> {
    let session = apple_ble::session::Session::new().await;
    apple_ble::advertisements::AirDropAdvertisement::register(
        &session.unwrap(),
        &apple_ble::advertisements::AirDropAdvertisementData::new(
            None, None, None,
        ),
    )
    .await?;
    Ok(())
}