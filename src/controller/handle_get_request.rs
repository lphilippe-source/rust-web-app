use postgres::{Client, NoTls};
use crate::controller::get_id;
use crate::{DB_URL, domain, INTERNAL_ERROR, NOT_FOUND, OK_RESPONSE};

pub fn get_one_user(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(&DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) =>
            match client.query_one("SELECT * FROM users WHERE id = $1", &[&id]) {
                Ok(row) => {

                    let user = domain::UserDto::new( row.get(1), row.get(2));

                    (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
                }
                _ => (NOT_FOUND.to_string(), "User not found".to_string()),
            }

        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}
pub fn get_all_user(_request: &str) -> (String, String) {
    match Client::connect(&DB_URL, NoTls) {
        Ok(mut client) => {
            let mut users = Vec::new(); // Vector to store the users

            for row in client.query("SELECT id, name, email FROM users", &[]).unwrap() {
                users.push(domain::UserDto::new(row.get(1), row.get(2)).set_id(row.get(0)));
            }

            (OK_RESPONSE.to_string(), serde_json::to_string(&users).unwrap())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}
