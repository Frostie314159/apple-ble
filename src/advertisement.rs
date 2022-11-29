use std::net::Ipv4Addr;
use std::{collections::BTreeMap, error::Error, time::Duration};

#[cfg(feature = "disable_afit")]
use async_trait::async_trait;
use bluer::adv::{Advertisement, AdvertisementHandle, Type};

use crate::session::Session;
use crate::util::get_first_two_bytes_of_sha256;

const APPLE_MAGIC: u16 = 0x4C_u16;

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
            // Message type
            0x05, 
            // Message length
            0x12,
            // 8bytes of padding
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            // AirDrop version
            0x01,
            apple_id[0],
            apple_id[1],
            phone[0],
            phone[1],
            email[0],
            email[1],
            email[0],
            email[1]
        ].to_vec()
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
        [
            0x0a,
            0x01,
            0x00
        ].to_vec()
    }
}

/// AirPlay source message https://github.com/furiousMAC/continuity/blob/master/messages/airplay_source.md
pub struct AirPlaySourceAdvertisement;
impl Advertisable<AirPlaySourceAdvertisementData> for AirPlaySourceAdvertisement {
    /// The user_data field can be ignored.
    fn assemble_advertisement(
        session: &Session,
        _user_data: &AirPlaySourceAdvertisementData,
    ) -> Result<Advertisement, Box<dyn Error>> {
        Ok(Advertisement{
            advertisement_type: Type::Broadcast,
            local_name: Some(session.adapter.name().to_string()),
            ..Default::default()
        })
    }
}


/// Data for an AirPlay target message
#[derive(Clone)]
pub struct AirPlayTargetAdvertisement{
    flags: Option<u8>,
    seed: Option<u8>,
    ip_address: Ipv4Addr
}
impl AdvertisableData for AirPlayTargetAdvertisement {
    fn octets(&self) -> Vec<u8> {
        let flags = self.flags.unwrap_or(0x03);
        let seed = self.seed.unwrap_or(0x07);
        let ip_address = self.ip_address.octets();
        [
            0x09,
            0x06,
            flags,
            seed,
            ip_address[0],
            ip_address[1],
            ip_address[2],
            ip_address[3]
        ].to_vec()
    }
}