use apple_ble::advertisement::{Advertisable, AirDropAdvertisementData, AdvertisableData, AirPlayTargetAdvertisementData, AirPrintAdvertisementData, FindMyAdvertisementData};
use bluer::Address;
use std::{error::Error, net::{Ipv4Addr, Ipv6Addr}};
use tokio::test;

#[test(flavor = "multi_thread", worker_threads = 1)]
async fn test_airdrop_advertisement() -> Result<(), Box<dyn Error>> {
    let mut session = apple_ble::session::Session::new().await?;
    apple_ble::advertisement::AirDropAdvertisement::register(
        &mut session,
        &apple_ble::advertisement::AirDropAdvertisementData{
            apple_id: [0x00, 0x00],
            phone: [0x00, 0x00],
            email : [0x00, 0x00]
        },
    )
    .await?;
    Ok(())
}

#[test(flavor = "multi_thread", worker_threads = 1)]
async fn test_airplaysource_advertisement() -> Result<(), Box<dyn Error>> {
    let mut session = apple_ble::session::Session::new().await?;
    apple_ble::advertisement::AirPlaySourceAdvertisement::register(
        &mut session,
        &apple_ble::advertisement::AirPlaySourceAdvertisementData {},
    )
    .await?;
    Ok(())
}

#[test(flavor = "multi_thread", worker_threads = 1)]
async fn test_airplaytarget_advertisement() -> Result<(), Box<dyn Error>> {
    let mut session = apple_ble::session::Session::new().await?;
    apple_ble::advertisement::AirPlayTargetAdvertisement::register(
        &mut session,
        &apple_ble::advertisement::AirPlayTargetAdvertisementData {
            ip_address: Ipv4Addr::LOCALHOST
        },
    )
    .await?;
    Ok(())
}

#[test(flavor = "multi_thread", worker_threads = 1)]
async fn test_airprint_advertisement() -> Result<(), Box<dyn Error>> {
    let mut session = apple_ble::session::Session::new().await?;
    apple_ble::advertisement::AirPrintAdvertisement::register(
        &mut session,
        &apple_ble::advertisement::AirPrintAdvertisementData {
            port: 0x1337,
            ip_addr: Ipv6Addr::LOCALHOST,
            power: 100
        },
    )
    .await?;
    Ok(())
}

#[test(flavor = "multi_thread", worker_threads = 1)]
async fn test_findmy_advertisement() -> Result<(), Box<dyn Error>> {
    let mut session = apple_ble::session::Session::new().await?;
    apple_ble::advertisement::FindMyAdvertisement::register(
        &mut session,
        &apple_ble::advertisement::FindMyAdvertisementData {
            public_key: [0x88; 28]
        },
    )
    .await?;
    Ok(())
}

#[test(flavor = "multi_thread", worker_threads = 1)]
async fn test_serialization_and_deserialization() -> Result<(), Box<dyn Error>> {
    let data = AirDropAdvertisementData {
        apple_id: [0xfe, 0xdc],
        email: [0xba, 0x98],
        phone: [0x76, 0x54]
    };
    let serialized = data.clone().octets();
    let deserialized = AirDropAdvertisementData::try_from(serialized)?;
    assert_eq!(data, deserialized);

    let data = AirPlayTargetAdvertisementData {
        ip_address: Ipv4Addr::LOCALHOST
    };
    let serialized = data.clone().octets();
    let deserialized = AirPlayTargetAdvertisementData::try_from(serialized)?;
    assert_eq!(data, deserialized);

    let data = AirPrintAdvertisementData {
        port: 0xf00d,
        ip_addr: Ipv6Addr::LOCALHOST,
        power: 0xff
    };
    let serialized = data.clone().octets();
    let deserialized = AirPrintAdvertisementData::try_from(serialized)?;
    assert_eq!(data, deserialized);

    let data = FindMyAdvertisementData {
        public_key: [0x00_u8; 28]
    };
    let serialized = data.clone().octets();
    let deserialized = FindMyAdvertisementData::try_from((Address::new(data.public_key[0..6].try_into()?), serialized))?;
    assert_eq!(data, deserialized);
    Ok(())
}