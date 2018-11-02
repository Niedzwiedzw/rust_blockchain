use crate::*;
use sha2::{Sha256, Digest};
use std::fmt;
use crate::proof::Proof;

#[derive(Debug)]
pub struct BlockChain {
    pub blocks: Vec<Block>
}

impl BlockChain {
    pub fn new() -> Self {
        let mut block = Block::new(&String::from("Genesis block"), 1);
        let mut proof = Proof::new(&block);

        let (nonce, hash) = proof.run();
        block.nonce = nonce;
        block.hash = hash;

        BlockChain {
            blocks: vec![block]
        }
    }

    pub fn add(&mut self, data: &String) {
        let new_block = create_block(data, &self.blocks.last().unwrap());
        self.blocks.push(new_block);
    }
}

pub fn to_text(hash: &Hash) -> String {
    let mut buffer = String::new();
    for byte in hash.iter() {
        buffer += &format!("{:x}", *byte);
    }
    buffer
}


pub struct Block {
    pub hash: Hash,
    pub data: BlockData,
    pub prev_hash: Hash,
    pub nonce: u32,
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Block {{\n  data: {},\n  hash: {},\n  prev_hash: {}\n}}",
               format!("{}", String::from_utf8_lossy(&self.data[..])),
               format!("{:?}", to_text(&self.hash)),
               format!("{:?}", to_text(&self.prev_hash))
        )
    }
}

impl Block {
    pub fn new(data: &String, nonce: u32) -> Self {
        let mut block = Block {
            hash: [0; HASH_SIZE],
            data: [0; DATA_SIZE],
            prev_hash: [0; HASH_SIZE],
            nonce,
        };
        for (i, byte) in data.chars().enumerate() {
            block.data[i] = byte as u8;
        }
        block
    }
}

fn create_block(data: &String, prev: &Block) -> Block {
    let mut block = Block::new(&data, 0);
    for (i, byte) in prev.hash.iter().enumerate() {
        block.prev_hash[i] = *byte
    }


    let mut proof = Proof::new(&block);

    let (nonce, hash) = proof.run();
    block.nonce = nonce;
    block.hash = hash;

    block

}