use crate::keymanager::hash_string;
use crate::traits::Printer;
use crate::transaction::Transaction;
use chrono;
use crossterm::{cursor, QueueableCommand};
use std::io::{stdout, Write};

#[derive(Clone)]
pub struct Block {
    timestamp: String,
    nonce: u64,
    hash: String,
    previous_hash: String,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new() -> Self {
        let b = Self {
            timestamp: format!("{:?}", chrono::offset::Utc::now()),
            nonce: 0,
            hash: String::new(),
            previous_hash: String::new(),
            transactions: Vec::<Transaction>::new(),
        };
        b.calc_hash();
        return b;
    }

    pub fn calc_hash(&self) -> String {
        let mut s: String = String::new();
        s.push_str(&self.get_transactions_str());
        s.push_str(&self.timestamp);
        s.push_str(&self.nonce.to_string());
        s.push_str(&self.previous_hash);
        return hash_string(&s);
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let hash_compare = String::from_utf8(vec![b'0'; difficulty]).unwrap();
        let mut hash_start: String = self.hash.chars().take(difficulty).collect();
        let mut stdout = stdout();

        while !hash_compare.eq(&hash_start) {
            self.nonce += 1;
            self.hash = self.calc_hash();
            hash_start = self.hash.chars().take(difficulty).collect();
            // print for debugging
            stdout.queue(cursor::SavePosition).expect("cursor save");
            stdout
                .write(format!("Mining hash: {}", self.hash).as_bytes())
                .expect("write hash");
            stdout
                .queue(cursor::RestorePosition)
                .expect("cursor restore");
            stdout.flush().expect("flush");
        }
        println!("\n");
    }

    pub fn verify(&self) -> bool {
        for transaction in self.transactions.iter() {
            if !transaction.verify() {
                return false;
            }
        }

        return self.get_hash().eq(&self.calc_hash());
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    pub fn get_transactions(&self) -> &Vec<Transaction> {
        return &self.transactions;
    }

    pub fn get_transactions_str(&self) -> String {
        let mut s: String = String::new();
        s.push_str("[");
        for transaction in self.transactions.iter() {
            let t: String = transaction.print();
            if s.len() > 1 {
                s.push_str(",");
            }
            s.push_str(&t);
        }
        s.push_str("]");
        return s;
    }

    pub fn set_previous_hash(&mut self, hash: &str) {
        self.previous_hash = hash.to_string();
    }

    pub fn get_previous_hash(&self) -> &str {
        return &self.previous_hash[..];
    }

    pub fn get_hash(&self) -> &str {
        return &self.hash[..];
    }

    pub fn set_hash(&mut self, hash: &str) {
        self.hash = hash.to_string();
    }

    pub fn set_timestamp(&mut self, timestamp: &str) {
        self.timestamp = timestamp.to_string();
    }
}

impl Printer for Block {
    fn print(&self) -> String {
        return format!(
            "{{Transactions: {}, Timestamp: {}, Nonce: {}, Hash: {}, PreviousHash: {}}}",
            self.get_transactions_str(),
            self.timestamp,
            self.nonce,
            self.hash,
            self.previous_hash
        );
    }
}

impl PartialEq<Block> for Block {
    fn eq(&self, other: &Block) -> bool {
        if self.timestamp[..].eq(&other.timestamp[..])
            && self.hash[..].eq(&other.hash[..])
            && self.previous_hash[..].eq(&other.previous_hash[..])
            && self
                .get_transactions_str()
                .eq(&other.get_transactions_str())
        {
            return true;
        }
        return false;
    }
}
