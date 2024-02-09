use postgres::{Client, NoTls};
use crate::{DB_URL, domain::*, INTERNAL_ERROR, OK_RESPONSE};
use crate::service::user_service;

pub fn get_one_user(request: &str) -> (String, String) {
    user_service::get_user(request)
}
pub fn get_all_user(_request: &str) -> (String, String) {
    match Client::connect(&DB_URL, NoTls) {
        Ok(mut client) => {
            let mut users = Vec::new(); // Vector to store the users

            for row in client.query("SELECT id, name, email FROM users", &[]).unwrap() {
                users.push(user_dto::UserDto::new(row.get(1), row.get(2)).set_id(row.get(0)));
            }

            (OK_RESPONSE.to_string(), serde_json::to_string(&users).unwrap())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}
