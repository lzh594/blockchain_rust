use std::thread;
use std::time::Duration;
use core::blockchain;
fn main() {
    let mut bc = blockchain::BlockChain::new();
    println!("start mining!");
    thread::sleep(Duration::from_secs(2));
    bc.add_block("a=>b: 9btc".to_string());
    thread::sleep(Duration::from_secs(2));
    bc.add_block("c".to_string());
    for block in &bc.blocks {
        println!("++++++++++++++++++++++++++++++++");
        println!("{:#?}", block);
        println!();
    }
    // println!("{:#?}",bc.blocks)
}
