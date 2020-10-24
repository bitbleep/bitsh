use bitsh::*;

fn main() {
    let mut buf = [0u8; 2];
    (0x3_u16).pack_le_bits(&mut buf, 7, 4);
    print_buf(&buf);
    (0xf_u16).pack_le_bits(&mut buf, 3, 4);
    print_buf(&buf);
    (-1_i16).pack_le_bits(&mut buf, 8, 8);
    print_buf(&buf);
    println!("unpack u16 7-11: {:04x}", u16::unpack_le_bits(&buf, 7, 4));
    println!("unpack u16 3-7: {:04x}", u16::unpack_le_bits(&buf, 3, 4));
    println!("unpack i16 8-16: {}", i16::unpack_le_bits(&buf, 8, 8));
    println!("unpack u16 *: {:04x}", u16::unpack_le_bits(&buf, 0, 16));
}

fn print_buf(buf: &[u8]) {
    print!("buf: ");
    for byte in buf {
        print!("{:02x} ", byte);
    }
    println!();
}
