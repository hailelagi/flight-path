use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Flight path tracker microservice. Responds only to POST /calculate")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .route("/calculate", web::post().to(flight_path))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn flight_path() -> impl Responder {
    // TODO:  validate req body
    // TODO: flight path logic
    HttpResponse::Ok().body("Hey there!")
}
