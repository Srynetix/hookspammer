use hmac::{Hmac, Mac};
use sha2::Sha256;

pub fn compute_signature(body: &[u8], secret: &str) -> String {
    let mut hmac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    hmac.update(body);

    hex::encode(hmac.finalize().into_bytes())
}
