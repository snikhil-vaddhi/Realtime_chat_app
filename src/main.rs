use actix::*;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{App, HttpServer, http, web};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};

mod db;
mod models;
mod routes;
mod schema;
mod server;
mod session;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = server::ChatServer::new().start();
    let conn_spec = "chat.db";
    let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    let server_addr = "127.0.0.1";
    let server_port = 8080;
    let app = HttpServer::new(move ||{
        
    })
}
