use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse, Result,
};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
        _ => Err(TicketError::ValidationError {
            field: "no flight path found".to_string(),
        }),
    }
}

fn find_key_for_value<'a>(map: &'a HashMap<String, i32>, value: i32) -> Option<&'a String> {
    map.iter()
        .find_map(|(key, &val)| if val == value { Some(key) } else { None })
}

#[derive(Debug, Display, Serialize, Error)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_ticket() {
        let input = vec![Ticket("SFO".to_string(), "EWR".to_string())];
        let expected = Ticket("SFO".to_string(), "EWR".to_string());

        if let Ok(ticket) = ticket_route(input) {
            assert_eq!(expected, ticket);
        }
    }

    #[test]
    fn test_two_tickets() {
        let input = vec![
            Ticket("ATL".to_string(), "EWR".to_string()),
            Ticket("SFO".to_string(), "ATL".to_string()),
        ];
        let expected = Ticket("SFO".to_string(), "EWR".to_string());

        if let Ok(ticket) = ticket_route(input) {
            assert_eq!(expected, ticket);
        }
    }

    #[test]
    fn test_three_tickets() {
        let input = vec![
            Ticket("IND".to_string(), "EWR".to_string()),
            Ticket("SFO".to_string(), "ATL".to_string()),
            Ticket("GSO".to_string(), "IND".to_string()),
            Ticket("ATL".to_string(), "GSO".to_string()),
        ];
        let expected = Ticket("SFO".to_string(), "EWR".to_string());

        if let Ok(ticket) = ticket_route(input) {
            assert_eq!(expected, ticket);
        }
    }

    #[test]
    fn bad_input() {
        let input = Vec::new();

        if let Ok(_ticket) = ticket_route(input) {
            panic!("Bad input should fail")
        } else {
            ()
        }
    }
}
