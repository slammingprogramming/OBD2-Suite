use crate::data_parser::{parse_vehicle_data, VehicleData};

pub struct Diagnostics;

impl Diagnostics {
    pub fn retrieve_diagnostics(file_path: &str) -> VehicleData {
        parse_vehicle_data(file_path)
    }

    pub fn interpret_dtc(dtcs: Vec<String>) -> Vec<String> {
        dtcs.iter()
            .map(|code| format!("DTC: {}, Interpretation: [Details about {}]", code, code))
            .collect()
    }
}
