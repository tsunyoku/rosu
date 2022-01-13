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
#[repr(i16)]
pub enum Packets {
    OSU_CHANGE_ACTION = 0,
    OSU_SEND_PUBLIC_MESSAGE = 1,
    OSU_LOGOUT = 2,
    OSU_REQUEST_STATUS_UPDATE = 3,
    OSU_PING = 4,
    CHO_USER_ID = 5,
    CHO_SEND_MESSAGE = 7,
    CHO_PONG = 8,
    CHO_HANDLE_IRC_CHANGE_USERNAME = 9,
    CHO_HANDLE_IRC_QUIT = 10,
    CHO_USER_STATS = 11,
    CHO_USER_LOGOUT = 12,
    CHO_SPECTATOR_JOINED = 13,
    CHO_SPECTATOR_LEFT = 14,
    CHO_SPECTATE_FRAMES = 15,
    OSU_START_SPECTATING = 16,
    OSU_STOP_SPECTATING = 17,
    OSU_SPECTATE_FRAMES = 18,
    CHO_VERSION_UPDATE = 19,
    OSU_ERROR_REPORT = 20,
    OSU_CANT_SPECTATE = 21,
    CHO_SPECTATOR_CANT_SPECTATE = 22,
    CHO_GET_ATTENTION = 23,
    CHO_NOTIFICATION = 24,
    OSU_SEND_PRIVATE_MESSAGE = 25,
    CHO_UPDATE_MATCH = 26,
    CHO_NEW_MATCH = 27,
    CHO_DISPOSE_MATCH = 28,
    OSU_PART_LOBBY = 29,
    OSU_JOIN_LOBBY = 30,
    OSU_CREATE_MATCH = 31,
    OSU_JOIN_MATCH = 32,
    OSU_PART_MATCH = 33,
    CHO_TOGGLE_BLOCK_NON_FRIEND_DMS = 34,
    CHO_MATCH_JOIN_SUCCESS = 36,
    CHO_MATCH_JOIN_FAIL = 37,
    OSU_MATCH_CHANGE_SLOT = 38,
    OSU_MATCH_READY = 39,
    OSU_MATCH_LOCK = 40,
    OSU_MATCH_CHANGE_SETTINGS = 41,
    CHO_FELLOW_SPECTATOR_JOINED = 42,
    CHO_FELLOW_SPECTATOR_LEFT = 43,
    OSU_MATCH_START = 44,
    CHO_ALL_PLAYERS_LOADED = 45,
    CHO_MATCH_START = 46,
    OSU_MATCH_SCORE_UPDATE = 47,
    CHO_MATCH_SCORE_UPDATE = 48,
    OSU_MATCH_COMPLETE = 49,
    CHO_MATCH_TRANSFER_HOST = 50,
    OSU_MATCH_CHANGE_MODS = 51,
    OSU_MATCH_LOAD_COMPLETE = 52,
    CHO_MATCH_ALL_PLAYERS_LOADED = 53,
    OSU_MATCH_NO_BEATMAP = 54,
    OSU_MATCH_NOT_READY = 55,
    OSU_MATCH_FAILED = 56,
    CHO_MATCH_PLAYER_FAILED = 57,
    CHO_MATCH_COMPLETE = 58,
    OSU_MATCH_HAS_BEATMAP = 59,
    OSU_MATCH_SKIP_REQUEST = 60,
    CHO_MATCH_SKIP = 61,
    CHO_UNAUTHORIZED = 62, // unused
    OSU_CHANNEL_JOIN = 63,
    CHO_CHANNEL_JOIN_SUCCESS = 64,
    CHO_CHANNEL_INFO = 65,
    CHO_CHANNEL_KICK = 66,
    CHO_CHANNEL_AUTO_JOIN = 67,
    OSU_BEATMAP_INFO_REQUEST = 68,
    CHO_BEATMAP_INFO_REPLY = 69,
    OSU_MATCH_TRANSFER_HOST = 70,
    CHO_PRIVILEGES = 71,
    CHO_FRIENDS_LIST = 72,
    OSU_FRIEND_ADD = 73,
    OSU_FRIEND_REMOVE = 74,
    CHO_PROTOCOL_VERSION = 75,
    CHO_MAIN_MENU_ICON = 76,
    OSU_MATCH_CHANGE_TEAM = 77,
    OSU_CHANNEL_PART = 78,
    OSU_RECEIVE_UPDATES = 79,
    CHO_MONITOR = 80, // unused
    CHO_MATCH_PLAYER_SKIPPED = 81,
    OSU_SET_AWAY_MESSAGE = 82,
    CHO_USER_PRESENCE = 83,
    OSU_IRC_ONLY = 84,
    OSU_USER_STATS_REQUEST = 85,
    CHO_RESTART = 86,
    OSU_MATCH_INVITE = 87,
    CHO_MATCH_INVITE = 88,
    CHO_CHANNEL_INFO_END = 89,
    OSU_MATCH_CHANGE_PASSWORD = 90,
    CHO_MATCH_CHANGE_PASSWORD = 91,
    CHO_SILENCE_END = 92,
    OSU_TOURNAMENT_MATCH_INFO_REQUEST = 93,
    CHO_USER_SILENCED = 94,
    CHO_USER_PRESENCE_SINGLE = 95,
    CHO_USER_PRESENCE_BUNDLE = 96,
    OSU_USER_PRESENCE_REQUEST = 97,
    OSU_USER_PRESENCE_REQUEST_ALL = 98,
    OSU_TOGGLE_BLOCK_NON_FRIEND_DMS = 99,
    CHO_USER_DM_BLOCKED = 100,
    CHO_TARGET_IS_SILENCED = 101,
    CHO_VERSION_UPDATE_FORCED = 102,
    CHO_SWITCH_SERVER = 103,
    CHO_ACCOUNT_RESTRICTED = 104,
    CHO_RTX = 105, // unused
    CHO_MATCH_ABORT = 106,
    CHO_SWITCH_TOURNAMENT_SERVER = 107,
    OSU_TOURNAMENT_JOIN_MATCH_CHANNEL = 108,
    OSU_TOURNAMENT_LEAVE_MATCH_CHANNEL = 109,
}

impl Serialize for Packets {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i16(*self as i16)
    }
}

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