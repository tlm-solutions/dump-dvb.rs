use crate::schema::*;

use log::warn;
use pbkdf2::{
    password_hash::{Encoding, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use uuid::Uuid;

use diesel::{AsExpression, Insertable, Queryable};
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, Output, ToSql};

/// Enum representing the role a user has inside our systems. Values are pretty self-explanatory
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug, AsExpression)]
#[diesel(sql_type = diesel::sql_types::Integer)]
#[allow(missing_docs)]
pub enum Role {
    EditCompanyStations = 0,
    CreateCompanyStations = 1,
    DeleteCompanyStations = 2,
    EditMaintainedStations = 3,
    CreateMaintainedStations = 4,
    DeleteMaintainedStations = 5,
    EditOrgUserRoles = 6,
    EditOwnCompany = 7,
    ApproveStations = 8,
}

impl TryFrom<i32> for Role {
    type Error = &'static str;
    fn try_from(role: i32) -> Result<Self, Self::Error> {
        match role {
            0 => Ok(Role::EditCompanyStations),
            1 => Ok(Role::CreateCompanyStations),
            2 => Ok(Role::DeleteCompanyStations),
            3 => Ok(Role::EditMaintainedStations),
            4 => Ok(Role::CreateMaintainedStations),
            5 => Ok(Role::DeleteMaintainedStations),
            6 => Ok(Role::EditOrgUserRoles),
            7 => Ok(Role::EditOwnCompany),
            8 => Ok(Role::ApproveStations),
            _ => Err("No role corresponding to {role} value!"),
        }
    }
}

impl From<Role> for i32 {
    fn from(val: Role) -> Self {
        match val {
            Role::EditCompanyStations => 0,
            Role::CreateCompanyStations => 1,
            Role::DeleteCompanyStations => 2,
            Role::EditMaintainedStations => 3,
            Role::CreateMaintainedStations => 4,
            Role::DeleteMaintainedStations => 5,
            Role::EditOrgUserRoles => 6,
            Role::EditOwnCompany => 7,
            Role::ApproveStations => 8,
        }
    }
}

impl FromSql<diesel::sql_types::Integer, Pg> for Role {
    fn from_sql(bytes: diesel::backend::RawValue<'_, Pg>) -> deserialize::Result<Self> {
        let v: i32 = i32::from_sql(bytes)?;
        let res: Self = v.try_into()?;
        Ok(res)
    }
}

impl ToSql<diesel::sql_types::BigInt, Pg> for Role {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
            Role::EditCompanyStations => <i32 as ToSql<diesel::sql_types::Integer, Pg>>::to_sql(&0_i32, out),
            Role::CreateCompanyStations => <i32 as ToSql<diesel::sql_types::Integer, Pg>>::to_sql(&1_i32, out),
            Role::DeleteCompanyStations => <i32 as ToSql<diesel::sql_types::Integer, Pg>>::to_sql(&2_i32, out),
            Role::EditMaintainedStations => <i32 as ToSql<diesel::sql_types::Integer, Pg>>::to_sql(&3_i32, out),
            Role::CreateMaintainedStations => <i32 as ToSql<diesel::sql_types::Integer, Pg>>::to_sql(&4_i32, out),
            Role::DeleteMaintainedStations => <i32 as ToSql<diesel::sql_types::Integer, Pg>>::to_sql(&5_i32, out),
            Role::EditOrgUserRoles => <i32 as ToSql<diesel::sql_types::Integer, Pg>>::to_sql(&6_i32, out),
            Role::EditOwnCompany => <i32 as ToSql<diesel::sql_types::Integer, Pg>>::to_sql(&7_i32, out),
            Role::ApproveStations => <i32 as ToSql<diesel::sql_types::Integer, Pg>>::to_sql(&8_i32, out),
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
    /// This value is interesting for newsletters and other notifications that are distributed via
    /// mail.
    pub email_setting: Option<i32>,
    /// If the user struct is deleted is kept for database consistency.
    pub deactivated: bool,
    /// If user is tlms-wide administrator
    pub admin: bool,
}

/// Database struct holding the relations between organizations and users. Keeps track of user
/// roles within organization
#[derive(Debug, Clone, Deserialize, Queryable, Insertable)]
#[diesel(table_name = org_users_relation)]
pub struct OrgUsersRelation {
    /// Primary key
    id: Uuid,
    /// For which org the role is set
    organisation: Uuid,
    /// For which user within org the role is set
    user_id: Uuid,
    /// The role itself, see [`Roles`] enum for possible values
    role: Role,
}

/// The UUID of special "community" organization, which is used for crowdsourced stations
pub const COMMUNITY_ORG_ID: Uuid = Uuid::from_u128(0x53e643d7_c300_4de7_ab48_540d08a0cbc6);

/// Database struct holding the information about organizations
#[derive(Debug, Clone, Deserialize, Queryable, Insertable)]
#[diesel(table_name = organization)]
pub struct Organization {
    /// Primary Key
    id: Uuid,
    /// Company Name
    name: String,
    /// If Company information is public
    public: bool,
}

/// custom serializer so we dont accidentailly leak password to the outside
impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("User", 5)?;
        s.serialize_field("id", &self.id.to_string())?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("email", &self.email)?;
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
