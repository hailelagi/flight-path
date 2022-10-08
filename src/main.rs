use actix_web::{
    get, web, error, Result, Responder,
    http::{header::ContentType, StatusCode},
     middleware::Logger,
    App, HttpResponse, HttpServer,
};
use std::collections::HashMap;
use serde::Deserialize;
use log::info;
use derive_more::{Display, Error};


#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Flight path tracker microservice. Responds only to POST /calculate")
}

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

async fn flight_path(tickets: web::Json<Vec<(String, String)>>) -> impl Responder {

    println!("{:?}", ticket_route(vec![Ticket(String::from("SFO"), String::from("EWR"))]));
    HttpResponse::Ok().body("pass")
}

#[derive(Debug, Deserialize)]
pub struct Ticket(String, String);

pub fn ticket_route(tickets: Vec<Ticket>) -> Result<Ticket, TicketError> {
    let mut ticket_path = HashMap::new();

    // O(n) time complexity
    for ticket in tickets {
        let source = ticket.0;
        let destination = ticket.1;

        *ticket_path.entry(source).or_insert(0) += 1;
        *ticket_path.entry(destination).or_insert(0) -= 1;
    }

    let search_source = find_key_for_value(&ticket_path, 1);
    let search_destination = find_key_for_value(&ticket_path, -1);

    match (search_source, search_destination) {
        (Some(s), Some(d)) => Ok(Ticket(s.to_string(), d.to_string())),
        _ => Err(TicketError::ValidationError{field: "no flight path found".to_string()})
    }
}

fn find_key_for_value<'a>(map: &'a HashMap<String, i32>, value: i32) -> Option<&'a String> {
    map.iter().find_map(|(key, &val)| if val == value { Some(key) } else { None })
}

#[derive(Debug, Display, Error)]
pub enum TicketError {
    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },
}

impl error::ResponseError for TicketError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            TicketError::ValidationError { .. } => StatusCode::BAD_REQUEST,
        }
    }
}
