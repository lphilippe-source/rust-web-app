use crate::INTERNAL_ERROR;
use crate::domain::user_dto;
use crate::persistence::user_repository;

pub fn get_user(request: &str) -> (String, String) {
    match get_id(&request).parse::<i32>() {
        Ok(id) =>
            user_repository::retrieve_user(id),
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string())
    }
}

//Get id from request URL
pub fn get_id(request: &str) -> &str {
     request.split("/").nth(4).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

//deserialize user from request body without id
pub fn get_user_request_body(request: &str) -> Result<user_dto::UserDto, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}

pub fn get_all_users(_request: &str)->(String, String){
    user_repository::retrieve_all_users()
}
