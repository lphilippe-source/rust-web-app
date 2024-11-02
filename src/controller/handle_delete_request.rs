use postgres::{Client, NoTls};
use crate::service::user_service;
use crate::{DB_URL, INTERNAL_ERROR, NOT_FOUND, OK_RESPONSE};

pub fn delete_user(request: &str) -> (String, String) {
    match (user_service::get_id(&request).parse::<i32>(), Client::connect(&DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) => {
            let rows_affected = client.execute("DELETE FROM users WHERE id = $1", &[&id]).unwrap();

            //if rows affected is 0, user not found
            if rows_affected == 0 {
                return (NOT_FOUND.to_string(), "User not found".to_string());
            }

            (OK_RESPONSE.to_string(), "User deleted".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}
