use apple_ble::advertisement::Advertisable;
use std::{error::Error, net::{Ipv4Addr, Ipv6Addr}};
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

#[test(flavor = "multi_thread", worker_threads = 1)]
async fn test_airplaytarget_advertisement() -> Result<(), Box<dyn Error>> {
    let session = apple_ble::session::Session::new().await?;
    apple_ble::advertisement::AirPlayTargetAdvertisement::register(
        &session,
        &apple_ble::advertisement::AirPlayTargetAdvertisementData {
            flags: Some(0x0),
            seed: Some(0x42),
            ip_address: Ipv4Addr::from([133, 71, 33, 7])
        },
    )
    .await?;
    Ok(())
}

#[test(flavor = "multi_thread", worker_threads = 1)]
async fn test_airprint_advertisement() -> Result<(), Box<dyn Error>> {
    let session = apple_ble::session::Session::new().await?;
    apple_ble::advertisement::AirPrintAdvertisement::register(
        &session,
        &apple_ble::advertisement::AirPrintAdvertisementData {
            port: 0x1337,
            ip_addr: Ipv6Addr::LOCALHOST,
            power: 100
        },
    )
    .await?;
    Ok(())
}