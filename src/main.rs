use actix_web::{App, get, HttpRequest, HttpResponse, HttpServer, web};
use actix_web::http::StatusCode;
use env_logger::{Env};
use actix_web::middleware::Logger;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(check)
    }
    ).bind("0.0.0.0:8080")?
        .workers(1)
        .run()
        .await
}


#[get("/check")]
pub async fn check(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().status(StatusCode::CREATED).finish()
}