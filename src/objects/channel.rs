use crate::objects::user::{PlayerList, User};
use crate::packets::handlers::channel_message;
use tokio::sync::RwLock;
use std::sync::Arc;

// Structure representing an in-game channel meant for chatting.
pub struct Channel {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub users: PlayerList,
    pub autojoin: bool,
}

impl Channel {
    pub async fn add_user(&self, player: Arc<RwLock<User>>) {
        self.users.add_player_ptr(player).await;
        // TODO: Handle adding the Arc for this channel to the user.
    }

    // Handles removing a user from the channel.
    pub async fn remove_user(&self, user_id: i32) {
        if let Some(user_locked) = self.users.get_id(user_id.clone()).await {
            let mut user = user_locked.write().await;

            // Remove channel arc.
            user.channels.remove(&self.name);

            self.users.remove(user_id).await;
        } else {
            println!("Tried to remove a user from a channel they weren't a part of?");
        }
        
    }

    pub async fn send_message(&self, player: Arc<RwLock<User>>, content: String) {
        let player = player.read().await;
        self.users.enqueue(channel_message(
            player.username.clone(),
            player.id.clone(),
            content,
            self.name.clone(),
        )).await;
    }

    pub async fn send_message_userid(&self, user_id: i32, content: String) {
        if let Some(player_locked) = self.users.get_id(user_id).await {
            self.send_message(player_locked, content).await;
        }
    }


}
