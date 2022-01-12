use serde::{Serialize, Serializer};

macro_rules! pub_struct { // w.
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Clone)]
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

pub_struct!(User {
    id: i32,
    osuver: String,
    username: String,
    username_safe: String,
    ban_datetime: i32,
    password_md5: String,
    salt: String, // unused
    email: String,
    register_datetime: i32,
    rank: i8,
    allowed: i8,
    latest_activity: i32,
    silence_end: i32,
    silence_reason: String,
    password_version: i8, // unused
    privileges: i64, // TODO: privilege enum
    donor_expire: i32,
    flags: i32,
    achievements_version: i32, // unused
    achievements_0: i32, // unused?
    achievements_1: i32, // unused?
    notes: String,

    // wow i hate my old self for making freeze like this lol
    frozen: i32,
    freezedate: i32,
    firstloginafterfrozen: i32,

    bypass_hwid: i8,
    ban_reason: String,

    // set upon login, not from db.
    utc_offset: i32,
    country: String,
    geoloc: u8,
    bancho_priv: u8, // TODO: bancho privilege enum
    long: f32,
    lat: f32,

    // status stuff
    action: Action,
    info_text: String,
    map_md5: String,
    mods: i32, // TODO: enum
    current_mode: u8, // TODO: mode enum/struct/class/idk
    map_id: i32,

    token: String, // rando token
});

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[repr(u8)]
pub enum Action {
    Idle = 0,
    Afk = 1,
    Playing = 2,
    Editing = 3,
    Modding = 4,
    Multiplayer = 5,
    Watching = 6,
    Unknown = 7,
    Testing = 8,
    Submitting = 9,
    Paused = 10,
    Lobby = 11,
    Multiplaying = 12,
    OsuDirect = 13,
}

impl Serialize for Action {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}