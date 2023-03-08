//!
//! TLMS.rs is the crate for the Transit Live Mapping Solutions.
//!
//! **Contact:** <hello@tlm.solutions>
//!
//! This crate exports a lot of structs, schemas and functionality that is used by the project and
//! its services and programs.
//!

#![deny(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
#[macro_use]
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
/// This module exports information about locations. This includes region information and mappings
/// from reporting point to gps position.
///
#[cfg(feature = "locations")]
pub mod locations;

///
/// Exports the configuration for a Station a.k.a. Receiver
///
#[cfg(feature = "receivers")]
pub mod receivers;

///
/// This module exports structs like User, Station, and Region that are required for managing the
/// services. The module also contains Enums like AntennaType or Architecture that are used to
/// describe the Station.
///
#[cfg(feature = "management")]
pub mod management;

///
/// This module exports structs that are used by trekkie service for processing and storing
/// measurements.
///
#[cfg(feature = "measurements")]
pub mod measurements;

///
/// Trekkie is a service which receives live gps positions, recorded gps measurements.
/// and correlates them with the r09_telegrams to generate the stops mapping.
///
/// This module exports the database structures how trekkie measurements runs are saved in postgres
///
#[cfg(feature = "trekkie")]
pub mod trekkie;
