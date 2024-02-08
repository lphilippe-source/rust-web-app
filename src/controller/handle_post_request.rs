use postgres::{Client, NoTls};
use crate::controller::get_user_request_body;
use crate::{DB_URL,domain::*, INTERNAL_ERROR, OK_RESPONSE};
pub fn post_user(request: &str) -> (String, String) {
    match (get_user_request_body(request), Client::connect(&DB_URL, NoTls)) {
        (Ok(user), Ok(mut client)) => {
            // Insert the user and retrieve the ID

            let user_dto::UserField::Name(name) = user.get_name() else{
                log::error!("{:?}",user.get_name());
                return (INTERNAL_ERROR.to_string(), "Name is not set".to_string());
            };

            let user_dto::UserField::Email(email) = user.get_email() else{
                log::error!("{:?}",user.get_email());
                return (INTERNAL_ERROR.to_string(), "Email is not set".to_string());
            };
            let row = client
                .query_one(
                    "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id",
                    &[&name, &email]
                )
                .unwrap();

            log::trace!("User created {:?}", row);
            let user_id: i32 = row.get(0);

            log::trace!("User index {:?}", user_id);
            // Fetch the created user data
            match client.query_one("SELECT id, name, email FROM users WHERE id = $1", &[&user_id]) {
                Ok(row) => {

                    let user = user_dto::UserDto::new(row.get(1), row.get(2)).set_id(row.get(0));

                    log::trace!("User created {:?}", &user);
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
