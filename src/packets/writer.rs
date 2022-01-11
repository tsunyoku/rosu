use byteorder::LittleEndian;
use bincode::Infinite;
use serde::Serialize;

use crate::packets::packets::Packets;

pub fn pack<T: Serialize>(data: &T) -> Vec<u8> {
    return bincode::endian_choice::serialize::<_, _, LittleEndian>(data, Infinite).unwrap();
}

#[inline(always)]
pub fn write<T: Serialize>(packet: Packets, data: T) -> Vec<u8>{
    let mut bytes = Vec::new();

    bytes.append(&mut pack(&(packet as i16)));
    bytes.push(0);

    let mut data_bytes: Vec<u8>;

    if std::any::type_name::<T>() == "String" {
        let packet_string = unsafe {
            std::mem::transmute_copy::<T, String>(&data)
        };

        data_bytes = write_osu_string(packet_string);
    } else if std::any::type_name::<T>() == "&str" {
        let packet_str = unsafe {
            std::mem::transmute_copy::<T, &str>(&data)
        };

        data_bytes = write_osu_string(packet_str.to_string());
    } else {
        data_bytes = pack(&data);
    }

    let mut data_len = pack(&(data_bytes.len() as u32));

    bytes.append(&mut data_len);
    bytes.append(&mut data_bytes);

    return bytes;
}

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

pub fn write_osu_string(_value: String) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut value = _value.as_bytes().to_vec();

    if value.is_empty() {
        bytes.push(0);
    } else {
        bytes.push(11); // 0x0B
        bytes.append(&mut write_uleb128(value.len() as i32));
        bytes.append(&mut value);
    }

    return bytes;
}