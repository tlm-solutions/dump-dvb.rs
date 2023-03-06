use crate::schema::*;

use log::warn;
use pbkdf2::{
    password_hash::{Encoding, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Role {
    Trekkie = 9,
    User = 6,
    Administrator = 0,
    Unknown = 64,
}

impl From<i32> for Role {
    fn from(role: i32) -> Self {
        match role {
            0 => Role::Administrator,
            6 => Role::User,
            9 => Role::Trekkie,
            _ => Role::Unknown,
        }
    }
}

impl From<Role> for i32 {
    fn from(val: Role) -> Self {
        match val {
            Role::Administrator => 0,
            Role::User => 6,
            Role::Trekkie => 9,
            Role::Unknown => 9,
        }
    }
}

/// Schema implementation
#[derive(Debug, Clone, Deserialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: String,
    pub role: i32,
    pub email_setting: Option<i32>,
    pub deactivated: bool,
}

/// Minimal User mainly used for trekkie
#[derive(Debug, Clone, Deserialize, Queryable)]
#[diesel(table_name = users)]
pub struct MinimalUser {
    pub id: Uuid,
    pub role: i32,
    pub deactivated: bool,
}

/// Fully Registered User created by clicky-bunty server
#[derive(Debug, Clone, Deserialize, Queryable)]
#[diesel(table_name = users)]
pub struct RegisteredUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: i32,
    pub email_setting: i32,
    pub deactivated: bool,
}

impl User {
    pub fn is_admin(&self) -> bool {
        Role::from(self.role) == Role::Administrator
    }
}

impl RegisteredUser {
    pub fn is_admin(&self) -> bool {
        Role::from(self.role) == Role::Administrator
    }

    pub fn from_user(user: &User) -> Option<RegisteredUser> {
        if user.name.is_none() || user.email.is_none() || user.email_setting.is_none() {
            return None;
        }
        Some(RegisteredUser {
            id: user.id,
            name: user.name.clone().unwrap(),
            email: user.email.clone().unwrap(),
            password: user.password.clone(),
            email_setting: user.email_setting.unwrap(),
            role: user.role,
            deactivated: user.deactivated,
        })
    }
}

/// custom serializer so we dont accidentailly leak password to the outside
impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("User", 6)?;
        s.serialize_field("id", &self.id.to_string())?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("email", &self.email)?;
        s.serialize_field("role", &self.role)?;
        s.serialize_field("email_setting", &self.email_setting)?;
        s.serialize_field("deactivated", &self.deactivated)?;
        s.end()
    }
}

pub fn hash_password(password: &String) -> Option<String> {
    let default_salt_path = String::from("/run/secrets/clicky_bunty_salt");
    let salt_path = std::env::var("SALT_PATH").unwrap_or(default_salt_path);
    let salt = SaltString::b64_encode(std::fs::read(salt_path).unwrap().as_slice()).unwrap();

    match Pbkdf2.hash_password(password.as_bytes(), &salt) {
        Ok(password_hash) => PasswordHash::new(&password_hash.to_string())
            .map(|x| x.to_string())
            .ok(),
        Err(e) => {
            warn!("Unable to hash password: {} with error {:?}", password, e);
            None
        }
    }
}

pub fn verify_password(password: &String, hashed_password: &str) -> bool {
    let password_hash = match PasswordHash::parse(hashed_password, Encoding::B64) {
        Ok(data) => data,
        Err(e) => {
            warn!("cannot hash password with error {:?}", e);
            return false;
        }
    };
    Pbkdf2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok()
}
