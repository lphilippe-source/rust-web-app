use postgres::{Client,NoTls};
use crate::domain::user_dto;
use crate::{DB_URL, INTERNAL_ERROR, NOT_FOUND, OK_RESPONSE};

pub fn retrieve_user(id: i32) -> (String, String) {
    match Client::connect(&DB_URL, NoTls) {
        Ok(mut client) => match client.query_one("SELECT * FROM users WHERE id = $1", &[&id]) {
            Ok(row) => {
                let user = user_dto::UserDto::new(row.get(1), row.get(2));

                (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
            }
            _ => (NOT_FOUND.to_string(), "User not found".to_string()),
        },
        Err(e) => ("CONNECTION_ERROR".to_string(),"connexion to database failed".to_string())
    }
}

pub fn retrieve_all_users()->(String, String){

    match Client::connect(&DB_URL, NoTls){
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
