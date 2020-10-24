use super::{pack_bits, unpack_bits, Pack, Packing};

impl Pack for u8 {
    fn pack_le_bits(&self, into: &mut [u8], start_bit: usize, num_bits: usize) {
        pack_bits(&[*self], into, start_bit, num_bits);
    }

    fn pack_be_bits(&self, into: &mut [u8], start_bit: usize, num_bits: usize) {
        self.pack_le_bits(into, start_bit, num_bits);
    }

    fn unpack_le_bits(from: &[u8], start_bit: usize, num_bits: usize) -> Self {
        let mut buf = [0u8; 1];
        unpack_bits(
            from,
            &mut buf,
            start_bit,
            num_bits,
            Packing::UnsignedLittleEndian,
        );
        buf[0] as Self
    }

    fn unpack_be_bits(from: &[u8], start_bit: usize, num_bits: usize) -> Self {
        Self::unpack_le_bits(from, start_bit, num_bits)
    }
}

impl Pack for i8 {
    fn pack_le_bits(&self, into: &mut [u8], start_bit: usize, num_bits: usize) {
        pack_bits(&[*self as u8], into, start_bit, num_bits);
    }

    fn pack_be_bits(&self, into: &mut [u8], start_bit: usize, num_bits: usize) {
        self.pack_le_bits(into, start_bit, num_bits);
    }

    fn unpack_le_bits(from: &[u8], start_bit: usize, num_bits: usize) -> Self {
        let mut buf = [0u8; 1];
        unpack_bits(
            from,
            &mut buf,
            start_bit,
            num_bits,
            Packing::UnsignedLittleEndian,
        );
        buf[0] as Self
    }

    fn unpack_be_bits(from: &[u8], start_bit: usize, num_bits: usize) -> Self {
        Self::unpack_le_bits(from, start_bit, num_bits)
    }
}

impl Pack for u16 {
    fn pack_le_bits(&self, into: &mut [u8], start_bit: usize, num_bits: usize) {
        let buf = self.to_le_bytes();
        pack_bits(&buf, into, start_bit, num_bits);
    }

    fn pack_be_bits(&self, into: &mut [u8], start_bit: usize, num_bits: usize) {
        let buf = self.to_be_bytes();
        pack_bits(&buf, into, start_bit, num_bits);
    }

    fn unpack_le_bits(from: &[u8], start_bit: usize, num_bits: usize) -> Self {
        let mut buf = [0u8; 2];
        unpack_bits(
            from,
            &mut buf,
            start_bit,
            num_bits,
            Packing::UnsignedLittleEndian,
        );
        Self::from_le_bytes([buf[0], buf[1]])
    }

    fn unpack_be_bits(from: &[u8], start_bit: usize, num_bits: usize) -> Self {
        let mut buf = [0u8; 2];
        unpack_bits(
            from,
            &mut buf,
            start_bit,
            num_bits,
            Packing::UnsignedBigEndian,
        );
        Self::from_be_bytes([buf[0], buf[1]])
    }
}

impl Pack for i16 {
    fn pack_le_bits(&self, into: &mut [u8], start_bit: usize, num_bits: usize) {
        let buf = self.to_le_bytes();
        pack_bits(&buf, into, start_bit, num_bits);
    }

    fn pack_be_bits(&self, into: &mut [u8], start_bit: usize, num_bits: usize) {
        let buf = self.to_be_bytes();
        pack_bits(&buf, into, start_bit, num_bits);
    }

    fn unpack_le_bits(from: &[u8], start_bit: usize, num_bits: usize) -> Self {
        let mut buf = [0u8; 2];
        unpack_bits(
            from,
            &mut buf,
            start_bit,
            num_bits,
            Packing::SignedLittleEndian,
        );
        Self::from_le_bytes([buf[0], buf[1]])
    }

    fn unpack_be_bits(from: &[u8], start_bit: usize, num_bits: usize) -> Self {
        let mut buf = [0u8; 2];
        unpack_bits(
            from,
            &mut buf,
            start_bit,
            num_bits,
            Packing::SignedBigEndian,
        );
        Self::from_be_bytes([buf[0], buf[1]])
    }
}

impl Pack for u32 {
    fn pack_le_bits(&self, into: &mut [u8], start_bit: usize, num_bits: usize) {
        let buf = self.to_le_bytes();
        pack_bits(&buf, into, start_bit, num_bits);
    }

    fn pack_be_bits(&self, into: &mut [u8], start_bit: usize, num_bits: usize) {
        let buf = self.to_be_bytes();
        pack_bits(&buf, into, start_bit, num_bits);
    }

    fn unpack_le_bits(from: &[u8], start_bit: usize, num_bits: usize) -> Self {
        let mut buf = [0u8; 4];
        unpack_bits(
            from,
            &mut buf,
            start_bit,
            num_bits,
            Packing::UnsignedLittleEndian,
        );
        Self::from_le_bytes([buf[0], buf[1], buf[2], buf[3]])
    }

    fn unpack_be_bits(from: &[u8], start_bit: usize, num_bits: usize) -> Self {
        let mut buf = [0u8; 4];
        unpack_bits(
            from,
            &mut buf,
            start_bit,
            num_bits,
            Packing::UnsignedBigEndian,
        );
        Self::from_be_bytes([buf[0], buf[1], buf[2], buf[3]])
    }
}

impl Pack for i32 {
    fn pack_le_bits(&self, into: &mut [u8], start_bit: usize, num_bits: usize) {
        let buf = self.to_le_bytes();
        pack_bits(&buf, into, start_bit, num_bits);
    }

    fn pack_be_bits(&self, into: &mut [u8], start_bit: usize, num_bits: usize) {
        let buf = self.to_be_bytes();
        pack_bits(&buf, into, start_bit, num_bits);
    }

    fn unpack_le_bits(from: &[u8], start_bit: usize, num_bits: usize) -> Self {
        let mut buf = [0u8; 4];
        unpack_bits(
            from,
            &mut buf,
            start_bit,
            num_bits,
            Packing::SignedLittleEndian,
        );
        Self::from_le_bytes([buf[0], buf[1], buf[2], buf[3]])
    }

    fn unpack_be_bits(from: &[u8], start_bit: usize, num_bits: usize) -> Self {
        let mut buf = [0u8; 4];
        unpack_bits(
            from,
            &mut buf,
            start_bit,
            num_bits,
            Packing::SignedBigEndian,
        );
        Self::from_be_bytes([buf[0], buf[1], buf[2], buf[3]])
    }
}

impl Pack for u64 {
    fn pack_le_bits(&self, into: &mut [u8], start_bit: usize, num_bits: usize) {
        let buf = self.to_le_bytes();
        pack_bits(&buf, into, start_bit, num_bits);
    }

    fn pack_be_bits(&self, into: &mut [u8], start_bit: usize, num_bits: usize) {
        let buf = self.to_be_bytes();
        pack_bits(&buf, into, start_bit, num_bits);
    }

    fn unpack_le_bits(from: &[u8], start_bit: usize, num_bits: usize) -> Self {
        let mut buf = [0u8; 8];
        unpack_bits(
            from,
            &mut buf,
            start_bit,
            num_bits,
            Packing::UnsignedLittleEndian,
        );
        Self::from_le_bytes([
            buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
        ])
    }

    fn unpack_be_bits(from: &[u8], start_bit: usize, num_bits: usize) -> Self {
        let mut buf = [0u8; 8];
        unpack_bits(
            from,
            &mut buf,
            start_bit,
            num_bits,
            Packing::UnsignedBigEndian,
        );
        Self::from_be_bytes([
            buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
        ])
    }
}

impl Pack for i64 {
    fn pack_le_bits(&self, into: &mut [u8], start_bit: usize, num_bits: usize) {
        let buf = self.to_le_bytes();
        pack_bits(&buf, into, start_bit, num_bits);
    }

    fn pack_be_bits(&self, into: &mut [u8], start_bit: usize, num_bits: usize) {
        let buf = self.to_be_bytes();
        pack_bits(&buf, into, start_bit, num_bits);
    }

    fn unpack_le_bits(from: &[u8], start_bit: usize, num_bits: usize) -> Self {
        let mut buf = [0u8; 8];
        unpack_bits(
            from,
            &mut buf,
            start_bit,
            num_bits,
            Packing::SignedLittleEndian,
        );
        Self::from_le_bytes([
            buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
        ])
    }

    fn unpack_be_bits(from: &[u8], start_bit: usize, num_bits: usize) -> Self {
        let mut buf = [0u8; 8];
        unpack_bits(
            from,
            &mut buf,
            start_bit,
            num_bits,
            Packing::SignedBigEndian,
        );
        Self::from_be_bytes([
            buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
        ])
    }
}
