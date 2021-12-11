extern crate rand;
extern crate secp256k1;

use secp256k1::bitcoin_hashes::sha256;
use secp256k1::rand::rngs::OsRng;
use secp256k1::{All, Message, PublicKey, Secp256k1, SecretKey, Signature};
use sha2::{Digest, Sha256};
use std::str::FromStr;

pub struct KeyManager {
    secp: Secp256k1<All>,
    public_key: String,
    secret_key: String,
}

/* KeyManager manages keys for the transactions */
impl KeyManager {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng::new().expect("OsRng");
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);
        return Self {
            secp,
            secret_key: secret_key.to_string(),
            public_key: public_key.to_string(),
        };
    }

    /* Sign a message */
    pub fn sign(&self, message: String) -> String {
        return self
            .secp
            .sign(
                &Message::from_hashed_data::<sha256::Hash>(message.as_bytes()),
                &SecretKey::from_str(&self.secret_key[..]).unwrap(),
            )
            .to_string();
    }

    /* Verify a message */
    pub fn verify(&self, message: String, signature: String, public_key: Option<String>) -> bool {
        let public_key_ = match public_key {
            Some(public_key) => PublicKey::from_str(&public_key[..]).unwrap(),
            None => PublicKey::from_str(&self.public_key[..]).unwrap(),
        };
        return self
            .secp
            .verify(
                &Message::from_hashed_data::<sha256::Hash>(message.as_bytes()),
                &Signature::from_str(&signature[..]).unwrap(),
                &public_key_,
            )
            .is_ok();
    }

    pub fn get_public_key(&self) -> String {
        return self.public_key.clone();
    }
}

/* sha256 */
pub fn hash_string(in_str: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(in_str);
    return format!("{:x}", hasher.finalize());
}
