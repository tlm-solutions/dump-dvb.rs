//! This module holds structs and associated stuff for storing the region metadata ([`Region`] and
//! [`InsertRegion`], as well as some region-related structs that are employed for caching the
//! region data.

use crate::schema::*;
use crate::telegrams::r09::R09Type;
use serde::{Deserialize, Serialize};

/// Struct holding the information for a region.
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Region {
    /// Unique region identifier this is really just an arbitrery number.
    #[diesel(deserialize_as = i64)]
    pub id: i64,
    /// Name of the region / city
    pub name: String,
    /// Name of the operator in the city e.g DVB.
    pub transport_company: String,
    /// Name of the Regional operator e.g. VVO (Verkehrs Verbund Oberelbe)
    /// which encompasses the transport_companty
    pub regional_company: Option<String>,
    /// The frequency the operator sends it VDV 420 traffic.
    pub frequency: Option<i64>,
    /// Which R09 types are used look at [`R09Type`][crate::telegrams::r09::R09Type] for possible
    /// values
    pub r09_type: Option<R09Type>,
    /// Which encoding this regions uses. Look at [`Encoding`] for possible values.
    pub encoding: Option<i32>,
    /// This value is set to true if the region is deleted.
    pub deactivated: bool,
}

/// This struct is the same as [`Region`] but with the difference that id is optional
/// this is required to use the auto increment function from postgres
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = regions)]
pub struct InsertRegion {
    /// Unqiue region identifier which is nullable to let postgres set a value for us.
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,
    /// Name of the region / city
    pub name: String,
    /// Name of the operator in the city e.g DVB.
    pub transport_company: String,
    /// Name of the Regional operator e.g. VVO (Verkehrs Verbund Oberelbe)
    /// which encompasses the transport_companty
    pub regional_company: Option<String>,
    /// The frequency the operator sends it VDV 420 traffic.
    pub frequency: Option<i64>,
    /// Which R09 types are used look at [`R09Type`][crate::telegrams::r09::R09Type] for possible
    /// values
    pub r09_type: Option<R09Type>,
    /// Which encoding this regions used look at [`Encoding`] for possible values.
    pub encoding: Option<i32>,
    /// This value is set to true if the region is deleted.
    pub deactivated: bool,
}
