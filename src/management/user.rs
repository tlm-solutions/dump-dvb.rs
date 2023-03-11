use crate::schema::*;

use log::warn;
use pbkdf2::{
    password_hash::{Encoding, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use uuid::Uuid;

/// Enum representing the role a user has inside our systems.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Role {
    /// Unkown Role is a fallback for errors and has 0 privileges
    Unknown = 64,
    /// This Role is given to users which use the stasi app so they can submit data to trekkie
    Trekkie = 9,
    /// The Default user role u can manage create stations and trekkie runs.
    User = 6,
    /// Admin can do everything.
    Administrator = 0,
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

/// Database struct holding user information
#[derive(Debug, Clone, Deserialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    /// Unique identifier for a user.
    pub id: Uuid,
    /// Name of the user.
    pub name: Option<String>,
    /// Email of the user.
    pub email: Option<String>,
    /// Password of the user.
    pub password: String,
    /// Which role the user has inside the system take a look at [`Role`] for more information.
    pub role: i32,
    /// This value is interesting for newsletters and other notifications that are distributed via
    /// mail.
    pub email_setting: Option<i32>,
    /// If the user struct is deleted is kept for database consistency.
    pub deactivated: bool,
}

impl User {
    /// Returns if the user has role admin.
    pub fn is_admin(&self) -> bool {
        Role::from(self.role) == Role::Administrator
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

/// Function that takes the plain text passwords and returns the corresponding pbkdf2 hash.
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

/// Function that takes plain text passwords and the pbkdf2 hash from the database and returns true
/// if the they correspond to the same password.
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
