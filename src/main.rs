mod bind_snappy;

use bind_snappy::*;

fn main() {
    let x = max_compressed_length(100);
    println!("max compressed length of a 100 byte buffer: {}", x);
}