#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum Packing {
    SignedLittleEndian,
    SignedBigEndian,
    UnsignedLittleEndian,
    UnsignedBigEndian,
}
