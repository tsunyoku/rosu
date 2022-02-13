use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, RwLock};

use crate::objects::user::User;

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

    // Adds an rwlocked player shared pointer to the player list.
    pub async fn add_player_ptr(&self, player: Arc<RwLock<User>>) {
        let user_id = player.read().await.id.clone();
        self.players.lock().await.insert(user_id, player);
    }

    pub async fn enqueue(&self, bytes: Vec<u8>) {
        for player in self.players.lock().await.values() {
            let user = player.read().await;
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

    pub async fn remove(&self, user_id: i32) {
        self.players.lock().await.remove(&user_id);
    }
}
