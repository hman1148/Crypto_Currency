use blockchainlib::*;

fn main () {
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    let mut genesis_block: Block = Block::new(0, now(), vec![0; 32], vec![
        Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_address: "Alice".to_owned(),
                    value: 50
                },
                transaction::Output {
                    to_address: "Bob".to_owned(),
                    value: 7
                }
            ]
        }
    ],  difficulty);

    genesis_block.mine();
    println!("Mined genesis block {:?}", &genesis_block);

    let mut blockchain = BlockChain::new();

    blockchain.update_with_block(genesis_block).expect("Failed to add genesis block");

}