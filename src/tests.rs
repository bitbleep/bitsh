use super::*;

#[test]
fn pack_and_unpack_u16() {
    let mut data = [0u8; 8];
    let value = 0x1234;
    pack_le_u16(value, &mut data, 0, 16);
    let unpacked_value = unpack_le_u16(&data, 0, 16);
    assert_eq!(value, unpacked_value);
}

#[test]
fn pack_and_unpack_i16() {
    let mut data = [0u8; 8];
    let value = -123;
    pack_le_i16(value, &mut data, 0, 16);
    let unpacked_value = unpack_le_i16(&data, 0, 16);
    assert_eq!(value, unpacked_value);
}
