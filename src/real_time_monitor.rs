pub struct RealTimeMonitor;

impl RealTimeMonitor {
    pub fn display_sensor_data(rpm: u32, speed: u32, coolant_temp: f32) {
        println!("Current RPM: {}", rpm);
        println!("Current Speed: {} km/h", speed);
        println!("Coolant Temperature: {} °C", coolant_temp);
    }
}
