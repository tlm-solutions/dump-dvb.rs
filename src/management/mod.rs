use crate::schema::*;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum Role {
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

#[derive(Debug, Clone, Deserialize, Insertable, Queryable)]
#[table_name = "users"]
pub struct User {
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

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "regions"]
pub struct Region {
    pub id: Option<i32>,
    pub name: String,
    pub transport_company: String,
    pub regional_company: Option<String>,
    pub frequency: Option<i64>,
    pub r09_type: Option<i32>,
    pub encoding: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Insertable, Queryable, Associations)]
#[table_name = "stations"]
#[belongs_to(User, foreign_key = "owner")]
#[belongs_to(Region, foreign_key = "region")]
pub struct Station {
    pub id: Uuid,
    pub token: Option<String>,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub region: i32,
    pub owner: Uuid,
    pub approved: bool,
    pub deactivated: bool,
    pub public: bool,
    pub radio: Option<i32>,
    pub architecture: Option<i32>,
    pub device: Option<i32>,
    pub elevation: Option<f64>,
    pub telegram_decoder_version: Option<Vec<i32>>,
    pub antenna: Option<i32>,
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("User", 7)?;
        s.serialize_field("id", &self.id.to_string())?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("email", &self.email)?;
        s.serialize_field("role", &self.role)?;
        s.serialize_field("email_setting", &self.email_setting)?;
        s.serialize_field("deactivated", &self.deactivated)?;
        s.end()
    }
}

impl Serialize for Station {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Station", 17).unwrap();

        s.serialize_field("id", &self.id)?;
        s.serialize_field("token", &self.token)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("lat", &self.lat)?;
        s.serialize_field("lon", &self.lon)?;
        s.serialize_field("region", &self.region)?;
        s.serialize_field("owner", &self.owner.to_string())?;
        s.serialize_field("approved", &self.approved)?;
        s.serialize_field("deactivated", &self.deactivated)?;
        s.serialize_field("public", &self.public)?;
        s.serialize_field("radio", &self.radio)?;
        s.serialize_field("architecture", &self.architecture)?;
        s.serialize_field("device", &self.device)?;
        s.serialize_field("elevation", &self.elevation)?;
        s.serialize_field("telegram_decoder_version", &self.telegram_decoder_version)?;
        s.serialize_field("antenna", &self.antenna)?;
        s.end()
    }
}

#[derive(Serialize, Deserialize)]
pub enum Device {
    Other = 0,
    Raspberry3 = 1,
    Raspberry3b = 2,
    Raspberry3bPlus = 3,
    Raspberry4 = 4,
    OdroidC1 = 5,
    OdroidC2 = 6,
    OdroidC4 = 7,
    OdroidN2 = 8,
    OdroidU2 = 9,
    OdroidU3 = 10,
    PineH64 = 11,
    PineRock64 = 12,
    DellWyse3040 = 13,
}

#[derive(Serialize, Deserialize)]
pub enum Radio {
    Other = 0,
    HackRf = 1,
    RTLSDR = 2,
    NESDR = 3,
}

#[derive(Serialize, Deserialize)]
pub enum Architecture {
    Other = 0,
    X86 = 1,
    Aarch64 = 2,
}
