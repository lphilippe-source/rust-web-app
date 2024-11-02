use postgres::{Client, NoTls};
use crate::{DB_URL, domain::*, INTERNAL_ERROR, OK_RESPONSE};
use crate::service::user_service;

pub fn get_one_user(request: &str) -> (String, String) {
    user_service::get_user(request)
}
pub fn get_all_users(request: &str) -> (String, String) {
    user_service::get_all_users(request)
}
