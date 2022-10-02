use crate::schema::*;

use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use uuid::Uuid;
use pbkdf2::{
    password_hash::{Encoding, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use log::warn;
use regex::Regex;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum Role {
    Trekkie = 9,
    User = 6,
    Administrator = 0,
}

impl Role {
    pub fn from(role: i32) -> Role {
        match role {
            0 => Role::Administrator,
            _ => Role::User,
        }
    }

    pub fn as_int(&self) -> i32 {
        match self {
            Role::Administrator => 0,
            _ => 6,
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
    pub password: Option<String>,
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

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterUserRequest {
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String
}


impl RegisteredUser {
    pub fn is_admin(&self) -> bool {
        Role::from(self.role) == Role::Administrator
    }

    pub fn from(user: &User) -> Option<RegisteredUser> {
        if user.name.is_none() || user.email.is_none() || user.password.is_none() || user.email_setting.is_none() {
            return None;
        }
        Some(RegisteredUser {
            id: user.id,
            name: user.name.clone().unwrap(),
            email: user.email.clone().unwrap(),
            password: user.password.clone().unwrap(),
            email_setting: user.email_setting.clone().unwrap(),
            role: user.role,
            deactivated: user.deactivated
        })
    }
}


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

pub fn verify_password(password: &String, hashed_password: &String) -> bool {
    let password_hash = match PasswordHash::parse(hashed_password, Encoding::B64) {
        Ok(data) => data,
        Err(e) => {
            warn!("cannot hash password with error {:?}", e);
            return false;
        }
    };
    match Pbkdf2.verify_password(password.as_bytes(), &password_hash) {
        Ok(_) => true,
        Err(_) => false
    }
}

impl User {
    pub fn new(request: &RegisterUserRequest) -> Option<RegisteredUser> {
        let email_regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .unwrap();

        if !email_regex.is_match(&request.email) ||
            request.name.is_empty() ||
            request.password.len() < 8 {
            return None;
        }

        let password_hash;
        match hash_password(&request.password) {
            Some(hashed_password) => {
                password_hash = hashed_password;
            }
            None => {
                warn!("User did not supply hashable password");
                return None;
            }
        }

        Some(RegisteredUser{
            id: Uuid::new_v4(),
            name: request.name.clone(),
            email: request.email.clone(),
            password: password_hash,
            role: Role::User.as_int(),
            deactivated: false,
            email_setting: 0,
        })
    }
}


