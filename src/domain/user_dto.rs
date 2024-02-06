use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDto {
    id: Option<i32>,
    name: Option<String>,
    email:Option<String>,
}
#[derive(Debug)]
pub enum UserField <'a>{
    Id(i32),
    Name( &'a str),
    Email(&'a str),
    IdIsNotSet,
    NameIsNotSet,
    EmailIsNotSet,
}
impl UserDto{

    pub fn new(name: String, email: String) -> UserDto {
        Self {
            id: None,
            name: Some(name),
            email: Some(email),
        }
    }
    pub fn get_id(&self) -> UserField {
        match &self.id {
            Some(id) => UserField::Id(*id),
            None => {
                println!("Id is not set");
                UserField::IdIsNotSet
            }
        }
    }
    pub fn get_name(&self) -> UserField {
        match &self.name {
            Some(name) => UserField::Name(name),
            None => {
                println!("Name is not set");
                UserField::NameIsNotSet
            }
        }
    }
    pub fn get_email(&self) -> UserField {
        match &self.email{
            Some(email) => UserField::Email(email),
            None => {
                println!("Email is not set");
                UserField::EmailIsNotSet
            },
        }
    }
    pub fn set_id(mut self, id: i32)->Self {
        self.id = Some(id);
        self
    }
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }
    pub fn set_email(&mut self, email: String) -> &mut Self{
        self.email = Some(email);
        self
    }
}
