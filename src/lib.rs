#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate num_derive;

pub mod telegrams;
pub mod schema;
pub mod stations;
pub mod receivers;
pub mod measurements;
