use crate::keymanager::{hash_string, KeyManager};
use crate::traits::Printer;
use chrono;

#[derive(Clone)]
pub struct Transaction {
    to_address: String,
    amount: u64,
    timestamp: String,
    hash: String,
    signature: String,
    public_key: String,
}

impl Transaction {
    pub fn new(from_address: &KeyManager, to_address: String, amount: u64) -> Self {
        let mut t = Self {
            timestamp: format!("{:?}", chrono::offset::Utc::now()),
            amount,
            to_address,
            hash: String::new(),
            signature: String::new(),
            public_key: from_address.get_public_key(),
        };
        t.calc_hash();
        t.signature = from_address.sign(t.hash.clone());
        return t;
    }

    pub fn calc_hash(&mut self) {
        let mut s = String::new();
        s.push_str(&self.to_address);
        s.push_str(&self.public_key);
        s.push_str(&self.amount.to_string());
        s.push_str(&self.timestamp);
        self.hash = hash_string(&s);
    }

    pub fn verify(&self) -> bool {
        let message = self.hash.to_string();
        if self.amount == 0 {
            return false;
        }
        return KeyManager::new().verify(
            message,
            self.signature.to_string(),
            Some(self.public_key.to_string()),
        );
    }

    pub fn get_to_address(&self) -> &str {
        return &self.to_address[..];
    }

    pub fn get_amount(&self) -> u64 {
        return self.amount;
    }

    pub fn get_public_key(&self) -> &str {
        return &self.public_key[..];
    }
}

impl Printer for Transaction {
    fn print(&self) -> String {
        return format!(
            "{{From: {}, To: {}, Amount: {}, Timestamp: {}, Hash: {}, Signature: {}}}",
            self.public_key,
            self.to_address,
            self.amount,
            self.timestamp,
            self.hash,
            self.signature
        );
    }
}
