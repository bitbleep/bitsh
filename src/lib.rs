pub mod impls;
#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone)]
pub enum Packing {
    SignedLittleEndian,
    SignedBigEndian,
    UnsignedLittleEndian,
    UnsignedBigEndian,
}

pub trait Pack {
    fn pack_le_bits(&self, into: &mut [u8], start_bit: usize, num_bits: usize);
    fn pack_be_bits(&self, into: &mut [u8], start_bit: usize, num_bits: usize);
    fn unpack_le_bits(from: &[u8], start_bit: usize, num_bits: usize) -> Self;
    fn unpack_be_bits(from: &[u8], start_bit: usize, num_bits: usize) -> Self;
}

#[inline(always)]
fn pack_bits(from: &[u8], to: &mut [u8], start_bit: usize, num_bits: usize) {
    let mut from_bit: usize = 0;
    while from_bit < num_bits {
        let from_shift = from_bit % 8;
        let to_bit = from_bit + start_bit;
        let to_shift = to_bit % 8;
        let available = 8 - from_shift;
        let space = 8 - to_shift;
        let take = if space < available { space } else { available };
        let copied_bits = from[from_bit / 8] >> from_shift;
        to[to_bit / 8] |= copied_bits << (to_bit % 8);
        from_bit += take;
    }
}

#[inline(always)]
fn unpack_bits(from: &[u8], to: &mut [u8], start_bit: usize, num_bits: usize, packing: Packing) {
    let init_value = match packing {
        Packing::SignedLittleEndian | Packing::SignedBigEndian => {
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
    let mut to_bit: usize = 0;
    while to_bit < num_bits {
        let to_shift = to_bit % 8;
        let from_bit = to_bit + start_bit;
        let from_shift = from_bit % 8;
        let available = 8 - from_shift;
        let space = num_bits - to_bit;
        let take = if space < available { space } else { available };
        let copied_bits = from[from_bit / 8] >> from_shift;
        let mut mask = 0xff_u8;
        for i in to_shift..to_shift + take {
            mask ^= 1 << i;
        }
        let to_byte = to_bit / 8;
        to[to_byte] &= mask;
        to[to_byte] |= (copied_bits << to_shift) & !mask;
        to_bit += take;
    }
}

#[inline(always)]
fn is_negative(data: &[u8], start_bit: usize, num_bits: usize, packing: Packing) -> bool {
    let sign_bit_offset = match packing {
        Packing::SignedLittleEndian => start_bit + num_bits - 1,
        Packing::SignedBigEndian if num_bits >= 8 => start_bit + 7,
        Packing::SignedBigEndian => start_bit + num_bits - 1,
        _ => return false,
    };
    data[sign_bit_offset / 8] & (1 << (sign_bit_offset % 8)) != 0
}
