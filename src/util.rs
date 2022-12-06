use std::error::Error;

use sha2::{Digest, Sha256};

use crate::session::Session;
pub fn get_first_two_bytes_of_sha256(input: impl AsRef<[u8]>) -> [u8; 2] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    [result[0], result[1]]
}
pub fn set_device_addr(session: &Session, device_addr: &[u8]) -> Result<(), Box<dyn Error>> {
    sudo::escalate_if_needed()?;
    std::process::Command::new("bdaddr").args([
        "-i",
        session.adapter.name(),
        &eui48::MacAddress::from_bytes(device_addr)?.to_hex_string(),
    ]);
    systemctl::restart("bluetooth.service")?;
    Ok(())
}
