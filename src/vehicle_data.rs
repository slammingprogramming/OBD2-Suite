pub struct VehicleData {
    pub vin: String,
    pub dtcs: Vec<String>,
    pub engine_data: EngineData,
}

pub struct EngineData {
    pub rpm: u32,
    pub speed: u32,
    pub coolant_temp: f32,
}