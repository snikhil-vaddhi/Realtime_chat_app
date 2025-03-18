use crate::db;
use crate::model;
use crate::server;
use crate::session;
use actix::*;
use actix_files::NamedFile;
use actix_web::{Error, HttpRequest, HttpResponse, Responder, get, post, web};
use actix_web_actors::ws;
use diesel::sqlite::SqliteConnection;
use serde_json::json;
use std::sync::Mutex;
use std::time::Instant;
use uuid::Uuid;

pub async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

pub async fn chat_server(
    req: HttpRequest,
    stream: web::Payload,
    conn: web::Data<Mutex<SqliteConnection>>,
    srv: web::Data<Addr<server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        session::WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "main".to_string(),
            name: None,
            addr: srv.get_ref().clone(),
            db_conn: conn,
        },
        &req,
        stream,
    )
}

#[post("/users/create")]
pub async fn create_user(
    conn: web::Data<Mutex<SqliteConnection>>,
    form: web::Json<model::NewUser>,
) -> Result<HttpResponse, Error> {
    let mut conn = conn.lock().unwrap(); // Lock the Mutex to access the connection
    let user = db::insert_new_user(&mut conn, &form.username, &form.phone)
        .map_err(actix_web::error::ErrorUnprocessableEntity)?;
    Ok(HttpResponse::Ok().json(user))
}

#[get("users/{user_id}")]
pub async fn get_user_by_id(
    conn: web::Data<Mutex<SqliteConnection>>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_id = id.to_owned();
    let mut conn = conn.lock().unwrap(); // Lock the Mutex to access the connection
    let user = db::find_user_by_uid(&mut conn, user_id)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error":404,
                "message":format!("No user found with phone:{id}")
            })
            .to_string(),
        );
        Ok(res)
    }
}

#[get("/conversations/{uid}")]
pub async fn get_conversation_by_id(
    conn: web::Data<Mutex<SqliteConnection>>,
    uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let room_id = uid.to_owned();
    let mut conn = conn.lock().unwrap(); // Lock the Mutex to access the connection
    let conversations = db::get_conversation_by_room_uid(&mut conn, room_id)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    if let Some(data) = conversations {
        Ok(HttpResponse::Ok().json(data))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error":404,
                "message":format!("No conversation with room_id:{room_id}")
            })
            .to_string(),
        );
        Ok(res)
    }
}

#[get("/users/phone/{user_phone}")]
pub async fn get_user_by_phone(
    conn: web::Data<Mutex<SqliteConnection>>,
    phone: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let user_phone = phone.to_string();
    let mut conn = conn.lock().unwrap(); // Lock the Mutex to access the connection
    let user = db::find_user_by_phone(&mut conn, user_phone)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error":404,
                "message":format!("No user found with phone:{}", phone.to_string()),
            })
            .to_string(),
        );
        Ok(res)
    }
}

#[get("/rooms")]
pub async fn get_rooms(conn: web::Data<Mutex<SqliteConnection>>) -> Result<HttpResponse, Error> {
    let mut conn = conn.lock().unwrap(); // Lock the Mutex to access the connection
    let rooms = db::get_all_rooms(&mut conn).map_err(actix_web::error::ErrorInternalServerError)?;
    if !rooms.is_empty() {
        Ok(HttpResponse::Ok().json(rooms))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error":404,
                "message":"No rooms available at the moment."
            })
            .to_string(),
        );
        Ok(res)
    }
}
