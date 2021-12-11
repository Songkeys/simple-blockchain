use crate::block::Block;
use crate::keymanager::KeyManager;
use crate::traits::Printer;
use crate::transaction::Transaction;

pub struct Blockchain {
    blocks: Vec<Block>,
    mining_reward: u64,
    difficulty: usize,
    pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new(mining_reward: u64, difficulty: usize) -> Self {
        let mut blockchain = Self {
            blocks: Vec::<Block>::new(),
            mining_reward,
            difficulty,
            pending_transactions: Vec::<Transaction>::new(),
        };
        let genesis = blockchain.create_genesis_block();
        blockchain.blocks.clear();
        blockchain.blocks.push(genesis);
        return blockchain;
    }

    pub fn create_genesis_block(&self) -> Block {
        let mut genesis_block = Block::new();
        genesis_block.set_timestamp(&"Beginning of time".to_string());
        genesis_block.set_hash(&genesis_block.calc_hash());
        return genesis_block;
    }

    pub fn get_last_hash(&self) -> Result<&str, &'static str> {
        match self.blocks.last() {
            None => Err("No blocks in the chain"),
            Some(b) => Ok(b.get_hash()),
        }
    }

    pub fn verify(&self) -> bool {
        let genesis = self.create_genesis_block();
        let first = self.blocks[0].clone();

        if genesis != first {
            return false;
        }

        let mut previous_block: Option<&Block> = None;
        for block in self.blocks.iter().skip(1) {
            if !block.verify() {
                return false;
            }
            match previous_block {
                None => (),
                Some(b) => {
                    if !block.get_previous_hash().eq(b.get_hash()) {
                        println!(
                            "prev_block hash {}, block previous_hash {}",
                            block.get_previous_hash(),
                            b.get_hash()
                        );
                        return false;
                    }
                }
            }
            previous_block = Some(block);
        }
        return true;
    }

    pub fn add_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<&'static str, &'static str> {
        if !transaction.verify() {
            return Err("Invalid transaction");
        }

        match self.get_balance(&transaction.get_public_key()) {
            Ok(balance) => {
                if balance < transaction.get_amount() {
                    return Err("Insufficient funds.");
                }
            }
            Err(s) => {
                return Err(s);
            }
        }

        self.pending_transactions.push(transaction);
        return Ok("Transaction added");
    }

    pub fn mine_pending_transactions(&mut self, keys: &KeyManager, reward_address: String) {
        let reward_transaction = Transaction::new(keys, reward_address.clone(), self.mining_reward);
        let mut block = Block::new();
        block.add_transaction(reward_transaction);
        for transaction in self.pending_transactions.iter() {
            block.add_transaction(transaction.clone());
        }
        block.set_previous_hash(self.get_last_hash().expect("No genesis block"));
        block.mine_block(self.difficulty);
        self.blocks.push(block);
        self.pending_transactions.clear();
    }

    pub fn get_balance(&self, address: &str) -> Result<u64, &'static str> {
        let mut balance = 0;
        for block in self.blocks.iter().skip(1) {
            for transaction in block.get_transactions().iter() {
                if transaction.get_to_address().eq(address) {
                    balance += transaction.get_amount();
                }
                // public key is 'from_address'
                else if transaction.get_public_key().eq(address) {
                    if transaction.get_amount() > balance {
                        return Err("Invalid chain");
                    }
                    balance -= transaction.get_amount();
                }
            }
        }
        return Ok(balance);
    }
}

impl Printer for Blockchain {
    fn print(&self) -> String {
        let mut s: String = String::new();
        for block in self.blocks.iter() {
            if s.len() == 0 {
                s.push_str("[");
            } else {
                s.push_str(",");
            }
            s.push_str(block.print().as_str());
        }
        if s.len() > 0 {
            s.push_str("]");
        }
        return s;
    }
}
