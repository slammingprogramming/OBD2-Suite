mod config;
mod data_parser;
mod diagnostics;
mod gui;
mod obd_protocol;
mod real_time_monitor;
mod report_generator;
mod vehicle_data;

use gui::run_gui;

fn main() {
    run_gui();
}
