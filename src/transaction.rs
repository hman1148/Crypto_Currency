use std::collections::HashSet;

use super::*;

pub struct Output {
    pub to_address: Address,
    pub value: u64
}

impl Hashable for Output {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.to_address.as_bytes());
        bytes.extend(&u64_bytes(&self.value));
        
        return bytes;
    }
}

pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}

impl Transaction {

    pub fn input_value(&self) -> u64 {
        return self.inputs
        .iter()
        .map(|input| input.value)
        .sum()
    }

    pub fn output_value(&self) -> u64 {
        return self.inputs
        .iter()
        .map(|output| output.value)
        .sum()
    }

    pub fn input_hashes(&self) -> std::collections::HashSet<Hash> {
        return self.inputs
        .iter()
        .map(|input| input.hash())
        .collect::<std::collections::HashSet<Hash>>()
    }

    pub fn output_hashes(&self) -> std::collections::HashSet<Hash> {
        return self.inputs
        .iter()
        .map(|output| output.hash())
        .collect::<std::collections::HashSet<Hash>>()
    }

    pub fn is_coinbase(&self) -> bool {
        return self.inputs.len() == 0;
    }


}

impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(
            self.inputs
            .iter()
            .flat_map(|input| input.bytes())
            .collect::<Vec<u8>>()
        );

        bytes.extend(
            self.inputs
            .iter()
            .flat_map(|output| output.bytes())
            .collect::<Vec<u8>>()
        );
        return bytes;
    }
}