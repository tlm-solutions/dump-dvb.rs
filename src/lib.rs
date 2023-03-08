//! 
//! TLMS.rs is the crate for the Transit Live Mapping Solutions. 
//!
//! **Contact:** <hello@tlm.solutions>
//!
//! This crate exports a lot of structs, schemas and functionality that is used by the project and 
//! its services and programs.
//!

#![deny(missing_docs)]
#[macro_use]

/// Crate includes so we can access the macros 
extern crate diesel;
extern crate dotenv;
extern crate num_derive;

///
/// VDV 420 Telegram Definitions
///
#[cfg(feature = "telegrams")]
pub mod telegrams;

///
/// Rust Diesel Database Schema Definitions
///
#[cfg(feature = "schema")]
pub mod schema;

/// 
/// This module exports information about locations this includes region informatiom but also 
/// mappings from reporting point to gps position.
///
#[cfg(feature = "locations")]
pub mod locations;

/// 
/// Exports the configuration for a Station / Receiver
///
#[cfg(feature = "receivers")]
pub mod receivers;

///
/// This module / feature exports structs that are required for managing the services like User,
/// Station, Region and a lot of Enums that are used like AntennaType or Architecture of the
/// Station.
///
#[cfg(feature = "management")]
pub mod management;

/// 
/// This module / feature exports structs that are used for measurements that are then later
/// submitted to trekkie.
///
#[cfg(feature = "measurements")]
pub mod measurements;

///
/// Trekkie is a service which receives live gps positions, recorded gps measurements 
/// and correlates them with the r09_telegrams to generate the stops mapping.
///
/// This module exports the database structures how trekkie measurements runs are saved in postgres
///
#[cfg(feature = "trekkie")]
pub mod trekkie;
