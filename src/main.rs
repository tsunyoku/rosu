mod packets;
mod structs;

use ntex::web::{self, middleware, App, HttpRequest, HttpResponse};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{Pool, MySql};
use ntex::http::Method;
use ntex::util::Bytes;
use bcrypt;

use crate::packets::packets::Packets;
use crate::packets::writer;

type DBPool = web::types::Data<Pool<MySql>>;

// constant packets. optimisation p100!
const WELCOME_NOTIFICATION: &mut Vec<u8> = &mut writer::write(Packets::CHO_NOTIFICATION, "Welcome to ROsu!");
const PROTOCOL_VERSION: &mut Vec<u8> = &mut writer::write(Packets::CHO_PROTOCOL_VERSION, 19);
const INFO_END: &mut Vec<u8> = &mut writer::write(Packets::CHO_CHANNEL_INFO_END, None::<()>); // this none is ugly LOL

#[allow(unused)] // temporary while login isnt fully functional
async fn login(data: Vec<u8>, pool: DBPool) -> (&'static str, Vec<u8>) {
    let mut return_data: Vec<u8> = Vec::new();

    let login_str = String::from_utf8(data).unwrap();
    let mut login_data = login_str.split("\n").collect::<Vec<&str>>();

    login_data.pop(); // useless

    if login_data.len() != 3 {
        return ("no", return_data); // invalid request
    }

    let username = login_data[0];
    let password = login_data[1];

    let mut client_info = login_data[2].split("|").collect::<Vec<&str>>();
    if client_info.len() != 5 {
        return ("no", return_data); // invalid request
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

    let user = sqlx::query_as!(
        structs::User, 
        "select * from users where username_safe = ?", 
        username.lowercase().replace(" ", "_")
    ).fetch_one(&pool).await.unwrap();


    // TODO: find cleaner way to add multiple packets
    if !user {
        return_data.append(
            &mut writer::write(
                Packets::CHO_USER_ID,
                -1
            )
        );

        return_data.append(
            &mut writer::write(
                Packets::CHO_NOTIFICATION,
                "Unknown username"
            )
        );

        return ("no", return_data);
    }

    // verify password, using web::block to avoid blocking the thread
    let valid_password = web::block(move || {
        bcrypt::verify(password, &user.password_hash)
    }).await.unwrap();

    if !valid_password {
        return_data.append(
            &mut writer::write(
                Packets::CHO_USER_ID,
                -1
            )
        );

        return_data.append(
            &mut writer::write(
                Packets::CHO_NOTIFICATION,
                "Incorrect password"
            )
        );

        return ("no", return_data);
    }

    // TODO: hardware checks, clan

    return_data.append(PROTOCOL_VERSION);

    return_data.append(
        &mut writer::write(
            Packets::CHO_USER_ID,
            user.id
        )
    );

    return_data.append(WELCOME_NOTIFICATION);
    return_data.append(INFO_END);

    return_data.append(
        &mut writer::write(
            Packets::CHO_MAIN_MENU_ICON,
            "|" // icon | link (empty for now)
        )
    );

    let friends_list: Vec<i32>; // fake for now
    return_data.append(
        &mut writer::write(
            Packets::CHO_FRIENDS_LIST,
            friends_list
        )
    );

    return_data.append(
        &mut writer::write(
            Packets::CHO_SILENCE_END,
            0
        )
    );

    return_data.append(&mut writer::user_presence(&user));
    return_data.append(&mut writer::user_stats(&user));

    return ("no", return_data);
}

async fn bancho(req: HttpRequest, _data: Vec<u8>, _pool: DBPool) -> HttpResponse {
    let mut return_data: Vec<u8> = Vec::new();

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
