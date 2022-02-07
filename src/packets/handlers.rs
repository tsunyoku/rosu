use crate::constants::Packets;
use crate::objects::user::{Action, User};
use crate::objects::mode::Mode;
use crate::objects::mods::Mods;
use crate::packets::reader::Reader;
use crate::packets::writer::PacketWriter;
use crate::players;

use tokio::sync::RwLockReadGuard;
use std::collections::HashMap;
use num_traits::FromPrimitive;
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
    writer += &user.friends;
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

#[inline(always)]
pub fn logout(user_id: i32) -> Vec<u8> {
    let mut writer = PacketWriter::new(Packets::CHO_USER_LOGOUT);

    writer += user_id;
    writer += 0 as u8; // logout timeout?

    return writer.serialize();
}

#[inline(always)]
pub fn spectator_joined(user_id: i32) -> Vec<u8> {
    let mut writer = PacketWriter::new(Packets::CHO_FELLOW_SPECTATOR_JOINED);
    writer += user_id;
    return writer.serialize();
}

#[inline(always)]
pub fn host_spectator_joined(user_id: i32) -> Vec<u8> {
    let mut writer = PacketWriter::new(Packets::CHO_SPECTATOR_JOINED);
    writer += user_id;
    return writer.serialize();
}

#[inline(always)]
pub fn spectator_left(user_id: i32) -> Vec<u8> {
    let mut writer = PacketWriter::new(Packets::CHO_FELLOW_SPECTATOR_LEFT);
    writer += user_id;
    return writer.serialize();
}

#[inline(always)]
pub fn host_spectator_left(user_id: i32) -> Vec<u8> {
    let mut writer = PacketWriter::new(Packets::CHO_SPECTATOR_LEFT);
    writer += user_id;
    return writer.serialize();
}

#[inline(always)]
pub fn spectate_frames(frames: Vec<u8>) -> Vec<u8> {
    let mut writer = PacketWriter::new(Packets::OSU_SPECTATE_FRAMES);
    writer += frames;
    return writer.serialize();
}

#[inline(always)]
pub fn channel_message(src_name: String, src_id: i32, content: String, target_name: String) -> Vec<u8> {
    let mut writer = PacketWriter::new(Packets::CHO_SEND_MESSAGE);
    writer += src_name;
    writer += content;
    writer += target_name;
    writer += src_id;
    return writer.serialize(); // Ew american.
}

pub type HandlerHashMap = HashMap<
            Packets,
            for<'lt> fn(
                user: &'lt mut User,
                reader: &'lt mut Reader,
            ) -> BoxFuture<'lt, bool>>;

macro_rules! register_packets {(
    $(
        #[packet($id:path, $res:expr $(,)?)]
     $( #[$attr:meta] )*
        $pub:vis
        async
        fn $fname:ident ($user:ident : & $('_)? mut User, $reader:ident : & $('_)? mut Reader) -> bool
        $body:block
    )*
) => (
    $(
     $( #[$attr] )*
        $pub
        fn $fname<'lt> (
            $user : &'lt mut User,
            $reader : &'lt mut Reader,
        ) -> BoxFuture<'lt, bool>
        {
            return FutureExt::boxed(async move {
                let _ = (&$user, &$reader);
                $body
            })
        }
    )*

    lazy_static::lazy_static! {
        pub static ref PACKET_HANDLERS: HandlerHashMap = {
            let mut map = HashMap::new();
            $( map.insert($id, $fname as _); )*
            map
        };

        pub static ref RESTRICTED_PACKET_HANDLERS: HandlerHashMap = {
            let mut map = HashMap::new();
            $( 
                if $res {
                    map.insert($id, $fname as _);
                }
            )*
            map
        };
    }
)}

// read handlers
register_packets! {
    // format for attribute: #[packet(packet_enum, allowed while restricted)]
    // each function returns a bool of whether or not the reader buffer should be incremented

    #[packet(Packets::OSU_PING, true)]
    #[inline(always)]
    pub async fn ping(user: &mut User, reader: &mut Reader) -> bool {
        let mut writer = PacketWriter::new(Packets::CHO_PONG);
        let pong = writer.serialize();

        user.enqueue(pong).await;
        return true;
    }

    #[packet(Packets::OSU_REQUEST_STATUS_UPDATE, true)]
    #[inline(always)]
    pub async fn status_update(user: &mut User, reader: &mut Reader) -> bool {
        user.enqueue(user_stats(user)).await;
        return true;
    }

    #[packet(Packets::OSU_USER_STATS_REQUEST, true)]
    #[inline(always)]
    pub async fn stats_request(user: &mut User, reader: &mut Reader) -> bool {
        let user_ids = reader.read_i32_list();

        for uid in user_ids {
            match players.get_id(uid).await {
                Some(u) => {
                    if !u.read().await.restricted() {
                        user.enqueue(user_presence(user)).await;
                    }
                },
                _ => (),
            }
        }

        return false;
    }

    #[packet(Packets::OSU_USER_PRESENCE_REQUEST, true)]
    #[inline(always)]
    pub async fn presence_request(user: &mut User, reader: &mut Reader) -> bool {
        let user_ids = reader.read_i32_list();

        for uid in user_ids {
            match players.get_id(uid).await {
                Some(u) => {
                    let _user: &RwLockReadGuard<'_, User> = &u.read().await;
                    user.enqueue(user_presence(_user)).await;
                },
                _ => (),
            }
        }

        return false;
    }

    #[packet(Packets::OSU_USER_PRESENCE_REQUEST_ALL, true)]
    #[inline(always)]
    pub async fn full_presence(user: &mut User, reader: &mut Reader) -> bool {
        for u in players.players.lock().await.values() {
            let _user = &u.read().await;

            if !_user.restricted() {
                user.enqueue(user_presence(_user)).await;
            }
        }

        return true;
    }

    #[packet(Packets::OSU_FRIEND_ADD, true)]
    #[inline(always)]
    pub async fn add_friend(user: &mut User, reader: &mut Reader) -> bool {
        let target: i32 = reader.read_int();

        if user.friends.contains(&target) {
            return false;
        }

        user.add_friend(target).await;

        return false;
    }

    #[packet(Packets::OSU_FRIEND_REMOVE, true)]
    #[inline(always)]
    pub async fn remove_friend(user: &mut User, reader: &mut Reader) -> bool {
        let target: i32 = reader.read_int();

        if !user.friends.contains(&target) {
            return false;
        }

        user.remove_friend(target).await;

        return false;
    }

    #[packet(Packets::OSU_LOGOUT, true)]
    #[inline(always)]
    pub async fn user_logout(user: &mut User, reader: &mut Reader) -> bool {
        user.logout().await;

        println!("{} logged out", user.username);

        return true;
    }

    #[packet(Packets::OSU_CHANGE_ACTION, true)]
    #[inline(always)]
    pub async fn change_action(user: &mut User, reader: &mut Reader) -> bool {
        let action_id: u8 = reader.read_int();
        let action_info: String = reader.read_str();
        let map_md5: String = reader.read_str();
        let mods: u32 = reader.read_int();
        let mode: u8 = reader.read_int();
        let map_id: i32 = reader.read_int();

        user.action = Action::from_u8(action_id).unwrap();
        user.info_text = action_info;
        user.map_md5 = map_md5;
        user.mods = Mods::from_value(mods as i32);
        user.current_mode = Mode::from_mods(mode as i32, mods as i32);
        user.map_id = map_id;

        if !user.restricted() {
            players.enqueue(user_stats(user)).await;
        }

        return false;
    }

    #[packet(Packets::OSU_START_SPECTATING, false)]
    #[inline(always)]
    pub async fn start_spectating(user: &mut User, reader: &mut Reader) -> bool {
        let target: i32 = reader.read_int();

        if target == 999 || target == 1 { // ignore the bot
            return false;
        }

        let u = players.get_id(target).await.unwrap();
        let mut _user = u.write().await;
        _user.add_spectator(user).await;

        return false;
    }

    #[packet(Packets::OSU_STOP_SPECTATING, false)]
    #[inline(always)]
    pub async fn stop_spectating(user: &mut User, reader: &mut Reader) -> bool  {
        if user.spectating == None {
            return true;
        }

        let u = players.get_id(user.spectating.unwrap()).await.unwrap();
        let mut _user = u.write().await;
        _user.remove_spectator(user).await;

        return true;
    }

    #[packet(Packets::OSU_SPECTATE_FRAMES, false)]
    #[inline(always)]
    pub async fn user_spectate_frames(user: &mut User, reader: &mut Reader) -> bool {
        let frames = reader.read_raw();

        let frames_packet = spectate_frames(frames);
        for u in players.players.lock().await.values() {
            let _user = &u.read().await;

            _user.enqueue(frames_packet.clone()).await;
        }
        
        return false;
    }

    // TODO: channels & msgs (left as realistik wants to do them)
    // TODO: multiplayer (killing myself)
}
