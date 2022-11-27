use sha2::{Sha256, Digest};
pub fn get_first_two_bytes_of_sha256(input: impl AsRef<[u8]>) -> [u8; 2] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    [result[0], result[1]]
}
pub struct TraitContainer<T>(T);