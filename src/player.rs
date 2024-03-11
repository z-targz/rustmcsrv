use uuid::Uuid;
use crate::server::Connection;

pub struct Player {
    name: String,
    uuid: Uuid,
}