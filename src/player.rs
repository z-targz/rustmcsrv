use uuid::Uuid;
use crate::server::Connection;

pub struct Player<'a> {
    name: String,
    uuid: Uuid,
    connection: &'a Connection<'a>,
    data: PlayerData,
    test: i32,
}

impl<'a> Player<'a> {

}

pub struct PlayerData {
    test: String,
    test2: i32,
}

impl PlayerData {
    fn get_test(&self) -> &String {
        &self.test
    }
    fn get_test2(&self) -> &i32 {
        &self.test2
    }
    fn asdf(data: PlayerData) {
        data.get_test2().to_owned();
    }
}