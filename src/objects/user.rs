use crate::constants::action::Action;
use crate::constants::country::CountryCodes;
use crate::constants::mode::Mode;
use crate::constants::privileges::{BanchoPrivileges, Privileges};
use crate::objects::channel::Channel;
use crate::objects::mods::Mods;
use crate::objects::queue::PacketQueue;
use crate::objects::stats::Stats;
use crate::packets::handlers;
use crate::{db, players};

use uuid::Uuid;

use std::str::FromStr;
use strum::IntoEnumIterator;

use std::{collections::HashMap, sync::Arc};

macro_rules! pub_struct { // w.
    ($name:ident {$($field:ident: $t:ty,)*}) => {
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
    channels: HashMap<String, Arc<Channel>>,
});

impl User {
    // perhaps the worst part of this entire code rn
    pub async fn from_sql(username: &str, token: Uuid, osu_ver: &str, offset: i32) -> Option<Self> {
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

                let friend_rows = sqlx::query!(
                    "select user2 from users_relationships where user1 = ?",
                    user_row.id
                )
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
                    channels: HashMap::new(),
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
            .await
            .unwrap();
    }

    pub async fn remove_friend(&mut self, target: i32) {
        let user_index = self.friends.iter().position(|x| *x == target).unwrap();
        self.friends.remove(user_index);

        sqlx::query("DELETE FROM users_relationships WHERE user1 = ? AND user2 = ?")
            .bind(self.id)
            .bind(target)
            .execute(db.get().unwrap())
            .await
            .unwrap();
    }

    pub async fn logout(&mut self) {
        players.remove(self.id).await;

        for channel in self.channels.values() {
            channel.remove_user(self.id).await;
        }

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

    // generic function to do all actions after a confirmed restriction
    pub async fn handle_restriction(&mut self) {
        self.refresh_privileges().await; // reset their internal privileges for stuff

        // relog user so their panel etc. naturally refreshes
        self.enqueue(handlers::server_restart(0)).await;
    }

    pub async fn refresh_privileges(&mut self) {
        self.privileges =
            sqlx::query_as::<_, Privileges>("SELECT privileges FROM users WHERE id = ?")
                .bind(self.id)
                .fetch_one(db.get().unwrap())
                .await
                .unwrap();
    }
}
