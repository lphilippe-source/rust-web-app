use postgres::{Client, NoTls};
use crate::controller::{get_id, get_user_request_body};
use crate::{DB_URL, domain, INTERNAL_ERROR, OK_RESPONSE};

pub fn update_user(request: &str) -> (String, String) {
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
