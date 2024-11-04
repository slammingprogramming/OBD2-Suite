pub enum OBDProtocol {
    ISO9141_2,
    ISO14230_4,
    ISO15765_4,
    SAE_J1850_PWM,
    SAE_J1850_VPW,
}

impl OBDProtocol {
    pub fn as_str(&self) -> &str {
        match self {
            OBDProtocol::ISO9141_2 => "ISO 9141-2",
            OBDProtocol::ISO14230_4 => "ISO 14230-4 (KWP2000)",
            OBDProtocol::ISO15765_4 => "ISO 15765-4 (CAN-BUS)",
            OBDProtocol::SAE_J1850_PWM => "SAE J1850 PWM",
            OBDProtocol::SAE_J1850_VPW => "SAE J1850 VPW",
        }
    }
}
