use crate::objects::mode::Mode;
use crate::objects::mods::Mods;
use crate::objects::privileges::{Privileges, BanchoPrivileges};
use uuid::Uuid;

use ntex::web;
use sqlx::{Pool, MySql};

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
    privileges: Privileges,
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
    bancho_priv: BanchoPrivileges,
    long: f32,
    lat: f32,

    // status stuff
    action: Action,
    info_text: String,
    map_md5: String,
    mods: Mods,
    current_mode: Mode,
    map_id: i32,

    token: String, // rando token
});

type DBPool = web::types::Data<Pool<MySql>>;

impl User { // perhaps the worst part of this entire code rn
    pub async fn from_sql(username: &str, token: Uuid, osu_ver: &str, pool: DBPool) -> Option<Self> {
        let user_row_result = sqlx::query!("select * from users where username_safe = ?", username.to_lowercase().replace(" ", "_"))
                .fetch_one(&**pool).await;

        match user_row_result {
            Some(user_row) => {
                return Self {
                    id: user_row.id,
                    osuver: osu_ver.to_string(),
                    username: user_row.username,
                    username_safe: user_row.username_safe,
                    ban_datetime: user_row.ban_datetime.parse::<i32>().unwrap_or(0_i32),
                    password_md5: user_row.password_md5,
                    salt: user_row.salt,
                    email: user_row.email,
                    register_datetime: user_row.register_datetime,
                    rank: user_row.rank,
                    allowed: user_row.allowed,
                    latest_activity: user_row.latest_activity,
                    silence_end: user_row.silence_end,
                    silence_reason: user_row.silence_reason,
                    password_version: user_row.password_version,
                    privileges: Privileges::from_value(user_row.privileges),
                    donor_expire: user_row.donor_expire,
                    flags: user_row.flags,
                    achievements_version: user_row.achievements_version,
                    achievements_0: user_row.achievements_0,
                    achievements_1: user_row.achievements_1,
                    notes: user_row.notes.unwrap(),
        
                    frozen: user_row.frozen,
                    freezedate: user_row.freezedate,
                    firstloginafterfrozen: user_row.firstloginafterfrozen,
        
                    bypass_hwid: user_row.bypass_hwid,
                    ban_reason: user_row.ban_reason,
        
                    utc_offset: 0,
                    country: "XX".to_string(),
                    geoloc: 0,
                    bancho_priv: BanchoPrivileges::PLAYER,
                    long: 0.0,
                    lat: 0.0,
        
                    action: Action::Unknown,
                    info_text: "".to_string(),
                    map_md5: "".to_string(),
                    mods: Mods::NOMOD,
                    current_mode: Mode::std,
                    map_id: 0,
        
                    token: token.to_string(),
                }
            }

            None => {
                return None;
            }
        }

        
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
