use postgres::{Client, NoTls};
use crate::controller;
use crate::{DB_URL, INTERNAL_ERROR};
use crate::persistence::user_repository;
pub fn get_user(request: &str) -> (String, String) {
    match (controller::get_id(&request).parse::<i32>(), Client::connect(&DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) =>
            user_repository::retrieve_user(client, id),

        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}
