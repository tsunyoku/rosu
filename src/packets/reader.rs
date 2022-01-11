use std::convert::TryInto;

pub struct Reader {
    buf: Vec<u8>,
    offset: usize,
}

impl Reader {
    pub fn new(packet: Vec<u8>) -> Self {
        Self { buf: packet, offset: 0 }
    }

    fn incr_offset(&mut self, amount: usize) {
        self.offset += amount;
    }

    pub fn read_int<T: Readable>(&mut self) -> T {
        let value = T::from_le_bytes(&self.buf[self.offset..self.offset + T::SIZE]);
        self.incr_offset(T::SIZE);
        return value;
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
