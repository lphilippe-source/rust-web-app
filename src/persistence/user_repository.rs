use postgres::Client;
use crate::domain::user_dto;
use crate::{NOT_FOUND, OK_RESPONSE};

pub fn retrieve_user(mut client: Client, id: i32) -> (String, String) {
    match client.query_one("SELECT * FROM users WHERE id = $1", &[&id]) {
        Ok(row) => {
            let user = user_dto::UserDto::new(row.get(1), row.get(2));

            (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
        }
        _ => (NOT_FOUND.to_string(), "User not found".to_string()),
    }
}
