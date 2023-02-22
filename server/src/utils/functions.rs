use rand::RngCore;
pub use shared_utils::*;

pub fn generate_secret() -> [u8; 64] {
    let mut secret: [u8; 64] = [0u8; 64];
    rand::thread_rng().fill_bytes(&mut secret);
    secret
}
