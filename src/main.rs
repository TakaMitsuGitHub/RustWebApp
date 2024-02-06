use std::io::Result;
use log::info;
use actix_web::{App, HttpServer, Responder, HttpResponse, get, post
                middleware::Logger};
use env_logger::Env;


// #[get("/")]
// async fn index() -> impl Responder {
//     HttpResponse::Ok().body("Hello, Actix Web")
// }

#[get("/posts")]
pub async fn index() -> impl Responder {
    info!("called index");
    HttpResponse::Ok().body("Called index")
}

#[actix_rt::main]
async fn main() -> Result<()> {

    env_logger::init_from_env(Env::default().dafault_filter_or("info"));

    HttpServer::new(|| {
        App::new().service(index)
        .wrap(Logger::default())
    }).bind("127.0.0.1:8000")?.run().await
}

// https://atmarkit.itmedia.co.jp/ait/articles/2311/17/news004.html