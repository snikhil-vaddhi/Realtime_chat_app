use actix::*;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{App, HttpServer, http, web};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::sync::Mutex;

mod db;
mod model;
mod routes;
mod schema;
mod server;
mod session;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Start the chat server
    let server = server::ChatServer::new().start();

    // Set up the SQLite connection
    let conn_spec = "chat.db"; // SQLite database file
    let conn =
        SqliteConnection::establish(conn_spec).expect("Failed to connect to SQLite database");

    // Wrap the connection in a Mutex for thread safety
    let conn = std::sync::Arc::new(Mutex::new(conn));

    // Server address and port
    let server_addr = "127.0.0.1";
    let server_port = 8080;

    // Start the HTTP server
    let app = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(server.clone())) // Share the chat server
            .app_data(web::Data::new(conn.clone())) // Share the SQLite connection
            .wrap(cors)
            .service(web::resource("/").to(routes::index))
            .route("/ws", web::get().to(routes::chat_server))
            .service(routes::create_user)
            .service(routes::get_user_by_id)
            .service(routes::get_user_by_phone)
            .service(routes::get_conversation_by_id)
            .service(routes::get_rooms)
            .service(Files::new("/", "./static")) // Serve static files
    })
    .workers(2)
    .bind((server_addr, server_port))?
    .run();

    println!("Server running at http://{server_addr}:{server_port}/");
    app.await
}
