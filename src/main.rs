mod packets;

use ntex::web::{self, middleware, App, HttpRequest, HttpResponse};
use ntex::http::header::{HeaderName, HeaderValue};
use ntex::util::Bytes;
use std::str;

use crate::packets::packets::Packets;
use crate::packets::writer;

async fn index(_req: HttpRequest) -> &'static str {
    return "rosu 2021:tm:";
}

async fn bancho(_data: Bytes) -> HttpResponse {
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

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::server(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service((
                web::resource("/").route(web::get().to(index)).route(web::post().to(bancho)),
            ))
    })
    .bind("127.0.0.1:9292")?
    .run()
    .await
}
