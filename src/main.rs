use bitsh::*;

fn main() {
    let from = [0x3f_u8, 0];
    let mut buf = [0u8; 2];

    println!("try le i16");
    pack_le_i16(3, &mut buf, 7, 4);
    println!("unpack_le_i16; {}", unpack_le_i16(&buf, 7, 4));
    println!("buf: {:?}", buf);

    println!("try be i16");
    pack_be_i16(3, &mut buf, 0, 4);
    println!("unpack_be_i16; {}", unpack_be_i16(&buf, 0, 4));
    println!("buf: {:?}", buf);
}
