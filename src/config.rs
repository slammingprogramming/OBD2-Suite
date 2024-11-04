pub struct Config {
    pub adapter_type: String,
    pub supported_protocols: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            adapter_type: "USB".to_string(),
            supported_protocols: vec![
                "ISO 9141-2".to_string(),
                "ISO 14230-4 (KWP2000)".to_string(),
                "ISO 15765-4 (CAN-BUS)".to_string(),
                "SAE J1850 PWM".to_string(),
                "SAE J1850 VPW".to_string(),
            ],
        }
    }
}
