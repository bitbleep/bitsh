use super::*;

#[test]
fn pack_and_unpack_u16() {
    let mut data = [0u8; 8];
    let value: u16 = 0x1234;
    value.pack_le_bits(&mut data, 0, 16);
    let unpacked_value = u16::unpack_le_bits(&data, 0, 16);
    assert_eq!(value, unpacked_value);
}

#[test]
fn pack_and_unpack_i16() {
    let mut data = [0u8; 8];
    let value = -123;
    value.pack_le_bits(&mut data, 0, 16);
    let unpacked_value = i16::unpack_le_bits(&data, 0, 16);
    assert_eq!(value, unpacked_value);
}
