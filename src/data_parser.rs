pub fn parse_dtc(data: &[u8]) -> Option<u32> {
    if data.len() >= 3 {
        Some(u32::from_be_bytes([data[0], data[1], data[2], 0]))
    } else {
        None
    }
}

pub fn interpret_dtc(code: u32) -> String {
    format!("Diagnostic Trouble Code {} interpretation", code)
}
