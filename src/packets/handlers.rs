use crate::structs::{User, Packets};
use crate::packets::writer::PacketWriter;

#[inline(always)]
pub fn user_id(user_id: i32) -> Vec<u8> {
    let mut writer = PacketWriter::new(Packets::CHO_USER_ID);
    writer += user_id;
    return writer.serialize();
}

#[inline(always)]
pub fn notification(notification: &str) -> Vec<u8> {
    let mut writer = PacketWriter::new(Packets::CHO_NOTIFICATION);
    writer += notification;
    return writer.serialize();
}

#[inline(always)]
pub fn protocol_version(version: i32) -> Vec<u8> {
    let mut writer = PacketWriter::new(Packets::CHO_PROTOCOL_VERSION);
    writer += version;
    return writer.serialize();
}

#[inline(always)]
pub fn bancho_privileges(privs: i32) -> Vec<u8> {
    let mut writer = PacketWriter::new(Packets::CHO_PRIVILEGES);
    writer += privs;
    return writer.serialize();
}

#[inline(always)]
pub fn channel_info_end() -> Vec<u8> { // lol this is so stupid
    let mut writer = PacketWriter::new(Packets::CHO_CHANNEL_INFO_END);
    return writer.serialize();
}

#[inline(always)]
pub fn main_menu_icon(icon: &str, link: &str) -> Vec<u8> {
    let mut writer = PacketWriter::new(Packets::CHO_MAIN_MENU_ICON);
    writer += format!("{}|{}", icon, link).as_str();
    return writer.serialize();
}

#[inline(always)]
pub fn friends_list(user: &User) -> Vec<u8> {
    let mut writer = PacketWriter::new(Packets::CHO_FRIENDS_LIST);
    
    // fake list for now
    let friends_list = vec![user.id];
    writer += &friends_list;

    return writer.serialize();
}

#[inline(always)]
pub fn silence_end(silence_end: i32) -> Vec<u8> {
    let mut writer = PacketWriter::new(Packets::CHO_SILENCE_END);
    writer += silence_end;
    return writer.serialize();
}

#[inline(always)]
pub fn user_presence(user: &User) -> Vec<u8> {
    let mut writer = PacketWriter::new(Packets::CHO_USER_PRESENCE);

    writer += user.id;
    writer += &user.username;
    writer += user.utc_offset + 24;
    writer += user.geoloc;
    writer += user.bancho_priv | (user.current_mode << 5);
    writer += user.long;
    writer += user.lat;
    writer += 0 as i32; // user rank (hardcode for now)

    return writer.serialize();
}

#[inline(always)]
pub fn user_stats(user: &User) -> Vec<u8> {
    let mut writer = PacketWriter::new(Packets::CHO_USER_STATS);

    writer += user.id;
    writer += user.action as u8;
    writer += &user.info_text;
    writer += &user.map_md5;
    writer += user.mods;
    writer += user.current_mode;
    writer += user.map_id;

    // hardcoded stats for now!
    writer += 0 as i64; // ranked score
    writer += 0.0 as f32; // accuracy
    writer += 0 as i32; // playcount
    writer += 0 as i64; // total score
    writer += 0 as i32; // global rank
    writer += 0 as i16; // pp

    return writer.serialize();
}