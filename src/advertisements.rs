use std::{collections::BTreeMap, error::Error, time::Duration};

#[cfg(feature = "disable_afit")]
use async_trait::async_trait;
use bluer::adv::{Advertisement, AdvertisementHandle, Type};

use crate::session::Session;
use crate::util::get_first_two_bytes_of_sha256;

pub trait AdvertisableData: Into<Vec<u8>> + Clone {
    fn get_user_data_length() -> usize;
}

// If the user opted out of using "async_fn_in_trait", use the crate async-trait instead.
#[cfg_attr(feature = "disable_afit", async_trait)]
/// Any kind of advertisement.
pub trait Advertisable<T: AdvertisableData> {
    /// Assemble user data.
    fn assemble_user_data(user_data: Option<T>) -> Vec<u8> {
        match user_data {
            Some(ud) => ud.into(),
            None => vec![0; T::get_user_data_length()] // NOTE: This took me fucking forever! Because stupid me wanted to use const generics.
        }
    }
    /// Advertisement-specific: validate user supplied data.
    fn validate_user_data(_user_data: &Option<T>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    /// Advertisement-specific: assemble user supplied data to advertisement.
    fn assemble_advertisement(
        session: &Session,
        user_data: &Option<T>,
    ) -> Result<Advertisement, Box<dyn Error>>;
    /// Register any advertisement.
    async fn register(
        session: &Session,
        user_data: &Option<T>,
    ) -> Result<AdvertisementHandle, Box<dyn Error>> {
        Self::validate_user_data(user_data)?;

        Ok(session
            .adapter
            .advertise(Self::assemble_advertisement(session, user_data)?)
            .await?)
    }
}
pub struct AirDropAdvertisement;
#[derive(Clone)]
/// Data for an AirDrop advertisement.
pub struct AirDropAdvertisementData {
    pub apple_id: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub email2: Option<String>,
}
impl AdvertisableData for AirDropAdvertisementData {
    fn get_user_data_length() -> usize {
        8 // 4 * 2bytes = 8bytes
}
}
impl Into<Vec<u8>> for AirDropAdvertisementData {
    fn into(self) -> Vec<u8> {
        [
            get_first_two_bytes_of_sha256(self.apple_id.unwrap_or_default()).to_be_bytes(),
            get_first_two_bytes_of_sha256(self.phone.unwrap_or_default()).to_be_bytes(),
            get_first_two_bytes_of_sha256(self.email.unwrap_or_default()).to_be_bytes(),
            get_first_two_bytes_of_sha256(self.email2.unwrap_or_default()).to_be_bytes(),
        ]
        .concat()
    }
}
impl Advertisable<AirDropAdvertisementData> for AirDropAdvertisement {
    fn assemble_advertisement(
        session: &Session,
        user_data: &Option<AirDropAdvertisementData>,
    ) -> Result<Advertisement, Box<dyn Error>> {
        let advertisement = Advertisement {
            advertisement_type: Type::Broadcast,
            discoverable: Some(true),
            local_name: Some(session.adapter.name().to_string()),
            timeout: Some(Duration::from_millis(0)),
            min_interval: Some(Duration::from_millis(100)),
            max_interval: Some(Duration::from_millis(200)),
            service_uuids: vec![Uuid]
            manufacturer_data: BTreeMap::from([(
                0x4C,
                Self::assemble_user_data(user_data.clone()),
            )]),
            ..Default::default()
        };
        println!("{:#?}", advertisement);
        Ok(advertisement)
    }
}
