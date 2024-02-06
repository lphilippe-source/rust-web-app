use std::io::{Read, Write};
use std::net::TcpStream;
use crate::{ NOT_FOUND, OK_RESPONSE};
use crate::controller::handle_delete_request::delete_user;
use crate::controller::handle_post_request::post_user;
use crate::controller::handle_get_request::*;
use crate::controller::handle_put_request::update_user;


pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("OPTIONS") => (OK_RESPONSE.to_string(), "".to_string()),
                r if r.starts_with("POST /api/rust/users") => post_user(r),
                r if r.starts_with("GET /api/rust/users/") => get_one_user(r),
                r if r.starts_with("GET /api/rust/users") => get_all_user(r),
                r if r.starts_with("PUT /api/rust/users/") => update_user(r),
                r if r.starts_with("DELETE /api/rust/users/") => delete_user(r),
                _ => (NOT_FOUND.to_string(), "404 not found".to_string()),
            };

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => eprintln!("Unable to read stream: {}", e),
    }
}
