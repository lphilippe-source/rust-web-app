mod db;
mod domain;
mod router;
mod controller;
mod service;
mod persistence;

use std::net:: TcpListener;
// use std::env;
use env_logger;

//DATABASE URL
// const DB_URL: &str = env!("DATABASE_URL");
const DB_URL: &str ="postgresql://postgres:postgres@localhost:5432/postgres";

//constants
const OK_RESPONSE: &str =
    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE\r\nAccess-Control-Allow-Headers: Content-Type\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";

//main function
fn main() {

    env_logger::init();

    //Set Database
    if let Err(_) = db::set_database() {
        println!("Error setting database");
        return;
    }

    //start server and print port
    let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                router::handle_client(stream);
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
