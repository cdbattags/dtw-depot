use neon::prelude::*;
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

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    format!(
        "{:?}",
        req,
    )
}

fn hello(mut cx: FunctionContext) -> JsResult<JsObject> {
    println!("hello world");

    // Create the runtime
    Ok(
        actix_rt::System::new("app").block_on(async move {
            env_logger::from_env(Env::default().default_filter_or("info")).init();

            HttpServer::new(|| App::new()
                .wrap(Compress::default())
                .wrap(Logger::default())
                .service(index))
                .bind("127.0.0.1:8080")?
                .run()
                .await
        })
    )
}

register_module!(mut cx, {
    cx.export_function("hello", hello)
});
