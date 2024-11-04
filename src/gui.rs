use rfd::MessageDialog;
use crate::diagnostics::Diagnostics;

pub fn run_gui() {
    let file_path = "vehicle_data.json"; // Example file path
    let vehicle_data = Diagnostics::retrieve_diagnostics(file_path);
    MessageDialog::new()
        .set_title("OBD-II Diagnostics")
        .set_description(&format!("VIN: {}\nRPM: {}\nSpeed: {}\nCoolant Temp: {}°C",
            vehicle_data.vin, vehicle_data.engine_data.rpm,
            vehicle_data.engine_data.speed, vehicle_data.engine_data.coolant_temp))
        .show();
}
