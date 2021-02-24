use super::*;
use proptest::{prelude::ProptestConfig, prop_assert_eq, proptest};

// todo: not sure about this
// #[test]
// #[should_panic]
// fn value_exceeding_number_of_bits_should_panic() {
//     let mut data = [0u8; 8];
//     0x1234_u16.pack_le_bits(&mut data, 0, 2);
// }

#[test]
#[should_panic]
fn packing_exceeding_buffer_should_panic() {
    let mut data = [0u8; 2];
    0x1234_u16.pack_le_bits(&mut data, 8, 12);
}

#[test]
#[should_panic]
fn packing_more_bits_than_type_holds_should_panic() {
    let mut data = [0u8; 4];
    0x1234_u16.pack_le_bits(&mut data, 0, 17);
}

#[test]
fn u16_little_endian_byte_order_correct() {
    let mut data = [0u8; 4];
    let value: u16 = 0x1234;
    value.pack_le_bits(&mut data, 0, 16);
    assert_eq!(&data, &[0x34, 0x12, 0, 0]);
}

#[test]
fn u16_big_endian_byte_order_correct() {
    let mut data = [0u8; 4];
    let value: u16 = 0x1234;
    value.pack_be_bits(&mut data, 0, 16);
    assert_eq!(&data, &[0x12, 0x34, 0, 0]);
}

#[test]
fn u16_pack_with_offset() {
    let mut data = [0u8; 4];
    let value = 0xf;
    value.pack_le_bits(&mut data, 7, 4);
    assert_eq!(&data, &[0x80, 0x07, 0, 0]);
}

#[test]
fn u16_little_endian_pack_and_unpack_with_offset() {
    let mut data = [0u8; 8];
    let value = 0x123;
    value.pack_le_bits(&mut data, 11, 13);
    let unpacked_value = u16::unpack_le_bits(&mut data, 11, 13);
    assert_eq!(value, unpacked_value);
}

#[test]
fn u16_little_endian_pack_and_unpack() {
    let mut data = [0u8; 2];
    let value = 0x1234;
    value.pack_le_bits(&mut data, 0, 16);
    let unpacked_value = u16::unpack_le_bits(&data, 0, 16);
    assert_eq!(value, unpacked_value);
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 10000,  .. ProptestConfig::default()
      })]
    #[test]
    fn u16_little_endian_pack_and_unpack_proptest(value in (0..65535u16), start_bit in (0..15usize)) {
        let mut data : [u8; 2]= [0, 0];
        // bits required to store value
        let num_bits = ((value as f32).log2() + 1.0) as usize;
        let start_bit = if start_bit + num_bits > 16 { 16 - num_bits } else { start_bit };

        value.pack_le_bits(&mut data, start_bit, num_bits);
        let unpacked_value = u16::unpack_le_bits(&data, start_bit, num_bits);
        prop_assert_eq!(value, unpacked_value)
    }
}

#[test]
fn u16_big_endian_pack_and_unpack() {
    let mut data = [0u8; 2];
    let value = 0x1234;
    value.pack_be_bits(&mut data, 0, 16);
    let unpacked_value = u16::unpack_be_bits(&data, 0, 16);
    assert_eq!(value, unpacked_value);
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 10000, .. ProptestConfig::default()
      })]
    #[test]
    fn u16_big_endian_pack_and_unpack_proptest(value in (0..65535u16), start_bit in (0..15usize)) {
        let mut data : [u8; 2]= [0, 0];
        // bits required to store value
        let num_bits = ((value as f32).log2() + 1.0) as usize;
        let start_bit = if start_bit + num_bits > 16 { 16 - num_bits } else { start_bit };

        value.pack_be_bits(&mut data, start_bit, num_bits);
        let unpacked_value = u16::unpack_be_bits(&data, start_bit, num_bits);
        prop_assert_eq!(value, unpacked_value)
    }
}

#[test]
fn i16_little_endian_byte_order_correct() {
    let mut data = [0u8; 2];
    let value: i16 = 0x1234;
    value.pack_le_bits(&mut data, 0, 16);
    assert_eq!(&data[..2], &[0x34, 0x12]);
}

#[test]
fn i16_big_endian_byte_order_correct() {
    let mut data = [0u8; 2];
    let value: u16 = 0x1234;
    value.pack_be_bits(&mut data, 0, 16);
    assert_eq!(&data[..2], &[0x12, 0x34]);
}

#[test]
fn i16_little_endian_pack_and_unpack() {
    let mut data = [0u8; 2];
    let value = -1234;
    value.pack_le_bits(&mut data, 0, 16);
    let unpacked_value = i16::unpack_le_bits(&data, 0, 16);
    assert_eq!(value, unpacked_value);
}

#[test]
fn i16_big_endian_pack_and_unpack() {
    let mut data = [0u8; 2];
    let value = -1234;
    value.pack_be_bits(&mut data, 0, 16);
    let unpacked_value = i16::unpack_be_bits(&data, 0, 16);
    assert_eq!(value, unpacked_value);
}

#[test]
fn i16_pack_with_offset() {
    let mut data = [0u8; 4];
    let value = -1_i16;
    value.pack_le_bits(&mut data, 7, 4);
    assert_eq!(&data, &[0x80, 0x07, 0, 0]);
}
