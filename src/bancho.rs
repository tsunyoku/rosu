use bcrypt;
use ntex::http::HeaderMap;
use ntex::web::{self, HttpRequest, HttpResponse};
use std::time::Instant;
use uuid::Uuid;

use maxminddb::geoip2;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::objects::user::User;
use crate::packets::handlers::{self, PACKET_HANDLERS, RESTRICTED_PACKET_HANDLERS};
use crate::packets::reader::Reader;
use crate::constants::packets::Packets;

use crate::{players, bcrypt_cache, reader};

use num_traits::FromPrimitive;

#[inline(always)]
async fn login(data: Vec<u8>, headers: &HeaderMap) -> (String, Vec<u8>) {
    let start = Instant::now();

    let mut return_data: Vec<u8> = Vec::new();

    let login_str = String::from_utf8(data).unwrap();
    let login_data = login_str
        .split("\n")
        .map(|chunk| chunk.to_owned())
        .collect::<Vec<String>>();

    if login_data.len() != 4 {
        return ("no".to_string(), return_data); // invalid request
    }

    let username = login_data[0].clone();
    let password = login_data[1].clone();

    let client_info = login_data[2].split("|").collect::<Vec<&str>>();
    if client_info.len() != 5 {
        return ("no".to_string(), return_data); // invalid request
    }

    let osu_ver = client_info[0]; // TODO: validate
    let utc_offset: i32 = client_info[1].parse().unwrap();

    let mut client_str = client_info[3].to_string();
    client_str.pop();

    // TODO: use these.
    let client_hashes = client_str.split(":").collect::<Vec<&str>>();
    let osu_md5 = client_hashes[0];
    let mac_md5 = client_hashes[2];
    let uninstall_md5 = client_hashes[3];
    let disk_md5 = client_hashes[4];

    let private_dms = client_info[4] == "1";

    let token = Uuid::new_v4();
    let user_result = User::from_sql(&username, token, osu_ver, utc_offset).await;

    let mut user = match user_result {
        Some(user) => user,
        _ => {
            return ("no".to_string(), handlers::user_id(-1));
        }
    };

    // verify password, using web::block to avoid blocking the thread
    let bcrypt = user.password_md5.clone();

    let to_cache = user.password_md5.clone();
    let md5 = password.clone();

    let valid_password: bool;
    if !bcrypt_cache.lock().await.contains_key(&md5) {
        valid_password = web::block(move || bcrypt::verify(password, &bcrypt))
            .await
            .unwrap();
    } else {
        valid_password = bcrypt_cache.lock().await.get(&md5).unwrap() == &to_cache;
    }

    if !valid_password {
        return_data.extend(handlers::user_id(-1));
        return_data.extend(handlers::notification("Incorrect password"));

        return ("no".to_string(), return_data);
    }

    bcrypt_cache.lock().await.insert(md5, to_cache);

    // parse geoloc
    let ip: &str;

    if headers.contains_key("CF-Connecting-IP") {
        ip = headers.get("CF-Connecting-IP").unwrap().to_str().unwrap();
    } else {
        let forwards = headers // this is fucking unbelievable HAHAHA
            .get("X-Forwarded-For")
            .unwrap()
            .to_str()
            .unwrap()
            .split(",")
            .collect::<Vec<&str>>();

        if forwards.len() != 1 {
            ip = &forwards[0];
        } else {
            ip = headers.get("X-Real-IP").unwrap().to_str().unwrap();
        }
    }

    let geoloc_ip: IpAddr = FromStr::from_str(ip).unwrap();
    let city: geoip2::City = reader.lookup(geoloc_ip).unwrap();

    let location = city.location.unwrap();
    user.long = location.longitude.unwrap() as f32;
    user.lat = location.latitude.unwrap() as f32;

    // TODO: hardware checks, clan

    return_data.extend(handlers::protocol_version(19));
    return_data.extend(handlers::user_id(user.id));
    return_data.extend(handlers::bancho_privileges(user.bancho_priv.value()));

    return_data.extend(handlers::channel_info_end());
    return_data.extend(handlers::main_menu_icon("", "")); // empty icon & url for now
    return_data.extend(handlers::friends_list(&user));
    return_data.extend(handlers::silence_end(0));

    return_data.extend(handlers::user_presence(&user));
    return_data.extend(handlers::user_stats(&user));

    players.add_player(user).await;
    return_data.extend(handlers::notification(
        format!(
            "Welcome to ROsu!\n\nTime Elapsed: {:.2?}\nPlayers online: {}",
            start.elapsed(),
            players.player_count().await
        )
        .as_str(),
    ));

    println!("{} has logged in!", &username);
    return (token.to_string(), return_data);
}

pub async fn bancho(req: HttpRequest, _data: Vec<u8>) -> HttpResponse {
    if !req.headers().contains_key("osu-token") {
        let (token, login_data) = login(_data, &req.headers()).await;

        if login_data.len() == 0 {
            // invalid request
            return HttpResponse::Ok().header("cho-token", "no").body("");
        }

        let packet_data = unsafe { String::from_utf8_unchecked(login_data) };
        return HttpResponse::Ok()
            .header("cho-token", token)
            .body(packet_data);
    }

    // already logged in client-side
    let token = req.headers().get("osu-token").unwrap().to_str().unwrap();

    let user: Arc<RwLock<User>>; // arc'd player, we will read from the arc below
    match players.get_token(token).await {
        Some(u) => user = u,
        _ => {
            let return_vec = handlers::server_restart(0);

            return HttpResponse::Ok().body(unsafe { String::from_utf8_unchecked(return_vec) });
        }
    }

    let mut player = user.write().await; // get readable player
    let mut _reader = Reader::new(_data);

    while !_reader.empty() {
        let (id, len) = _reader.read_header();
        let packet = Packets::from_i32(id).unwrap();

        // &* lmao
        let mut handler_map = &*PACKET_HANDLERS;
        if player.restricted() {
            handler_map = &*RESTRICTED_PACKET_HANDLERS;
        }

        if handler_map.contains_key(&packet) {
            let callback = handler_map[&packet];
            let should_increment = callback(&mut player, &mut _reader).await;

            if should_increment {
                _reader.incr_offset(len as usize);
            }

            if packet != Packets::OSU_PING {
                println!("Packet {:?} handled for {}", packet, player.username);
            }
        } else {
            _reader.incr_offset(len as usize);
        }
    }

    let return_data = player.dequeue().await;
    let packet_data = unsafe { String::from_utf8_unchecked(return_data) };
    return HttpResponse::Ok().body(packet_data);
}