mod packets;

use ntex::web::{self, middleware, App, HttpRequest, HttpResponse};
use ntex::http::header::{HeaderName, HeaderValue};
use ntex::http::Method;
use ntex::util::Bytes;

use crate::packets::packets::Packets;
use crate::packets::writer;

async fn bancho(_data: Bytes, _req: HttpRequest) -> HttpResponse {
    let mut return_data: Vec<u8> = Vec::new();

    return_data.append(
        &mut writer::write(
            Packets::CHO_USER_ID,
            -1,
        )
    );

    return_data.append(
        &mut writer::write(
            Packets::CHO_NOTIFICATION,
            "rosu: notification!",
        )
    );

    let packet_data = unsafe { String::from_utf8_unchecked(return_data) };
    let mut resp = HttpResponse::from(packet_data);
    
    let token_header = HeaderName::from_lowercase(b"cho-token").unwrap();
    let token_value = HeaderValue::from_str("no").unwrap();
    resp.headers_mut().insert(token_header, token_value);

    return resp;
}

async fn handle_conn(_data: Bytes, req: HttpRequest) -> HttpResponse {
    match req.method() {
        &Method::GET => { // GET request, render index
            return HttpResponse::Ok().body("rosu 2022™️");
        },
        &Method::POST => { // POST request, should be login/packet update request
            if req.headers().get("User-Agent").unwrap().to_str().unwrap() == "osu!" { // it's osu!
               return bancho(_data, req).await;
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
    web::server(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(handle_conn))
    })
    .bind("127.0.0.1:9292")? // TODO: maybe use (or configurable) unix socket?
    .run()
    .await
}
