use crate::constants::Packets;
use crate::objects::user::User;
use crate::packets::reader::Reader;
use crate::packets::writer::PacketWriter;

use tokio::sync::RwLockReadGuard;
use std::collections::HashMap;
use futures::future::{BoxFuture, FutureExt};

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
pub fn channel_info_end() -> Vec<u8> {
    // lol this is so stupid
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
    writer += (user.utc_offset + 24) as u8;
    writer += user.geoloc;
    writer += user.bancho_priv.value() as u8 | ((user.current_mode as u8) << 5);
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
    writer += user.mods.bits() as i32;
    writer += user.current_mode as u8;
    writer += user.map_id;

    let stats = &user.stats[user.current_mode as usize];

    writer += stats.ranked_score as i64;
    writer += stats.accuracy / 100.0 as f32;
    writer += stats.playcount as i32;
    writer += stats.total_score as i64;
    writer += 0 as i32; // global rank
    writer += stats.pp as i16;

    return writer.serialize();
}

#[inline(always)]
pub fn server_restart(time: i32) -> Vec<u8> {
    let mut writer = PacketWriter::new(Packets::CHO_RESTART);
    writer += time;
    return writer.serialize();
}

macro_rules! register_packets {(
    $(
        #[packet($id:path $(,)?)]
     $( #[$attr:meta] )*
        $pub:vis
        async
        fn $fname:ident ($user:ident : & $('_)? RwLockReadGuard<$('_, )? User>, $reader:ident : & $('_)? Reader) $( -> () )?
        $body:block
    )*
) => (
    $(
     $( #[$attr] )*
        $pub
        fn $fname<'lt> (
            $user : &'lt RwLockReadGuard<'_, User>,
            $reader : &'lt Reader,
        ) -> BoxFuture<'lt, ()>
        {
            FutureExt::boxed(async move {
                let _ = (&$user, $reader);
                $body
            })
        }
    )*

    lazy_static::lazy_static! {
        pub static ref PACKET_HANDLERS: HashMap<
            Packets,
            for<'lt> fn(
                user: &'lt RwLockReadGuard<'_, User>,
                reader: &'lt Reader,
            ) -> BoxFuture<'lt, ()>,
        > = {
            let mut map = HashMap::new();
            $( map.insert($id, $fname as _); )*
            map
        };
    }
)}

// read handlers
register_packets! {
    #[packet(Packets::OSU_PING)]
    pub async fn ping(user: &RwLockReadGuard<'_, User>, reader: &Reader) {
        let mut writer = PacketWriter::new(Packets::CHO_PONG);
        let pong = writer.serialize();

        user.enqueue(pong).await;
    }
}
