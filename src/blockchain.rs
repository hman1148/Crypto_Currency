use std::collections::HashSet;

use super::*;

#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimeStamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

pub struct BlockChain {
    pub blocks: Vec<Block>,
    unspent_outputs: std::collections::HashSet<Hash>
}

impl BlockChain {

    pub fn new() -> Self {
        BlockChain {
            blocks: std::vec![],
            unspent_outputs: std::collections::HashSet::new()
        }
    }

    pub fn update_with_block(&mut self, block: Block) -> Result<(), BlockValidationErr> {
           let index = self.blocks.len(); 
            
            if block.index != index as u32 {
                return Err(BlockValidationErr::MismatchedIndex);
            
            } else if !block::check_difficulty(&block.hash(), block.difficulty) {
                return Err(BlockValidationErr::InvalidHash);
            
            } else if index != 0 {
                // Not genesis block
                let prev_block = &self.blocks[index - 1];
                if block.timestamp <= prev_block.timestamp {
                    return Err(BlockValidationErr::AchronologicalTimeStamp);

                } else if block.prev_block_hash != prev_block.hash {
                    return Err(BlockValidationErr::MismatchedPreviousHash);
                }
            } else {
                // Genesis block
                if block.prev_block_hash != vec![0; 32] {
                   return Err(BlockValidationErr::InvalidGenesisBlockFormat);
                }
            }

            if let Some((coinbase, transactions)) = block.transactions.split_first() {
                if !coinbase.is_coinbase() {
                    return Err((BlockValidationErr::InvalidCoinbaseTransaction));
                }
                
                let mut block_spent: HashSet<Hash> = std::collections::HashSet::new();
                let mut block_created: HashSet<Hash> = std::collections::HashSet::new();

                let mut total_fee = 0; 

                for transaction in transactions {
                    let input_hashes = transaction.input_hashes();

                    if !(&input_hashes - &self.unspent_outputs).is_empty() || (&input_hashes & &block_spent).is_empty() {
                        return Err(BlockValidationErr::InvalidInput);
                    } 
                    

                    let input_value = transaction.input_value();
                    let output_value = transaction.output_value();

                    if output_value > input_value {
                        return Err(BlockValidationErr::InsufficientInputValue);
                    }

                    let fee = input_value - output_value;
                    total_fee += fee;

                    block_spent.extend(input_hashes);
                    block_created.extend(transaction.output_hashes());
                }   

                if coinbase.output_value() < total_fee {
                    return Err(BlockValidationErr::InvalidCoinbaseTransaction);
                } else {
                    block_created.extend(coinbase.output_hashes());
                }

                self.unspent_outputs.retain(|output| !block_spent.contains(output));
                self.unspent_outputs.extend(block_created);

            }
            self.blocks.push(block);
        return Ok(());
    }
}