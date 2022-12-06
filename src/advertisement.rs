use std::net::{Ipv4Addr, Ipv6Addr};
use std::{collections::BTreeMap, error::Error, time::Duration};

#[cfg(feature = "disable_afit")]
use async_trait::async_trait;
use bluer::adv::{Advertisement, AdvertisementHandle, Type};

use crate::session::Session;
use crate::util::{get_first_two_bytes_of_sha256, set_device_addr};

const APPLE_MAGIC: u16 = 0x4C00_u16;

pub trait AdvertisableData: Clone {
    fn octets(&self) -> Vec<u8>;
}

// If the user opted out of using "async_fn_in_trait", use the crate async-trait instead.
#[cfg_attr(feature = "disable_afit", async_trait)]
/// Any kind of advertisement.
pub trait Advertisable<T: AdvertisableData> {
    /// Advertisement-specific: validate user supplied data.
    fn validate_user_data(_user_data: &T) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    /// Advertisement-specific: assemble user supplied data to advertisement.
    fn assemble_advertisement(
        session: &Session,
        user_data: &T,
    ) -> Result<Advertisement, Box<dyn Error>>;
    /// Register any advertisement.
    async fn register(
        session: &Session,
        user_data: &T,
    ) -> Result<AdvertisementHandle, Box<dyn Error>> {
        Self::validate_user_data(user_data)?;

        Ok(session
            .adapter
            .advertise(Self::assemble_advertisement(session, user_data)?)
            .await?)
    }
}


/// Data for an AirDrop advertisement.
#[derive(Clone)]
pub struct AirDropAdvertisementData {
    pub apple_id: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>
}
impl AdvertisableData for AirDropAdvertisementData {
    fn octets(&self) -> Vec<u8> {
        let apple_id = get_first_two_bytes_of_sha256(self.apple_id.clone().unwrap_or_default());
        let phone = get_first_two_bytes_of_sha256(self.phone.clone().unwrap_or_default());
        let email = get_first_two_bytes_of_sha256(self.email.clone().unwrap_or_default());
        [   
            
            vec![
                0x05,   // Message type
                0x12    // Message length
            ],
            vec![0;8],  // 8bytes of padding
            vec![0x01], // AirDrop version
            apple_id.to_vec(),
            phone.to_vec(),
            email.to_vec(),
            email.to_vec()
        ].concat()
    }
}

/// https://github.com/furiousMAC/continuity/blob/master/messages/airdrop.md
pub struct AirDropAdvertisement;
impl Advertisable<AirDropAdvertisementData> for AirDropAdvertisement {
    fn assemble_advertisement(
        session: &Session,
        user_data: &AirDropAdvertisementData,
    ) -> Result<Advertisement, Box<dyn Error>> {
        Ok(Advertisement {
            advertisement_type: Type::Broadcast,
            local_name: Some(session.adapter.name().to_string()),
            timeout: Some(Duration::from_millis(0)),
            min_interval: Some(Duration::from_millis(100)),
            max_interval: Some(Duration::from_millis(200)),
            manufacturer_data: BTreeMap::from([(
                APPLE_MAGIC,
                user_data.octets(),
            )]),
            ..Default::default()
        })
    }
}


/// Data for an AirPlay source message
#[derive(Clone)]
pub struct AirPlaySourceAdvertisementData;
impl AdvertisableData for AirPlaySourceAdvertisementData {
    fn octets(&self) -> Vec<u8> {
        // This is constant.
        vec![
            0x0a,   // Message type
            0x01,   // Message length
            0x00
        ]
    }
}

/// AirPlay source message https://github.com/furiousMAC/continuity/blob/master/messages/airplay_source.md
pub struct AirPlaySourceAdvertisement;
impl Advertisable<AirPlaySourceAdvertisementData> for AirPlaySourceAdvertisement {
    /// The user_data field can be ignored.
    fn assemble_advertisement(
        session: &Session,
        user_data: &AirPlaySourceAdvertisementData,
    ) -> Result<Advertisement, Box<dyn Error>> {
        Ok(Advertisement{
            advertisement_type: Type::Broadcast,
            local_name: Some(session.adapter.name().to_string()),
            timeout: Some(Duration::from_millis(0)),
            min_interval: Some(Duration::from_millis(100)),
            max_interval: Some(Duration::from_millis(200)),
            manufacturer_data: BTreeMap::from([(
                APPLE_MAGIC,
                user_data.octets()
            )]),
            ..Default::default()
        })
    }
}


/// Data for an AirPlay target message
#[derive(Clone)]
pub struct AirPlayTargetAdvertisementData {
    pub flags: Option<u8>,
    pub seed: Option<u8>,
    pub ip_address: Ipv4Addr
}
impl AdvertisableData for AirPlayTargetAdvertisementData {
    fn octets(&self) -> Vec<u8> {
        let flags = self.flags.unwrap_or(0x03);
        let seed = self.seed.unwrap_or(0x07);
        let ip_address = self.ip_address.octets();
        [
            vec![
                0x09,   // Message type
                0x06,   // Message length
                flags,
                seed
            ],
            ip_address.to_vec()
        ].concat()
    }
}

/// AirPlay target message https://github.com/furiousMAC/continuity/blob/master/messages/airplay_target.md
pub struct AirPlayTargetAdvertisement;
impl Advertisable<AirPlayTargetAdvertisementData> for AirPlayTargetAdvertisement {
    fn assemble_advertisement(
        session: &Session,
        user_data: &AirPlayTargetAdvertisementData,
    ) -> Result<Advertisement, Box<dyn Error>> {
        Ok(Advertisement{
            advertisement_type: Type::Broadcast,
            local_name: Some(session.adapter.name().to_string()),
            timeout: Some(Duration::from_millis(0)),
            min_interval: Some(Duration::from_millis(100)),
            max_interval: Some(Duration::from_millis(200)),
            manufacturer_data: BTreeMap::from([(
                APPLE_MAGIC,
                user_data.octets()
            )]),
            ..Default::default()
        })
    }
}


/// Data for an AirPrint message
#[derive(Clone)]
pub struct AirPrintAdvertisementData {
    pub port: u16,
    pub ip_addr: Ipv6Addr,
    pub power: u8
}
impl AdvertisableData for AirPrintAdvertisementData {
    fn octets(&self) -> Vec<u8> {
        let port = self.port.to_be_bytes();
        let ip_addr = self.ip_addr.octets();
        [
            vec![
                0x03,   // Message type
                0x16,   // Message length
                0x74,   // Address type
                0x07,   // Resource path
                0x6f    // Security type
            ],
            port.to_vec(),
            ip_addr.to_vec(),
            vec![
                self.power
            ]
        ].concat()
    }
}

/// AirPrint message https://github.com/furiousMAC/continuity/blob/master/messages/airprint.md
pub struct AirPrintAdvertisement;
impl Advertisable<AirPrintAdvertisementData> for AirPrintAdvertisement {
    fn assemble_advertisement(
        session: &Session,
        user_data: &AirPrintAdvertisementData,
    ) -> Result<Advertisement, Box<dyn Error>> {
        Ok(Advertisement{
            advertisement_type: Type::Broadcast,
            local_name: Some(session.adapter.name().to_string()),
            timeout: Some(Duration::from_millis(0)),
            min_interval: Some(Duration::from_millis(100)),
            max_interval: Some(Duration::from_millis(200)),
            manufacturer_data: BTreeMap::from([(
                APPLE_MAGIC,
                user_data.octets()
            )]),
            ..Default::default()
        })
    }
}


/// Data for a FindMy message
#[derive(Clone)]
pub struct FindMyAdvertisementData {
    pub public_key: [u8; 28]
}
impl AdvertisableData for FindMyAdvertisementData {
    fn octets(&self) -> Vec<u8> {
        let public_key = self.public_key.split_at(6);
        [
            vec![
                0x12,   // Message length
                0x19,   // Message type
                0x00
            ],
            public_key.1.to_vec(),
            vec![
                public_key.0[0] >> 6,
                0x00
            ]
        ].concat()
    }
}

/// FindMy message https://github.com/furiousMAC/continuity/blob/master/messages/findmy.md
pub struct FindMyAdvertisement;
impl Advertisable<FindMyAdvertisementData> for FindMyAdvertisement {
    fn assemble_advertisement(
        session: &Session,
        user_data: &FindMyAdvertisementData,
    ) -> Result<Advertisement, Box<dyn Error>> {
        set_device_addr(session, &user_data.public_key[0..6])?;
        Ok(Advertisement{
            advertisement_type: Type::Broadcast,
            local_name: Some(session.adapter.name().to_string()),
            timeout: Some(Duration::from_millis(0)),
            min_interval: Some(Duration::from_millis(100)),
            max_interval: Some(Duration::from_millis(200)),
            manufacturer_data: BTreeMap::from([(
                APPLE_MAGIC,
                user_data.octets()
            )]),
            ..Default::default()
        })
    }
}