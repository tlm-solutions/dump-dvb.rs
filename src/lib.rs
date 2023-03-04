#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate num_derive;

#[cfg(feature = "telegrams")]
pub mod telegrams;

#[cfg(feature = "schema")]
pub mod schema;

#[cfg(feature = "locations")]
pub mod locations;

#[cfg(feature = "management")]
pub mod management;

#[cfg(feature = "measurements")]
pub mod measurements;

#[cfg(feature = "trekkie")]
pub mod trekkie;
