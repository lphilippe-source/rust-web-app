use postgres::{Client, NoTls};
use crate::{DB_URL, domain, INTERNAL_ERROR, NOT_FOUND, OK_RESPONSE};
use log;

//Get id from request URL
fn get_id(request: &str) -> &str {
    log::error!("request {}",request);
    request.split("/").nth(4).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

//deserialize user from request body without id
fn get_user_request_body(request: &str) -> Result<domain::UserDto, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())

}

//handle requests

//handle post request
pub fn handle_post_request(request: &str) -> (String, String) {
    match (get_user_request_body(request), Client::connect(&DB_URL, NoTls)) {
        (Ok(user), Ok(mut client)) => {
            // Insert the user and retrieve the ID

            let domain::UserField::Name(name) = user.get_name() else{
                log::error!("{:?}",user.get_name());
               return (INTERNAL_ERROR.to_string(), "Name is not set".to_string());
            };

            let domain::UserField::Email(email) = user.get_email() else{
                log::error!("{:?}",user.get_email());
                return (INTERNAL_ERROR.to_string(), "Email is not set".to_string());
            };
            let row = client
                .query_one(
                    "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id",
                    &[&name, &email]
                )
                .unwrap();

            log::error!("User created {:?}", row);
            let user_id: i32 = row.get(0);

            log::error!("User index {:?}", user_id);
            // Fetch the created user data
            match client.query_one("SELECT id, name, email FROM users WHERE id = $1", &[&user_id]) {
                Ok(row) => {

                let user = domain::UserDto::new( row.get(1), row.get(2)).set_id(row.get(0));

                    log::error!("User created {:?}", &user);
                    (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
                }
                Err(_) =>
                    (INTERNAL_ERROR.to_string(), "Failed to retrieve created user".to_string()),
            }
        }
        _ =>{
            (INTERNAL_ERROR.to_string(), "Internal error".to_string())
        }
    }
}

//handle get request
pub fn handle_get_request(request: &str) -> (String, String) {
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

//handle get all request
pub fn handle_get_all_request(_request: &str) -> (String, String) {
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

//handle put request
pub fn handle_put_request(request: &str) -> (String, String) {
    match
    (
        get_id(&request).parse::<i32>(),
        get_user_request_body(&request),
        Client::connect(&DB_URL, NoTls),
    )
    {
        (Ok(id), Ok(user), Ok(mut client)) => {

            let domain::UserField::Name(name) = user.get_name() else{
                log::error!("{:?}",user.get_name());
                return (INTERNAL_ERROR.to_string(), "Name is not set".to_string());
            };

            let domain::UserField::Email(email) = user.get_email() else{
                log::error!("{:?}",user.get_email());
                return (INTERNAL_ERROR.to_string(), "Email is not set".to_string());
            };
            client
                .execute(
                    "UPDATE users SET name = $1, email = $2 WHERE id = $3",
                    &[&name, &email, &id]
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "User updated".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

//handle delete request
pub fn handle_delete_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(&DB_URL, NoTls)) {
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
