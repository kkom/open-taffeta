// this file maps the DB tables as Rust structures
// Meaning of derives:
// (De)Serialize => can be used to (de)serialize data in JSON
// Queryable => can be used to represent entities in result set generated by queries
// AsChangeSet + Indentifiable => for `.save_changes()`
// ... there are many more ...

use serde_derive::{Serialize, Deserialize};
use crate::schema::users;
use crate::auth::Auth;

#[derive(Queryable, Clone, Serialize, Deserialize, Debug, Default, Insertable, AsChangeset, Identifiable, PartialEq)]
pub struct User {
    pub id: i32,
    pub password: String,
    pub email: String,
    pub is_active: bool
}

#[derive(Queryable, Serialize, Deserialize, Debug, Default)]
pub struct Door {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub buzzer_url: String,
    pub ring: bool,
    pub ring_ts: Option<i32>,
}

impl User {
    // generate tokens for signup + logins
    pub fn to_auth(&self) -> Auth {
        Auth::new(self.id, &self.email)
    }
}
