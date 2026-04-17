use k256::ecdsa::{SigningKey, VerifyingKey, Signature};
use k256::elliptic_curve::sec1::ToEncodedPoint;
use sha2::{Sha256, Digest};
use anyhow::Result;

pub struct TeeKeyPair {
    signing_key: SigningKey,
}

impl TeeKeyPair {
    pub fn generate() -> Result<Self> {
        let signing_key = SigningKey::random(&mut rand::thread_rng());
        Ok(Self { signing_key })
    }

    pub fn public_key_bytes(&self) -> [u8; 64] {
        let verifying_key = self.signing_key.verifying_key();
        let encoded = verifying_key.to_encoded_point(false);
        let bytes = encoded.as_bytes();
        let mut arr = [0u8; 64];
        arr.copy_from_slice(&bytes[1..65]);
        arr
    }

    pub fn sign(&self, message: &[u8]) -> Result<[u8; 64]> {
        let digest = Sha256::digest(message);
        let (signature, _) = self.signing_key.sign_prehashed_recoverable(digest.into())?;
        let bytes = signature.to_bytes();
        let mut arr = [0u8; 64];
        arr.copy_from_slice(&bytes);
        Ok(arr)
    }
}

pub fn hash_data(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&result);
    arr
}
