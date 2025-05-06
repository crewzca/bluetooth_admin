use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DeviceInfo {
    pub address: String,
    pub name: String,
}
