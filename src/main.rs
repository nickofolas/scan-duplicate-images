use image_hash::{compare_hashes, generate_hash, Result as LazyResult};
use std::fs::read;

fn main() -> LazyResult<()> {
    let buffer_1: Vec<u8> = read("test_case_0.png")?;
    let buffer_2: Vec<u8> = read("test_case_1.png")?;

    let hash_1 = generate_hash(&buffer_1)?;
    let hash_2 = generate_hash(&buffer_2)?;

    println!(
        "{:?}\n{:?}\n{:?}",
        hash_1,
        hash_2,
        compare_hashes(hash_1 as usize, hash_2 as usize)
    );

    Ok(())
}
