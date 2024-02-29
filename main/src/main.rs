use core::blockchain;

fn main() {
    let mut bc = blockchain::BlockChain::new_blockchain();
    bc.add_block("first block".to_string());
    bc.add_block("second block".to_string());
    bc.add_block("third block".to_string());
    for b in bc.blocks {
        println!("============");
        println!("{:#?}", b);
        println!("============");
    }
}
