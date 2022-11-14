use core::block::*;

fn main() {
    let block = Block::new(
        "data".to_string(),
        "63c222837c6c8ce660c2d9b974e3d6d2935e3697e03a19493319cc5e03ff017e".to_string(),
    );
    println!("{:?}", block);
}
