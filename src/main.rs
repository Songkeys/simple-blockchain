mod block;
mod blockchain;
mod keymanager;
mod traits;
mod transaction;

use blockchain::Blockchain;
use keymanager::KeyManager;
use transaction::Transaction;

// use crate::traits::Printer;

fn main() {
    // Some addresses
    let mining_pool = KeyManager::new();
    let wallet_a = KeyManager::new();
    let wallet_b = KeyManager::new();
    let wallet_c = KeyManager::new();

    // Create a new blockchain
    let mining_reward = 100;
    let difficulty = 4;
    println!(
        "Creating a new blockchain with mining_reward: {}, difficulty: {}\n",
        mining_reward, difficulty
    );
    let mut blockchain: Blockchain = Blockchain::new(mining_reward, difficulty);
    // println!("{}\n", blockchain.print());

    // Mine first block
    println!("Mining the first block (by wallet_a)\n");
    blockchain.mine_pending_transactions(&mining_pool, wallet_a.get_public_key());
    // println!("{}\n", blockchain.print());

    println!("Creating transactions t1 & t2\n");
    println!("t1: wallet_a ==50==> wallet_b\n");
    let t1: Transaction = Transaction::new(&wallet_a, wallet_b.get_public_key(), 50);
    println!("t2: wallet_a ==20==> wallet_b\n");
    let t2: Transaction = Transaction::new(&wallet_b, wallet_c.get_public_key(), 20);
    // println!("t1: {}\nt2: {}\n", t1.print(), t2.print());

    blockchain
        .add_transaction(t1)
        .expect("Adding a transaction");
    println!("Mining block t1... (by wallet_a)\n");
    blockchain.mine_pending_transactions(&mining_pool, wallet_a.get_public_key());
    // println!("{}\n", blockchain.print());

    blockchain
        .add_transaction(t2)
        .expect("Adding a transaction");
    println!("Mining block t2... (by wallet_a)\n");
    blockchain.mine_pending_transactions(&mining_pool, wallet_a.get_public_key());
    // println!("{}\n", blockchain.print());

    println!("Creating transactions t3 & t4\n");
    println!("t3: wallet_a ==200==> wallet_c\n");
    let t3: Transaction = Transaction::new(&wallet_a, wallet_c.get_public_key(), 200);
    println!("t4: wallet_c ==10==> wallet_b\n");
    let t4: Transaction = Transaction::new(&wallet_c, wallet_b.get_public_key(), 10);
    // println!("t3: {}\nt4: {}\n", t3.print(), t4.print());

    blockchain
        .add_transaction(t3)
        .expect("Adding a transaction");
    blockchain
        .add_transaction(t4)
        .expect("Adding a transaction");
    println!("Mining block t3... (by wallet_a)\n");
    blockchain.mine_pending_transactions(&mining_pool, wallet_a.get_public_key());
    // println!("{}\n", blockchain.print());

    println!(
        "Wallet a: {}\nWallet b: {}\nWallet c: {}\n",
        blockchain.get_balance(&wallet_a.get_public_key()).unwrap(),
        blockchain.get_balance(&wallet_b.get_public_key()).unwrap(),
        blockchain.get_balance(&wallet_c.get_public_key()).unwrap()
    );

    println!(
        "Blockchain valid? {}",
        if blockchain.verify() { "Yes" } else { "No" }
    );
}
