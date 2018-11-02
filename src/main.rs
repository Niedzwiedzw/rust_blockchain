mod blockchain;
mod proof;

use self::blockchain::{BlockChain, to_text};
use self::proof::Proof;


const DATA_SIZE: usize = 1024;
const HASH_SIZE: usize = 32;
const BUFF_SIZE: usize = DATA_SIZE + HASH_SIZE;

type BlockBuff = [u8; BUFF_SIZE];
type Hash = [u8; HASH_SIZE];
type BlockData = [u8; DATA_SIZE];


fn main() {
    let mut blockchain = BlockChain::new();

    blockchain.add(&String::from("First block after Genesis"));
    blockchain.add(&String::from("Second block after Genesis"));
    blockchain.add(&String::from("Third block after Genesis"));

    for block in blockchain.blocks {
        println!("{:?}", block);

        let mut proof = Proof::new(&block);
        println!("proof: {}\n\n", proof.validate());
    }
}
