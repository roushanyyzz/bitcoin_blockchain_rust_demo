use core::blockchain;
use std::thread;
use std::time::Duration;

fn main() {
    let mut bc = blockchain::BlockChain::new();
    println!("Strart mint...");
    thread::sleep(Duration::from_secs(5));
    bc.add_block("a -> b: 5btc".to_string());
    println!("Mint 1 block");
    println!("Strart mint...");
    thread::sleep(Duration::from_secs(5));
    bc.add_block("c -> d: 1btc".to_string());
    println!("Mint 1 block");

    for block in bc.blocks {
        println!("{:#?}", block);
    }
}
