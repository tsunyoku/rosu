mod packets;
mod constants;
mod objects;

use ntex::web::{self, middleware, App, HttpRequest, HttpResponse};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{Pool, MySql};
use std::time::Instant;
use ntex::http::Method;
use ntex::util::Bytes;
use bcrypt;
use uuid::Uuid;

use crate::packets::handlers;
use crate::objects::user::User;

type DBPool = web::types::Data<Pool<MySql>>;

#[allow(unused)] // temporary while login isnt fully functional
async fn login(data: Vec<u8>, pool: DBPool) -> (String, Vec<u8>) {
    let start = Instant::now();

    let mut return_data: Vec<u8> = Vec::new();

    let login_str = String::from_utf8(data).unwrap();
    let mut login_data = login_str.split("\n").map(|chunk| chunk.to_owned()).collect::<Vec<String>>();

    if login_data.len() != 4 {
        return ("no".to_string(), return_data); // invalid request
    }

    let username = login_data[0].clone();
    let password = login_data[1].clone();

    let mut client_info = login_data[2].split("|").collect::<Vec<&str>>();
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
    let user_result = User::from_sql(&username, token, osu_ver, pool).await;

    if user_result == None {
        return ("no".to_string(), handlers::user_id(-1));
    }
    let user = user_result.unwrap();

    // verify password, using web::block to avoid blocking the thread
    let second_user = user.clone();
    let valid_password = web::block(move || bcrypt::verify(password, &second_user.password_md5)).await.unwrap();

    if !valid_password {
        return_data.extend(handlers::user_id(-1));
        return_data.extend(handlers::notification("Incorrect password"));

        return ("no".to_string(), return_data);
    }

    // TODO: hardware checks, clan

    return_data.extend(handlers::protocol_version(19));
    return_data.extend(handlers::user_id(user.id));
    return_data.extend(handlers::bancho_privileges(16));

    return_data.extend(
        handlers::notification(
            format!("Welcome to ROsu!\nTime Elapsed: {:.2?}", start.elapsed()).as_str()
        )
    );

    return_data.extend(handlers::channel_info_end());
    return_data.extend(handlers::main_menu_icon("", "")); // empty icon & url for now
    return_data.extend(handlers::friends_list(&user));
    return_data.extend(handlers::silence_end(0));

    return_data.extend(handlers::user_presence(&user));
    return_data.extend(handlers::user_stats(&user));

    println!("{} has logged in!", &username);
    return (token.to_string(), return_data);
}

async fn bancho(req: HttpRequest, _data: Vec<u8>, _pool: DBPool) -> HttpResponse {
    let return_data: Vec<u8> = Vec::new();

    if !req.headers().contains_key("osu-token") {
        let (token, login_data) = login(_data, _pool).await;

        if login_data.len() == 0 { // invalid request
            return HttpResponse::Ok()
                .header("cho-token", "no")
                .body("");
        }

        let packet_data = unsafe { String::from_utf8_unchecked(login_data) };
        return HttpResponse::Ok()
                .header("cho-token", token)
                .body(packet_data);
    }

    // already logged in client-side

    let packet_data = unsafe { String::from_utf8_unchecked(return_data) };
    return HttpResponse::Ok().body(packet_data);
}

async fn handle_conn(req: HttpRequest, _data: Bytes, _pool: DBPool) -> HttpResponse {
    match req.method() {
        &Method::GET => { // GET request, render index
            return HttpResponse::Ok().body("rosu 2022™️");
        },
        &Method::POST => { // POST request, should be login/packet update request
            if req.headers().get("User-Agent").unwrap().to_str().unwrap() == "osu!" { // it's osu!
               return bancho(req, _data.to_vec(), _pool).await;
            } else { // not osu!, render index
                return HttpResponse::Ok().body("rosu 2022™️");
            }
        },
        _ => {
            return HttpResponse::BadRequest().body("rosu: bad request");
        }
    }
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let pool = MySqlPoolOptions::new().connect("").await.unwrap();

    web::server(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(handle_conn))
    })
    .bind("127.0.0.1:9292")? // TODO: maybe use (or configurable) unix socket?
    .run()
    .await
}
