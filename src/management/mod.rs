/// This module contains user structs and security functions.
pub mod user;

use crate::locations::region::Region;
use crate::schema::*;
use user::User;

use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use utoipa::ToSchema;
use uuid::Uuid;

/// This is the struct for a station / receiver which receives VDV420 R09 Telegrams and sends them
/// to [data-accumulator](https://github.com/tlm-solutions/data-accumulator) for collection and
/// further processing. This struct is used for token based authentication inside data-accumulator.
#[derive(Debug, Clone, Deserialize, Insertable, Queryable, Associations, ToSchema)]
#[diesel(table_name = stations)]
#[diesel(belongs_to(User, foreign_key = owner))]
#[diesel(belongs_to(Region, foreign_key = region))]
pub struct Station {
    /// Unique identifier for a station.
    pub id: Uuid,
    /// Secret token for a station which is send with every telegram for authentication.
    pub token: Option<String>,
    /// Name of the Station.
    pub name: String,
    /// Latitude of the Station.
    pub lat: f64,
    /// Longtitude of the Station.
    pub lon: f64,
    /// In which region the station is located.
    pub region: i64,
    /// Uuid of the owner of the station referecing a [`User`]
    pub owner: Uuid,
    /// If the station is approved to submit data into the system.
    pub approved: bool,
    /// Tells if the station is deleted or not. Is keept around for database consistentcy.
    pub deactivated: bool,
    /// If the station is public so information is this struct can be shared. Data from this
    /// station is shared regardless of this flag.
    pub public: bool,
    /// Radio enum helps users to keep track which radio was put into the Station take a look at
    /// [`Radio`] enum for more information.
    pub radio: Option<i32>,
    /// Which Processor Architecture the Station has look at [`Architecture`] for more information.
    pub architecture: Option<i32>,
    /// On which computer model the station runs on look at [`Device`] for more information.
    pub device: Option<i32>,
    /// Optional value to specify on which elevation the station / antenna is located.
    pub elevation: Option<f64>,
    /// Antenna type the station uses look at [`Antenna`] for more information.
    pub antenna: Option<i32>,
    /// Which telegram-decoder-version runs on the station.
    pub telegram_decoder_version: Option<String>,
    /// Field to add custom notes to your station.
    pub notes: Option<String>,
    /// Organization that this station belongs to.
    pub organization: Uuid,
}

impl Serialize for Station {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Station", 15)?;

        s.serialize_field("id", &self.id)?;
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
        s.serialize_field("notes", &self.notes)?;
        s.end()
    }
}

/// On which computer / device the station runs on.
#[derive(Serialize, Deserialize, ToSchema)]
pub enum Device {
    /// Unknown or Unlisted Device
    Other = 0,
    /// Enum variant for Raspberry Pi 3
    Raspberry3 = 1,
    /// Enum variant for Raspberry Pi 3b
    Raspberry3b = 2,
    /// Enum variant for Raspberry Pi 3b+
    Raspberry3bPlus = 3,
    /// Enum variant for Raspberry Pi 4
    Raspberry4 = 4,
    /// Enum variant for Odroid C1
    OdroidC1 = 5,
    /// Enum variant for Odroid C2
    OdroidC2 = 6,
    /// Enum variant for Odroid C4
    OdroidC4 = 7,
    /// Enum variant for Odroid N2
    OdroidN2 = 8,
    /// Enum variant for Odroid U2
    OdroidU2 = 9,
    /// Enum variant for Odroid U3
    OdroidU3 = 10,
    /// Enum variant for Pine H64
    PineH64 = 11,
    /// Enum variant for Pine Rock 64
    PineRock64 = 12,
    /// Enum variant for Dell Wyse 3040
    DellWyse3040 = 13,
}

/// Which Software Defined Radio is used by the station.
#[derive(Serialize, Deserialize, ToSchema)]
pub enum Radio {
    /// Unknown or Unlisted Device
    Other = 0,
    /// Enum variant for Hack Rf Radios
    HackRf = 1,
    /// Enum variant for RTL SDRs
    RTLSDR = 2,
    /// Enum variant for NES SDRs
    NESDR = 3,
}

/// Which CPU Architecture is used by the device the station runs on.
#[derive(Serialize, Deserialize, ToSchema)]
pub enum Architecture {
    /// Unknown or Unlisted Architecture
    Other = 0,
    /// X86 Maschine
    X86 = 1,
    /// ARM 8 64 Bit Maschine
    Aarch64 = 2,
}

/// Enum that encodes antenna types with which r09 telegrams are captured.
#[derive(Serialize, Deserialize, ToSchema)]
pub enum Antenna {
    /// Unknown or Unlisted Antenna Type
    Other = 0,
    /// Enum variant for Di-Pole Antennas
    Dipole = 1,
    /// Enum variant for Groundplane Antennas
    GroundPlane = 2,
    /// Enum variant for Yagi Antennas
    Yagi = 3,
}

/// With which encoding the data inside the r09 telegrams is encoded.
#[derive(Serialize, Deserialize, ToSchema)]
pub enum Encoding {
    /// Unknown or Unlisted Data Encoding
    Other = 0,
    /// Enum variant for On-Off-Keying
    OnOffKeying = 1,
    /// Enum variant for Nemo encoding
    Nemo = 2,
}

/// function that takes a device enums and converts it into the corresponding string
pub fn device_to_string(device: &Device) -> String {
    match device {
        Device::Raspberry3 => "rpi3".to_string(),
        Device::Raspberry3b => "rpi3".to_string(),
        Device::Raspberry3bPlus => "rpi3".to_string(),
        Device::Raspberry4 => "rpi4".to_string(),
        Device::DellWyse3040 => "dell-wyse-3040".to_string(),
        _ => "other".to_string(),
    }
}

/// function that takes a architecture enum and converts it into the corresponding string
pub fn arch_to_string(arch: &Architecture) -> String {
    match arch {
        Architecture::X86 => "x86_64-linux".to_string(),
        Architecture::Aarch64 => "aarch64-linux".to_string(),
        Architecture::Other => "other".to_string(),
    }
}
