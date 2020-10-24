#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone)]
pub enum Packing {
    SignedLittleEndian,
    SignedBigEndian,
    UnsignedLittleEndian,
    UnsignedBigEndian,
}

pub fn unpack_le_i16(data: &[u8], start_bit: usize, num_bits: usize) -> i16 {
    let mut tmp = [0u8; 2];
    unpack_bits(
        data,
        &mut tmp,
        start_bit,
        num_bits,
        Packing::SignedLittleEndian,
    );
    i16::from_le_bytes([tmp[0], tmp[1]])
}

pub fn pack_le_i16(value: i16, data: &mut [u8], start_bit: usize, num_bits: usize) {
    let tmp = value.to_le_bytes();
    unpack_bits(&tmp, data, start_bit, num_bits, Packing::SignedLittleEndian);
}

pub fn unpack_be_i16(data: &[u8], start_bit: usize, num_bits: usize) -> i16 {
    let mut tmp = [0u8; 2];
    unpack_bits(
        data,
        &mut tmp,
        start_bit,
        num_bits,
        Packing::SignedLittleEndian,
    );
    i16::from_be_bytes([tmp[0], tmp[1]])
}

pub fn pack_be_i16(value: i16, data: &mut [u8], start_bit: usize, num_bits: usize) {
    let tmp = value.to_be_bytes();
    unpack_bits(&tmp, data, start_bit, num_bits, Packing::SignedLittleEndian);
}

#[inline(always)]
fn unpack_bits(from: &[u8], to: &mut [u8], start_bit: usize, num_bits: usize, packing: Packing) {
    // eprintln!(
    //     "unpack_bits; start_bit: {} num_bits: {}",
    //     start_bit, num_bits
    // );
    let init_value = match packing {
        Packing::SignedLittleEndian | Packing::SignedBigEndian => {
            // eprintln!(
            //     "unpack_bits; is_negative: {}",
            //     is_negative(from, start_bit, num_bits, packing),
            // );
            match is_negative(from, start_bit, num_bits, packing) {
                true => 0xff,
                false => 0x00,
            }
        }
        _ => 0,
    };
    for byte in to.iter_mut() {
        *byte = init_value;
    }
    let end_bit = start_bit + num_bits;
    let mut offset: usize = 0;
    while offset < end_bit {
        let shift = offset % 8;
        let take = 8 - shift;
        let value = from[offset / 8] >> shift;
        let mask = 0xff_u8.wrapping_shl(take as u32);
        let byte_offset = (offset - start_bit) / 8;
        to[byte_offset] &= mask;
        to[byte_offset] |= value;
        offset += take;
    }
}

#[inline(always)]
fn is_negative(data: &[u8], start_bit: usize, num_bits: usize, packing: Packing) -> bool {
    let sign_bit_offset = match packing {
        Packing::SignedLittleEndian => start_bit + num_bits - 1,
        Packing::SignedBigEndian if num_bits >= 8 => start_bit + 7,
        Packing::SignedBigEndian => start_bit + num_bits - 1,
        _ => panic!("packing does not represent a signed value"),
    };
    data[sign_bit_offset / 8] & (1 << (sign_bit_offset % 8)) != 0
}

#[inline(always)]
fn pack_bits(from: &[u8], to: &mut [u8], start_bit: usize, num_bits: usize) {
    let start_byte = start_bit / 8;
    let end_bit = start_bit + num_bits;
    let num_bytes = match end_bit % 8 {
        0 => end_bit / 8,
        _ => end_bit / 8 + 1,
    };
    // eprintln!(
    //     "pack_bits; start_bit: {} end_bit: {} num_bits: {}",
    //     start_bit, end_bit, num_bits
    // );
    // eprintln!(
    //     "pack_bits; start_byte: {} num_bytes: {}",
    //     start_byte, num_bytes
    // );

    let shift = start_bit % 8;
    let upper_shift = 8 - shift;
    let mut upper_bits = 0;

    for byte_index in 0..num_bytes {
        let mask = match num_bits - byte_index * 8 {
            bits_left if bits_left >= 8 => 0xff,
            bits_left => 0xff >> (8 - bits_left),
        };
        // eprintln!("pack_bits; mask[{}]: {:02x}", byte_index, mask);
        let value = from[byte_index] & mask;
        // eprintln!("pack_bits; value[{}]: {:02x}", byte_index, value);
        let lower_bits = value << shift;
        to[start_byte + byte_index] |= lower_bits | upper_bits;
        upper_bits = value >> upper_shift;
    }
}
