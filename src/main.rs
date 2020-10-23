use bitsh::*;

fn main() {
    let from = [0x3f_u8, 0];
    let mut to = [0u8; 2];

    pack_bits(&from, &mut to, 2, 2);

    println!("from: {:02x}", from[0]);
    println!("to: {:?}", to);
}
