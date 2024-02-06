use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: i32,
    name: String,
    email: String,
}
 impl User {
    pub fn new(id:i32, name: String, email: String) -> User {
        User { id, name, email }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_email(&self) -> &str {
        &self.email
    }
}
