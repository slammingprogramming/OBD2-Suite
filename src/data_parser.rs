use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct VehicleData {
    pub vin: String,
    pub dtcs: Vec<String>,
    pub engine_data: EngineData,
}

#[derive(Deserialize)]
pub struct EngineData {
    pub rpm: u32,
    pub speed: u32,
    pub coolant_temp: f32,
}

pub fn parse_vehicle_data(file_path: &str) -> VehicleData {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    serde_json::from_str(&data).expect("JSON was not well-formatted")
}
