use blockchainlib::Block;
use blockchainlib::BlockChain;
use blockchainlib::Hashable;

fn main () {

    let difficulty = 0x000fffffffffffffffffffffffffffff as u128;
    let mut block = Block::new(1, 0, vec![0; 32], 0, "Genesis Block".to_owned(), difficulty);

    block.mine();
    println!("Mined genesis block {:?}", &block);

    let mut last_hash = block.hash.clone();


    let mut blockchain = BlockChain {
        blocks: std::vec![block],
    };
    

    for i in 1..=10 {
        let mut block = Block::new(i, 0, last_hash, 0, "Another block".to_owned(), difficulty);

        block.mine();
        println!("Mined genesis block {:?}", &block);

        last_hash = block.hash.clone();

        blockchain.blocks.push(block);
    
    }
}
