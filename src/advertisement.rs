use std::fmt::Debug;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::{collections::BTreeMap, error::Error, time::Duration};

#[cfg(not(feature = "disable_afit"))]
use async_trait::async_trait;
use bluer::adv::{Advertisement, AdvertisementHandle, Type};
use bluer::{Device, Address};
use futures::executor;

use crate::session::Session;
use crate::util::set_device_addr;

const APPLE_MAGIC: u16 = 0x4c;

pub trait AdvertisableData: Clone + PartialEq + Debug + Sync {
    fn octets(&self) -> Vec<u8>;
}

// If the user opted out of using "async_fn_in_trait", use the crate async-trait instead.
#[cfg_attr(not(feature = "disable_afit"), async_trait)]
/// Any kind of advertisement.
pub trait Advertisable<T: AdvertisableData> {
    /// Advertisement-specific: validate user supplied data.
    fn validate_user_data(_user_data: &T) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    /// Advertisement-specific: assemble user supplied data to advertisement.
    fn assemble_advertisement(
        session: &mut Session,
        user_data: &T,
    ) -> Result<Advertisement, Box<dyn Error>>;
    /// Register any advertisement.
    async fn register(
        session: &mut Session,
        user_data: &T,
    ) -> Result<AdvertisementHandle, Box<dyn Error>> {
        Self::validate_user_data(user_data)?;
        let advertisement = Self::assemble_advertisement(session, user_data)?;
        Ok(session.adapter.advertise(advertisement).await?)
    }
}
pub enum AdvertisementType {
    AirDrop(AirDropAdvertisementData),
    AirPlaySource, // Carries no dynamic data.
    AirPlayTarget(AirPlayTargetAdvertisementData),
    AirPrint(AirPrintAdvertisementData),
    FindMy(FindMyAdvertisementData),
}
pub fn get_adv_data_from_device(device: Device) -> Option<AdvertisementType> {
    let binding = executor::block_on(device.manufacturer_data()).ok()??;
    let manufacturer_data = binding.get(&APPLE_MAGIC)?;
    match manufacturer_data[0] {
        0x05 => Some(AdvertisementType::AirDrop(
            AirDropAdvertisementData::try_from(manufacturer_data.clone()).ok()?,
        )),
        0x0a => Some(AdvertisementType::AirPlaySource),
        0x09 => Some(AdvertisementType::AirPlayTarget(
            AirPlayTargetAdvertisementData::try_from(manufacturer_data.clone()).ok()?,
        )),
        0x03 => Some(AdvertisementType::AirPrint(
            AirPrintAdvertisementData::try_from(manufacturer_data.clone()).ok()?,
        )),
        0x12 => Some(AdvertisementType::FindMy(
            FindMyAdvertisementData::try_from((device.address(), manufacturer_data.clone())).ok()?,
        )),
        _ => None,
    }
}

/// Data for an AirDrop advertisement.
#[derive(Clone, PartialEq, Debug)]
pub struct AirDropAdvertisementData {
    pub apple_id: [u8; 2],
    pub phone: [u8; 2],
    pub email: [u8; 2],
}
impl AdvertisableData for AirDropAdvertisementData {
    fn octets(&self) -> Vec<u8> {
        [
            vec![
                0x05, // Message type
                0x12, // Message length
            ],
            vec![0; 8], // 8bytes of padding
            vec![0x01], // AirDrop version
            self.apple_id.to_vec(),
            self.phone.to_vec(),
            self.email.to_vec(),
            self.email.to_vec(),
        ]
        .concat()
    }
}
impl TryFrom<Vec<u8>> for AirDropAdvertisementData {
    type Error = Box<dyn Error>;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(AirDropAdvertisementData {
            apple_id: value[11..13].try_into()?,
            phone: value[13..15].try_into()?,
            email: value[15..17].try_into()?,
        })
    }
}

/// https://github.com/furiousMAC/continuity/blob/master/messages/airdrop.md
pub struct AirDropAdvertisement;
impl Advertisable<AirDropAdvertisementData> for AirDropAdvertisement {
    fn assemble_advertisement(
        session: &mut Session,
        user_data: &AirDropAdvertisementData,
    ) -> Result<Advertisement, Box<dyn Error>> {
        Ok(Advertisement {
            advertisement_type: Type::Broadcast,
            local_name: Some(session.adapter.name().to_string()),
            timeout: Some(Duration::from_millis(0)),
            min_interval: Some(Duration::from_millis(100)),
            max_interval: Some(Duration::from_millis(200)),
            manufacturer_data: BTreeMap::from([(APPLE_MAGIC, user_data.octets())]),
            ..Default::default()
        })
    }
}

/// Data for an AirPlay source message
#[derive(Clone, PartialEq, Debug)]
pub struct AirPlaySourceAdvertisementData;
impl AdvertisableData for AirPlaySourceAdvertisementData {
    fn octets(&self) -> Vec<u8> {
        // This is constant.
        vec![
            0x0a, // Message type
            0x01, // Message length
            0x00,
        ]
    }
}
impl TryFrom<Vec<u8>> for AirPlaySourceAdvertisementData {
    type Error = Box<dyn Error>;
    fn try_from(_value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(AirPlaySourceAdvertisementData {})
    }
}

/// AirPlay source message https://github.com/furiousMAC/continuity/blob/master/messages/airplay_source.md
pub struct AirPlaySourceAdvertisement;
impl Advertisable<AirPlaySourceAdvertisementData> for AirPlaySourceAdvertisement {
    /// The user_data field can be ignored.
    fn assemble_advertisement(
        session: &mut Session,
        user_data: &AirPlaySourceAdvertisementData,
    ) -> Result<Advertisement, Box<dyn Error>> {
        Ok(Advertisement {
            advertisement_type: Type::Broadcast,
            local_name: Some(session.adapter.name().to_string()),
            timeout: Some(Duration::from_millis(0)),
            min_interval: Some(Duration::from_millis(100)),
            max_interval: Some(Duration::from_millis(200)),
            manufacturer_data: BTreeMap::from([(APPLE_MAGIC, user_data.octets())]),
            ..Default::default()
        })
    }
}

/// Data for an AirPlay target message
#[derive(Clone, PartialEq, Debug)]
pub struct AirPlayTargetAdvertisementData {
    pub ip_address: Ipv4Addr,
}
impl AdvertisableData for AirPlayTargetAdvertisementData {
    fn octets(&self) -> Vec<u8> {
        let ip_address = self.ip_address.octets();
        [
            vec![
                0x09, // Message type
                0x06, // Message length
                0x03, 0x07,
            ],
            ip_address.to_vec(),
        ]
        .concat()
    }
}
impl TryFrom<Vec<u8>> for AirPlayTargetAdvertisementData {
    type Error = Box<dyn Error>;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let ip_address: [u8; 4] = value[4..8].try_into()?;
        Ok(AirPlayTargetAdvertisementData {
            ip_address: Ipv4Addr::from(ip_address),
        })
    }
}

/// AirPlay target message https://github.com/furiousMAC/continuity/blob/master/messages/airplay_target.md
pub struct AirPlayTargetAdvertisement;
impl Advertisable<AirPlayTargetAdvertisementData> for AirPlayTargetAdvertisement {
    fn assemble_advertisement(
        session: &mut Session,
        user_data: &AirPlayTargetAdvertisementData,
    ) -> Result<Advertisement, Box<dyn Error>> {
        Ok(Advertisement {
            advertisement_type: Type::Broadcast,
            local_name: Some(session.adapter.name().to_string()),
            timeout: Some(Duration::from_millis(0)),
            min_interval: Some(Duration::from_millis(100)),
            max_interval: Some(Duration::from_millis(200)),
            manufacturer_data: BTreeMap::from([(APPLE_MAGIC, user_data.octets())]),
            ..Default::default()
        })
    }
}

/// Data for an AirPrint message
#[derive(Clone, PartialEq, Debug)]
pub struct AirPrintAdvertisementData {
    pub port: u16,
    pub ip_addr: Ipv6Addr,
    pub power: u8,
}
impl AdvertisableData for AirPrintAdvertisementData {
    fn octets(&self) -> Vec<u8> {
        let port = self.port.to_be_bytes();
        let ip_addr = self.ip_addr.octets();
        [
            vec![
                0x03, // Message type
                0x16, // Message length
                0x74, // Address type
                0x07, // Resource path
                0x6f, // Security type
            ],
            port.to_vec(),
            ip_addr.to_vec(),
            vec![self.power],
        ]
        .concat()
    }
}
impl TryFrom<Vec<u8>> for AirPrintAdvertisementData {
    type Error = Box<dyn Error>;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let ip_address: [u8; 16] = value[7..23].try_into()?;
        Ok(AirPrintAdvertisementData {
            port: (value[5] as u16) << 8 | value[6] as u16,
            ip_addr: Ipv6Addr::from(ip_address),
            power: value[23],
        })
    }
}

/// AirPrint message https://github.com/furiousMAC/continuity/blob/master/messages/airprint.md
pub struct AirPrintAdvertisement;
impl Advertisable<AirPrintAdvertisementData> for AirPrintAdvertisement {
    fn assemble_advertisement(
        session: &mut Session,
        user_data: &AirPrintAdvertisementData,
    ) -> Result<Advertisement, Box<dyn Error>> {
        Ok(Advertisement {
            advertisement_type: Type::Broadcast,
            local_name: Some(session.adapter.name().to_string()),
            timeout: Some(Duration::from_millis(0)),
            min_interval: Some(Duration::from_millis(100)),
            max_interval: Some(Duration::from_millis(200)),
            manufacturer_data: BTreeMap::from([(APPLE_MAGIC, user_data.octets())]),
            ..Default::default()
        })
    }
}

/// Data for a FindMy message
#[derive(Clone, PartialEq, Debug)]
pub struct FindMyAdvertisementData {
    pub public_key: [u8; 28],
}
impl AdvertisableData for FindMyAdvertisementData {
    fn octets(&self) -> Vec<u8> {
        let public_key = self.public_key.split_at(6);
        [
            vec![
                0x12, // Message type
                0x19, // Message length
                0x00,
            ],
            public_key.1.to_vec(),
            vec![public_key.0[0] >> 6],
        ]
        .concat()
    }
}
impl TryFrom<(Address, Vec<u8>)> for FindMyAdvertisementData {
    type Error = Box<dyn Error>;
    fn try_from(value: (Address, Vec<u8>)) -> Result<Self, Self::Error> {
        let public_key: [u8; 28] = [&value.0.0, &value.1[2..24]]
            .concat()
            .as_slice()
            .try_into()?;
        Ok(FindMyAdvertisementData {
            public_key: public_key,
        })
    }
}

/// FindMy message https://github.com/furiousMAC/continuity/blob/master/messages/findmy.md
pub struct FindMyAdvertisement;
impl Advertisable<FindMyAdvertisementData> for FindMyAdvertisement {
    fn assemble_advertisement(
        session: &mut Session,
        user_data: &FindMyAdvertisementData,
    ) -> Result<Advertisement, Box<dyn Error>> {
        set_device_addr(session, &user_data.public_key[0..6])?;
        Ok(Advertisement {
            advertisement_type: Type::Broadcast,
            local_name: Some(session.adapter.name().to_string()),
            timeout: Some(Duration::from_millis(0)),
            min_interval: Some(Duration::from_millis(100)),
            max_interval: Some(Duration::from_millis(200)),
            manufacturer_data: BTreeMap::from([(APPLE_MAGIC, user_data.octets())]),
            ..Default::default()
        })
    }
}
