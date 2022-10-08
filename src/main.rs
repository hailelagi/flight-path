use actix_web::{
    get, middleware::Logger, web, web::Json, App, HttpResponse, HttpServer, Responder, Result,
};

pub mod ticket;
use ticket::{ticket_route, Ticket};

#[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .service(index)
            .route("/calculate", web::post().to(flight_path))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Flight path tracker microservice. Responds only to POST /calculate")
}

async fn flight_path(tickets: web::Json<Vec<Ticket>>) -> Result<impl Responder> {
    match tickets {
        Json(t) => Ok(Json(ticket_route(t))),
    }
}
