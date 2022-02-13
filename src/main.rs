// i didn't want to add these global allows, but some are unfixable cus rust isn't smart enough
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

extern crate redis;

mod bancho;
mod constants;
mod objects;
mod packets;
mod pubsubs;

use ntex::http::Method;
use ntex::util::Bytes;
use ntex::web::{self, middleware, App, HttpRequest, HttpResponse};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use std::env;

use maxminddb::Reader as MaxmindReader;
use std::collections::HashMap;
use tokio::sync::Mutex;

use crate::objects::players::PlayerList;

use lazy_static::lazy_static;
use once_cell::sync::OnceCell;

lazy_static! {
    static ref players: PlayerList = PlayerList::new();
    static ref reader: MaxmindReader<Vec<u8>> =
        MaxmindReader::open_readfile("ext/geoloc.mmdb").unwrap();
    static ref bcrypt_cache: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

static db: OnceCell<Pool<MySql>> = OnceCell::new();
static redis: OnceCell<redis::Client> = OnceCell::new();

async fn handle_conn(req: HttpRequest, _data: Bytes) -> HttpResponse {
    match req.method() {
        &Method::GET => {
            // GET request, render index
            return HttpResponse::Ok().body("rosu 2022™️");
        }
        &Method::POST => {
            // POST request, should be login/packet update request
            if req.headers().get("User-Agent").unwrap().to_str().unwrap() == "osu!" {
                // it's osu!
                return bancho::bancho(req, _data.to_vec()).await;
            } else {
                // not osu!, render index
                return HttpResponse::Ok().body("rosu 2022™️");
            }
        }
        _ => {
            return HttpResponse::BadRequest().body("rosu: bad request");
        }
    }
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let pool = MySqlPoolOptions::new()
        .connect(env::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap();

    db.set(pool).unwrap();

    let r = redis::Client::open("redis://127.0.0.1/").unwrap();
    redis.set(r).unwrap();

    tokio::spawn(async move {
        pubsubs::initialise_pubsubs().await;
    });

    web::server(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(handle_conn))
    })
    .bind_uds("/tmp/rosu.sock")?
    .run()
    .await
}
