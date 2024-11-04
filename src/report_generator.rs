use crate::data_parser::VehicleData;
use std::fs::File;
use std::io::Write;

pub struct ReportGenerator;

impl ReportGenerator {
    pub fn generate_report(vehicle_data: &VehicleData) {
        let report_content = format!(
            "Vehicle Report\nVIN: {}\nDTCs: {:?}\nRPM: {}\nSpeed: {}\nCoolant Temp: {}°C",
            vehicle_data.vin,
            vehicle_data.dtcs,
            vehicle_data.engine_data.rpm,
            vehicle_data.engine_data.speed,
            vehicle_data.engine_data.coolant_temp,
        );

        let mut file = File::create("diagnostic_report.txt").expect("Unable to create file");
        file.write_all(report_content.as_bytes()).expect("Unable to write data");
    }
}