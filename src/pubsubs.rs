use futures::StreamExt;
use serde_json::Value;
use std::str::FromStr;

use crate::packets::handlers;
use crate::{bcrypt_cache, players, redis};

async fn ban_handler(user_id: i32) {
    let _user = players.get_id(user_id).await.unwrap();
    let mut user = _user.write().await;

    user.handle_restriction().await; // generic function moment
}

async fn bot_msg_handler(raw: &str) {
    let data: Value = serde_json::from_str(raw).unwrap();

    unimplemented!(); // TODO: actually send msg
}

async fn change_password_handler(md5: &str) {
    // XX: do i even need this? it's not bound per user.
    let mut cache = bcrypt_cache.lock().await;
    if cache.contains_key(md5) {
        cache.remove(md5);
    }

    // TODO: maybe pre-cache their bcrypt here?
}

async fn change_username_handler(raw: &str) {
    let data: Value = serde_json::from_str(raw).unwrap(); // userID, newUsername

    let _user = players
        .get_id(data["userID"].as_i64().unwrap() as i32)
        .await
        .unwrap();
    let user = _user.read().await;

    let mut packet_bytes = handlers::notification(&format!(
        "Your username has been changed to {}!",
        data["newUsername"].as_str().unwrap()
    ));
    packet_bytes.extend(handlers::server_restart(0));

    // TODO: consider their status into what action we take rather than just sending notif + relogging the user
    user.enqueue(packet_bytes).await;
}

async fn disconnect_handler(raw: &str) {
    let data: Value = serde_json::from_str(raw).unwrap(); // userID, reason

    unimplemented!(); // TODO: handle kicks
}

async fn notification_handler(raw: &str) {
    let data: Value = serde_json::from_str(raw).unwrap(); // userID, message

    let _user = players
        .get_id(data["userID"].as_i64().unwrap() as i32)
        .await
        .unwrap();
    let user = _user.read().await;

    user.enqueue(handlers::notification(data["message"].as_str().unwrap()))
        .await;
}

pub async fn initialise_pubsubs() {
    let conn = redis.get().unwrap().get_async_connection().await.unwrap();
    let mut pubsub_conn = conn.into_pubsub();

    for pubsub in vec!["peppy:ban"] {
        pubsub_conn.subscribe(pubsub).await.unwrap();
    }

    loop {
        let msg = pubsub_conn.on_message().next().await.unwrap();
        let channel = msg.get_channel_name();

        let content: String = msg.get_payload().unwrap();

        match channel {
            "peppy:ban" => ban_handler(i32::from_str(&content).unwrap()).await,
            "peppy:bot_msg" => bot_msg_handler(&content).await,
            "peppy:change_pass" => change_password_handler(&content).await,
            "peppy:disconnect" => disconnect_handler(&content).await,
            "peppy:notification" => notification_handler(&content).await,
            _ => continue,
        };

        println!("Handled {} with content {}", channel, content);
    }
}
