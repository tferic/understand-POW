use ruint::{Uint, uint};
use rand::Rng;
use sha2::{Digest, Sha256};

fn main() {
    const DIFFICULTY: Uint<256, 4> = uint!(0b0011111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111_U256);
    println!("My difficulty:         {:x}", DIFFICULTY);
    let mut rng = rand::thread_rng();
    let random_number: u128 = rng.gen();

    let mut myhash = Sha256::new();
    myhash.update(random_number.to_be_bytes());
    let mut myhash_str: String = format!("{:x}", myhash.finalize());
    println!("My hash:               {}", myhash_str);
    let mut difficulty_str: String = format!("{:x}", DIFFICULTY);

    difficulty_str.truncate(8);
    myhash_str.truncate(8);
    println!("My difficulty (trunc): {}", difficulty_str);
    println!("My hash (trunc):       {}", myhash_str);

}
