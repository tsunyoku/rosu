extern crate alloc;

use byteorder::LittleEndian;
use alloc::string::String;
use std::ops::{Add, AddAssign};
use alloc::vec::Vec;
use bincode::Infinite;
use serde::Serialize;

use crate::constants::packets::Packets;

#[inline(always)]
pub fn pack<T: Serialize>(data: &T) -> Vec<u8> {
    return bincode::endian_choice::serialize::<_, _, LittleEndian>(data, Infinite).unwrap();
}

#[inline(always)]
pub fn write_raw<T: Serialize>(data: T) -> Vec<u8> {
    let mut data_bytes: Vec<u8> = Vec::new();

    if std::any::type_name::<T>() == "&alloc::string::String" {
        let packet_string = unsafe {
            std::mem::transmute_copy::<T, &alloc::string::String>(&data)
        };

        data_bytes = write_osu_string(packet_string.to_string());
    } else if std::any::type_name::<T>() == "&str" {
        let packet_str = unsafe {
            std::mem::transmute_copy::<T, &str>(&data)
        };

        data_bytes = write_osu_string(packet_str.to_string());
    } else if std::any::type_name::<T>() == "&alloc::vec::Vec<i32>" {
        let int_list = unsafe {
            std::mem::transmute_copy::<T, &alloc::vec::Vec<i32>>(&data)
        };

        data_bytes.extend(pack(&(int_list.len() as u16)));
        for data_elem in int_list {
            data_bytes.extend(write_raw(data_elem));
        }
    } else if std::any::type_name::<T>() != "core::option::Option<()>" {
        data_bytes = pack(&data);
    }

    return data_bytes;
}

#[inline(always)]
pub fn write_uleb128(_value: i32) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut value = _value;

    loop {
        let byte = value & 0x7f;
        value >>= 7;
        if value != 0 {
            bytes.push((byte | 0x80) as u8);
        } else {
            bytes.push(byte as u8);
            break;
        }
    }

    return bytes;
}

#[inline(always)]
pub fn write_osu_string(_value: String) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    let value = _value.as_bytes().to_vec();

    if value.is_empty() {
        bytes.push(0);
    } else {
        bytes.push(11); // 0x0B
        bytes.extend(write_uleb128(value.len() as i32));
        bytes.extend(value);
    }

    return bytes;
}

#[derive(Clone, Debug, PartialEq)]
pub struct PacketWriter {
    packet: Packets,
    data: Vec<u8> // we barely actually need any attributes, we just like the functions.
}

impl PacketWriter {
    pub fn new(packet: Packets) -> Self {
        return Self {
            packet: packet,
            data: Vec::new()
        }
    }

    pub fn write<T: Serialize>(&mut self, packet_data: T) {
        self.data.extend(write_raw(packet_data));
    }

    #[inline(always)]
    pub fn serialise(&mut self) -> Vec<u8> {
        let mut return_data: Vec<u8> = Vec::new();

        // first add packet id
        return_data.extend(write_raw(self.packet));

        return_data.push(0); // just osu things.

        // now calculate our data length, and follow regular packet structure.
        let data_len = pack(&(self.data.len() as u32));
        return_data.extend(data_len);
        return_data.append(&mut self.data);

        return return_data;
    }
}

macro_rules! packet_impl {
    ($name: ident) => {
        impl Add<$name> for PacketWriter {
            type Output = PacketWriter;

            fn add(mut self, data: $name) -> PacketWriter {
                self.write(data);

                return self;
            }
        }

        impl AddAssign<$name> for PacketWriter {
            fn add_assign(&mut self, data: $name) {
                self.write(data);
            }
        }
    }
}

packet_impl!(u8);
packet_impl!(i16);
packet_impl!(i32);
packet_impl!(f32);
packet_impl!(i64);
packet_impl!(String);

// these ones couldn't be handled by macro :(

impl Add<Vec<u8>> for PacketWriter {
    type Output = PacketWriter;

    fn add(mut self, data: Vec<u8>) -> PacketWriter {
        self.data.extend(data);

        return self;
    }
}

impl AddAssign<Vec<u8>> for PacketWriter {
    fn add_assign(&mut self, data: Vec<u8>) {
        self.data.extend(data);
    }
}

impl Add<&Vec<i32>> for PacketWriter {
    type Output = PacketWriter;

    fn add(mut self, data: &Vec<i32>) -> PacketWriter {
        self.write(data);

        return self;
    }
}

impl AddAssign<&Vec<i32>> for PacketWriter {
    fn add_assign(&mut self, data: &Vec<i32>) {
        self.write(data);
    }
}

impl Add<&str> for PacketWriter {
       type Output = PacketWriter;

    fn add(mut self, data: &str) -> PacketWriter {
        self.write(data);

        return self;
    }
}

impl AddAssign<&str> for PacketWriter {
    fn add_assign(&mut self, data: &str) {
        self.write(data);
    }
}

impl Add<&String> for PacketWriter {
    type Output = PacketWriter;

    fn add(mut self, data: &String) -> PacketWriter {
        self.write(data);

        return self;
    }
}

impl AddAssign<&String> for PacketWriter {
    fn add_assign(&mut self, data: &String) {
        self.write(data);
    }
}

impl Add<PacketWriter> for PacketWriter {
    type Output = PacketWriter;

    fn add(mut self, writer: PacketWriter) -> PacketWriter {
        self += writer.data;

        return self;
    }
}

impl AddAssign<PacketWriter> for PacketWriter {
    fn add_assign(&mut self, writer: PacketWriter) {
        self.data.extend(writer.data);
    }
}