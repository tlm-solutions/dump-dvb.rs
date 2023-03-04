use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RadioReceiver {
    pub id: Uuid,
    pub name: String,
    pub region: u32,
    pub lat: f64,
    pub lon: f64,
}
