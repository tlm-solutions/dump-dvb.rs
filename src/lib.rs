#[macro_use]
extern crate diesel;
extern crate diesel_codegen;
extern crate dotenv;
extern crate num_derive;

pub mod telegrams;
pub mod schema;
pub mod locations;
pub mod management;
pub mod receivers;
pub mod measurements;
