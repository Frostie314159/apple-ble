use apple_ble::advertisement::Advertisable;
use std::error::Error;
use tokio::test;

#[test(flavor = "multi_thread", worker_threads = 1)]
async fn test_airdrop_advertisement() -> Result<(), Box<dyn Error>> {
    let session = apple_ble::session::Session::new().await?;
    apple_ble::advertisement::AirDropAdvertisement::register(
        &session,
        &apple_ble::advertisement::AirDropAdvertisementData{
            apple_id: None,
            phone: None,
            email : None
        },
    )
    .await?;
    Ok(())
}

#[test(flavor = "multi_thread", worker_threads = 1)]
async fn test_airplaysource_advertisement() -> Result<(), Box<dyn Error>> {
    let session = apple_ble::session::Session::new().await?;
    apple_ble::advertisement::AirPlaySourceAdvertisement::register(
        &session,
        &apple_ble::advertisement::AirPlaySourceAdvertisementData {},
    )
    .await?;
    Ok(())
}
