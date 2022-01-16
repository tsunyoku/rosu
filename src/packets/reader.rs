use std::convert::TryInto;

pub struct Reader {
    buf: Vec<u8>,
    offset: usize,
}

impl Reader {
    pub fn new(packet: Vec<u8>) -> Self {
        Self {
            buf: packet,
            offset: 0,
        }
    }

    pub fn incr_offset(&mut self, amount: usize) {
        self.offset += amount;
    }

    /// Reads a primitive type `T` from the buffer.
    pub fn read_int<T: Readable>(&mut self) -> T {
        let value = T::from_le_bytes(&self.buf[self.offset..self.offset + T::SIZE]);
        self.incr_offset(T::SIZE);
        return value;
    }

    // Maybe this should be part of read_int. Would be easily doable.
    pub fn read_f32(&mut self) -> f32 {
        let val = f32::from_le_bytes(
            self.buf[self.offset..self.offset + 4]
                .try_into()
                .expect("Should never happen."),
        );
        self.incr_offset(4);
        val
    }

    /// Reads a 128bit unsigned LEB integer from the buffer.
    pub fn read_uleb128(&mut self) -> u32 {
        let mut shift = 0_u32;
        let mut val = 0_u32;

        loop {
            let cur_byte = self.read_int::<u8>() as u32;
            val |= (cur_byte & 0b01111111) << shift;

            if cur_byte & 0b10000000 == 0 {
                break;
            }

            shift += 7;
        }
        val
    }

    /// Reads an osu style string from the buffer.
    pub fn read_str(&mut self) -> String {
        // Check exists byte.
        if self.read_int::<u8>() != 0x0b {
            return String::new();
        }

        // read string len.
        let len = self.read_uleb128() as usize;
        let string = String::from_utf8(self.buf[self.offset..self.offset + len].into())
            .unwrap_or(String::new());
        self.incr_offset(len as usize);

        string
    }

    /// Reads a list of i32s precremented by an u16 specifying length.
    pub fn read_i32_list(&mut self) -> Vec<i32> {
        let len: u16 = self.read_int();

        if len == 0 {
            return Vec::new();
        }

        let mut l = Vec::with_capacity(len as usize);
        for _ in 0..(len as usize) {
            l.push(self.read_int());
        }

        l
    }

    pub fn read_header(&mut self) -> (i32, u32) {
        let packet_id: u16 = self.read_int();

        self.incr_offset(1); // padding byte

        let packet_len: u32 = self.read_int();

        return (packet_id as i32, packet_len);
    }

    pub fn empty(&self) -> bool {
        return self.buf.len() <= self.offset;
    }
}

// FOR GENERICS!
pub trait Readable {
    fn from_le_bytes(bytes: &[u8]) -> Self;
    const SIZE: usize;
}

// Testing macros go brrrr.
macro_rules! impl_readable {
    ($name: ident) => {
        impl Readable for $name {
            fn from_le_bytes(bytes: &[u8]) -> Self {
                Self::from_le_bytes(bytes.try_into().expect("Should never happen."))
            }

            const SIZE: usize = ($name::BITS / 8) as usize;
        }
    };
}

impl Readable for u8 {
    fn from_le_bytes(bytes: &[u8]) -> Self {
        bytes[0]
    }

    const SIZE: usize = 1;
}

impl_readable!(u16);
impl_readable!(u32);
impl_readable!(u64);
impl_readable!(i8);
impl_readable!(i16);
impl_readable!(i32);
impl_readable!(i64);

/*
impl Readable for u16 {
    fn from_le_bytes(bytes: &[u8]) -> Self {
        u16::from_le_bytes(bytes.try_into().expect("Should never happen."))
    }

    const SIZE: usize = 2;
}
*/
