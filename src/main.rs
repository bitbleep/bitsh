use bitsh::*;

fn main() {
    let mut buf = [0u8; 2];
    pack_le_u16(0x3, &mut buf, 7, 4);
    print_buf(&buf);
    pack_le_u16(0xf, &mut buf, 3, 4);
    print_buf(&buf);
    pack_le_i16(-1, &mut buf, 8, 8);
    print_buf(&buf);
    println!("unpack u16 7-11: {:04x}", unpack_le_u16(&buf, 7, 4));
    println!("unpack u16 3-7: {:04x}", unpack_le_u16(&buf, 3, 4));
    println!("unpack i16 8-16: {}", unpack_le_i16(&buf, 8, 8));
    println!("unpack u16 *: {:04x}", unpack_le_u16(&buf, 0, 16));
}

fn print_buf(buf: &[u8]) {
    print!("buf: ");
    for byte in buf {
        print!("{:02x} ", byte);
    }
    println!();
}
