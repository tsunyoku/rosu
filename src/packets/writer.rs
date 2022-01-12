extern crate alloc;

use byteorder::LittleEndian;
use alloc::string::String;
use alloc::vec::Vec;
use bincode::Infinite;
use serde::Serialize;

use crate::packets::packets::Packets;
use crate::structs;

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

        data_bytes.append(&mut pack(&(int_list.len() as u16)));
        for data_elem in int_list {
            data_bytes.append(&mut write_raw(data_elem));
        }
    } else {
        data_bytes = pack(&data);
    }

    return data_bytes;
}

#[inline(always)]
pub fn write<T: Into<Option<T>> + Serialize>(packet: Packets, _data: T) -> Vec<u8>{
    let mut bytes = Vec::new();

    bytes.append(&mut pack(&(packet as i16)));
    bytes.push(0);

    let mut data_bytes: Vec<u8> = Vec::new();

    if std::any::type_name::<T>() != "core::option::Option<()>" {
        if let Some(data) = _data.into() {
            data_bytes = write_raw(data);
        }
    }

    let mut data_len = pack(&(data_bytes.len() as u32));
    bytes.append(&mut data_len);

    if !data_bytes.is_empty() { // some packets don't have data, as we can see by Option argument.
        bytes.append(&mut data_bytes);
    }

    return bytes;
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

// writer will be good enough to handle lists one day.

#[inline(always)]
pub fn user_presence(user: &structs::User) -> Vec<u8> {
    let mut packet_vec = Vec::new();

    // manual construction :(
    packet_vec.append(
        &mut pack(&(Packets::CHO_USER_PRESENCE as i16))
    );
    packet_vec.push(0);

    // overall data is added here
    let mut data_vec: Vec<u8> = Vec::new();

    data_vec.append(
        &mut write_raw(&user.id)
    );

    data_vec.append(
        &mut write_raw(&user.username)
    );

    data_vec.append(
        &mut write_raw(&user.utc_offset + 24)
    );

    data_vec.append(
        &mut write_raw(&user.geoloc)
    );

    data_vec.append(
        &mut write_raw(&user.bancho_priv | (user.current_mode << 5 ))
    );

    data_vec.append(
        &mut write_raw(&user.long)
    );

    data_vec.append(
        &mut write_raw(&user.lat)
    );

    data_vec.append(
        &mut write_raw(0 as i32) // user rank (hardcode for now)
    );

    // get overall data length + add it to total packet
    let mut data_len = pack(&(data_vec.len() as u32));
    packet_vec.append(&mut data_len);
    packet_vec.append(&mut data_vec);

    return packet_vec;
}

#[inline(always)]
pub fn user_stats(user: &structs::User) -> Vec<u8> {
    let mut packet_vec: Vec<u8> = Vec::new();

    // manual construction :(
    packet_vec.append(
        &mut pack(&(Packets::CHO_USER_STATS as i16))
    );
    packet_vec.push(0);

    // overall data is added here
    let mut data_vec: Vec<u8> = Vec::new();

    data_vec.append(
        &mut write_raw(&user.id)
    );

    data_vec.append(
        &mut write_raw(user.action as u8)
    );

    data_vec.append(
        &mut write_raw(&user.info_text)
    );
    
    data_vec.append(
        &mut write_raw(&user.map_md5)
    );

    data_vec.append(
        &mut write_raw(&user.mods)
    );
    
    data_vec.append(
        &mut write_raw(&user.current_mode)
    );

    data_vec.append(
        &mut write_raw(&user.map_id)
    );

    // hardcoded stats for now!

    data_vec.append(
        &mut write_raw(0 as i64) // ranked score
    );

    data_vec.append(
        &mut write_raw(0.0 as f32) // accuracy
    );

    data_vec.append(
        &mut write_raw(0 as i32) // playcount
    );
    
    data_vec.append(
        &mut write_raw(0 as i64) // total score
    );

    data_vec.append(
        &mut write_raw(0 as i32) // global rank
    );
    
    data_vec.append(
        &mut write_raw(0 as i16) // pp
    );

    // get overall data length + add it to total packet
    let mut data_len = pack(&(data_vec.len() as u32));
    packet_vec.append(&mut data_len);
    packet_vec.append(&mut data_vec);

    return packet_vec;
}