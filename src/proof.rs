use crate::blockchain::{ Block, to_text };
use crate::{HASH_SIZE, DATA_SIZE, Hash};
use sha2::{Sha256, Digest};
use std::mem::transmute;

const DIFFICULTY: u64 = 8;
const BIG_BUFF_SIZE: usize = 2*8 + DATA_SIZE + HASH_SIZE;

type BigBuff = [u8; BIG_BUFF_SIZE];


fn following_zeroes(hash: &Hash) -> u32 {
    let mut zeroes = 0;
    for byte in hash {
        for i in (0..8).rev() {
            if ((*byte & 1 << i) >> i) == 0 {
                zeroes += 1;
            } else {
                return zeroes
            }
        }
    }
    zeroes
}


#[test]
fn test_following_zeroes() {
    let mut hash: Hash = [0; HASH_SIZE];
    assert_eq!(following_zeroes(&hash), 256);
    hash[0] = 1;
    assert_eq!(following_zeroes(&hash), 7);
}


fn to_hex(number: u64) -> [u8; 8] {
    let bytes: [u8; 8] = unsafe { transmute(number.to_le()) };
    bytes
}

fn to_hash(data: &[u8]) -> Hash {
    let mut hash = [0; HASH_SIZE];
    let mut hasher = Sha256::new();
    hasher.input(data);
    for (i, byte) in hasher.result().iter().enumerate() {
        hash[i] = *byte;
    }

    hash
}


pub struct Proof<'a> {
    block: &'a Block,
    target: u32,
}

impl<'a> Proof<'a> {
    pub fn new(block: &'a Block) -> Self {
        let target = 1;
        let target = target << HASH_SIZE - DIFFICULTY as usize;

        Proof { block, target }
    }

    pub fn init_data(&mut self, nonce: u32) -> BigBuff {
        let mut buff: BigBuff = [0; BIG_BUFF_SIZE];
        for (i, byte) in self.block.prev_hash.iter()
            .chain(self.block.data.iter())
            .chain(to_hex(nonce as u64).iter())
            .chain(to_hex(DIFFICULTY).iter())
            .enumerate() {
            buff[i] = *byte;
        }
        buff
    }

    pub fn validate(&mut self) -> bool {
        let data = self.init_data(self.block.nonce);
        let hash = to_hash(&data);

        following_zeroes(&hash) < self.target
    }

    pub fn run(&mut self) -> (u32, Hash) {
        for nonce in 0..u32::max_value() {
            let data = self.init_data(nonce);
            let hash = to_hash(&data[..]);
            let zeroes = following_zeroes(&hash);


            if zeroes > DIFFICULTY as u32 {
                println!("{}", to_text(&hash));
                return (zeroes, hash)
            }
        }
        panic!("Could not find a nonce to match the requirements...");
    }
}
