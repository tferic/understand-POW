use ruint::{Uint, uint};
use rand::Rng;
use sha2::{Digest, Sha256};

fn main() {
    const DIFFICULTY: Uint<256, 4> = uint!(0b0011111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111_U256);
    println!("My 256-bit number: {:x}", DIFFICULTY);
    let mut rng = rand::thread_rng();
    let random_number: u128 = rng.gen();
    let myhash = Sha256::digest(&random_number.to_be_bytes());
    println!("My hash:           {:x}", myhash);
}
