use serde::{Serialize, Deserialize};
use serde_json;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveInfo {
    pub nome: String,
    pub cpf: u64,
    pub data: DateTime<Utc>,
}

impl SaveInfo {
    pub fn save(info: SaveInfo) {
        let json = serde_json::to_string_pretty(&info).unwrap();
        println!("{:?}", json);
    }
}
