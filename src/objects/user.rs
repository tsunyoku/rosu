use crate::constants::CountryCodes;
use crate::objects::mode::{Mode, Stats};
use crate::objects::mods::Mods;
use crate::objects::privileges::{BanchoPrivileges, Privileges};
use crate::packets::handlers;
use crate::{players, db};

use uuid::Uuid;

use serde::{Serialize, Serializer};
use std::str::FromStr;
use num_derive::FromPrimitive;

use strum::IntoEnumIterator;

use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, RwLock};

macro_rules! pub_struct { // w.
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

pub struct PacketQueue {
    queue: Mutex<Vec<u8>>,
}

impl PacketQueue {
    pub fn new() -> Self {
        return Self {
            queue: Mutex::new(Vec::with_capacity(512)),
        };
    }

    #[inline(always)]
    pub async fn dequeue(&self) -> Vec<u8> {
        let mut queue = self.queue.lock().await;
        let queue_vec = queue.clone();

        queue.clear();
        return queue_vec;
    }

    pub async fn enqueue(&self, bytes: Vec<u8>) {
        self.queue.lock().await.extend(bytes);
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
    achievements_0: i32,       // unused?
    achievements_1: i32,       // unused?
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

    token: String,      // rando token
    queue: PacketQueue, // for sending packets to the user

    stats: Vec<Stats>,
    friends: Vec<i32>,

    spectating: Option<i32>,
    spectators: Vec<i32>,
});

impl User {
    // perhaps the worst part of this entire code rn
    pub async fn from_sql(
        username: &str,
        token: Uuid,
        osu_ver: &str,
        offset: i32,
    ) -> Option<Self> {
        let user_row = sqlx::query!(
            "select * from users where username_safe = ?",
            username.to_lowercase().replace(" ", "_")
        )
        .fetch_one(db.get().unwrap())
        .await;

        match user_row {
            Ok(user_row) => {
                let country =
                    sqlx::query!("select country from users_stats where id = ?", user_row.id)
                        .fetch_one(db.get().unwrap())
                        .await
                        .unwrap()
                        .country;

                let friend_rows = sqlx::query!("select user2 from users_relationships where user1 = ?", user_row.id)
                                    .fetch_all(db.get().unwrap())
                                    .await
                                    .unwrap();

                let friends_vec = friend_rows.iter().map(|v| v.user2).collect::<Vec<i32>>();

                let geoloc = CountryCodes::from_str(&country.to_uppercase())
                    .unwrap_or(CountryCodes::XX) as u8;

                let mut stats_vec: Vec<Stats> = Vec::new();
                for mode in Mode::iter() {
                    stats_vec.push(Stats::for_mode(mode, user_row.id).await);
                }

                return Some(Self {
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
                    utc_offset: offset,
                    country: country,
                    geoloc: geoloc,
                    bancho_priv: BanchoPrivileges::from_privileges(user_row.privileges),
                    long: 0.0, // set later in login
                    lat: 0.0,  // set later in login
                    action: Action::Idle,
                    info_text: "".to_string(),
                    map_md5: "".to_string(),
                    mods: Mods::NOMOD,
                    current_mode: Mode::std,
                    map_id: 0,
                    token: token.to_string(),
                    queue: PacketQueue::new(),
                    stats: stats_vec,
                    friends: friends_vec,
                    spectating: None,
                    spectators: Vec::new(),
                });
            }
            _ => return None,
        };
    }

    pub async fn enqueue(&self, bytes: Vec<u8>) {
        self.queue.enqueue(bytes).await;
    }

    pub async fn dequeue(&self) -> Vec<u8> {
        return self.queue.dequeue().await;
    }

    pub fn restricted(&self) -> bool {
        return self.privileges & Privileges::USER_PUBLIC < Privileges::USER_PUBLIC;
    }

    pub async fn add_friend(&mut self, target: i32) {
        self.friends.push(target);

        sqlx::query("INSERT INTO users_relationships (user1, user2) VALUES (?, ?)")
            .bind(self.id)
            .bind(target)
            .execute(db.get().unwrap())
            .await.unwrap();
    }

    pub async fn remove_friend(&mut self, target: i32) {
        let user_index = self.friends.iter().position(|x| *x == target).unwrap();
        self.friends.remove(user_index);

        sqlx::query("DELETE FROM users_relationships WHERE user1 = ? AND user2 = ?")
            .bind(self.id)
            .bind(target)
            .execute(db.get().unwrap())
            .await.unwrap();
    }

    pub async fn logout(&mut self) {
        players.remove_player(self.id).await;

        if !self.restricted() {
            players.enqueue(handlers::logout(self.id)).await;
        }
    }

    pub async fn add_spectator(&mut self, user: &mut User) {
        let join_packet = handlers::spectator_joined(user.id);

        // check, optionally create, and join spec channel

        for uid in &self.spectators {
            let u = players.get_id(*uid).await.unwrap();
            let _user = u.read().await;

            _user.enqueue(join_packet.clone()).await;
            user.enqueue(handlers::spectator_joined(_user.id)).await;
        }

        self.spectators.push(user.id);
        user.spectating = Some(self.id);

        self.enqueue(handlers::host_spectator_joined(user.id)).await;
        println!("{} started spectating {}", user.username, self.username);
    }

    pub async fn remove_spectator(&mut self, user: &mut User) {
        let user_index = self.spectators.iter().position(|x| *x == user.id).unwrap();
        self.spectators.remove(user_index);
        user.spectating = None;

        // leave spec channel (update channel info etc.) &
        // check if remaining spectators == 0 and remove self if so

        let leave_packet = handlers::spectator_left(user.id);
        for uid in &self.spectators {
            // this will need to be in the else clause of channel deletion once it exists
            let u = players.get_id(*uid).await.unwrap();
            let _user = u.read().await;

            _user.enqueue(leave_packet.clone()).await;
        }

        self.enqueue(handlers::host_spectator_left(user.id)).await;
        println!("{} stopped spectating {}", user.username, self.username);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
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

pub struct PlayerList {
    pub players: Mutex<HashMap<i32, Arc<RwLock<User>>>>,
}

impl PlayerList {
    pub fn new() -> Self {
        return Self {
            players: Mutex::new(HashMap::new()),
        };
    }

    pub async fn player_count(&self) -> usize {
        return self.players.lock().await.len();
    }

    pub async fn add_player(&self, player: User) {
        let user_id = player.id.clone();

        let player_arc = Arc::from(RwLock::from(player));
        self.players.lock().await.insert(user_id, player_arc);
    }

    pub async fn remove_player(&self, user_id: i32) {
        self.players.lock().await.remove(&user_id);
    }

    pub async fn enqueue(&self, bytes: Vec<u8>) {
        for player in self.players.lock().await.values() {
            let user = player.read().await;
            println!("playerlist enqueue sent to {}", user.username);
            user.enqueue(bytes.clone()).await;
        }
    }

    pub async fn get_id(&self, user_id: i32) -> Option<Arc<RwLock<User>>> {
        match self.players.lock().await.get(&user_id) {
            Some(u) => Some(u.clone()),
            _ => None,
        }
    }

    pub async fn get_username(&self, username: &str) -> Option<Arc<RwLock<User>>> {
        for u in self.players.lock().await.values() {
            let player = u.read().await;
            if &player.username == username {
                return Some(u.clone());
            }
        }

        return None;
    }

    pub async fn get_token(&self, token: &str) -> Option<Arc<RwLock<User>>> {
        for u in self.players.lock().await.values() {
            let player = u.read().await;
            if &player.token == token {
                return Some(u.clone());
            }
        }

        return None;
    }

    pub async fn remove(&mut self, user_id: i32) {
        self.players.lock().await.remove(&user_id);
    }
}
