mod obd_protocol;
mod data_parser;
mod gui;
mod report_generator;

use obd_protocol::OBDInterface;
use gui::launch_gui;

#[tokio::main]
async fn main() {
    println!("Universal OBD-II Debugging Suite Initializing...");
    let obd_interface = OBDInterface::new();
    
    // Initialize GUI
    launch_gui(&obd_interface);
}
