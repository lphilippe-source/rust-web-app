pub mod handle_post_request;
pub mod handle_get_request;
pub mod handle_put_request;
pub mod handle_delete_request;

use crate::domain::*;
use log;

//Get id from request URL
pub fn get_id(request: &str) -> &str {
    log::error!("request {}",request);
    request.split("/").nth(4).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

//deserialize user from request body without id
pub fn get_user_request_body(request: &str) -> Result<user_dto::UserDto, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())

}
