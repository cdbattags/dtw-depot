use actix_web::{
    get,
    App,
    HttpServer,
    HttpRequest,
    Responder,
};
use actix_web::middleware::Logger;
use actix_web::middleware::Compress;
use env_logger::Env;

// #[get("/{id}/{name}")]
// async fn index(req: HttpRequest, info: web::Path<(u32, String)>) -> impl Responder {
//     format!(
//         "Hello {}! id:{}",
//         info.1,
//         info.0
//     )
// }

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    format!(
        "{:?}",
        req,
    )
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| App::new()
        .wrap(Compress::default())
        .wrap(Logger::default())
        .service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
