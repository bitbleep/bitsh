#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum Packing {
    SignedLittleEndian,
    SignedBigEndian,
    UnsignedLittleEndian,
    UnsignedBigEndian,
}

// #[inline(always)]
// pub fn unpack_bits(from: &[u8], to: &mut [u8], start_bit: usize, num_bits: usize) {
//     unimplemented!();
// }

#[inline(always)]
pub fn pack_bits(from: &[u8], to: &mut [u8], start_bit: usize, num_bits: usize) {
    let start_byte = start_bit / 8;
    let end_bit = start_bit + num_bits;
    let num_bytes = match end_bit % 8 {
        0 => end_bit / 8,
        _ => end_bit / 8 + 1,
    };
    eprintln!(
        "pack_bits; start_bit: {} end_bit: {} num_bits: {}",
        start_bit, end_bit, num_bits
    );
    eprintln!(
        "pack_bits; start_byte: {} num_bytes: {}",
        start_byte, num_bytes
    );

    let shift = start_bit % 8;
    let upper_shift = 8 - shift;
    let mut upper_bits = 0;

    for byte_index in 0..num_bytes {
        let mask = match num_bits - byte_index * 8 {
            bits_left if bits_left >= 8 => 0xff,
            bits_left => 0xff >> (8 - bits_left),
        };
        eprintln!("pack_bits; mask[{}]: {:02x}", byte_index, mask);
        let value = from[byte_index] & mask;
        eprintln!("pack_bits; value[{}]: {:02x}", byte_index, value);
        let lower_bits = value << shift;
        let to_offset = start_byte + byte_index;
        to[to_offset] |= lower_bits | upper_bits;
        upper_bits = value >> upper_shift;
    }
}
