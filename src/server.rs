use crate::session;
use actix::prelude::*;
use rand::{self, Rng, rngs::ThreadRng};
use serde_json::json;
use std::collections::{HashMap, HashSet};
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: usize,
    pub msg: String,
    pub room: String,
}
pub struct ListRooms;
impl actix::Message for ListRooms {
    type Result = Vec<String>;
}
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub id: usize,
    pub name: String,
}
