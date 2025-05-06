use super::device::DeviceInfo;
use windows::{
    core::*,
    Devices::{Bluetooth::BluetoothLEDevice, Enumeration::DeviceInformation},
};

pub async fn get_device_info() -> Result<Vec<DeviceInfo>> {
    let selector = BluetoothLEDevice::GetDeviceSelector()?;
    let devices_op = DeviceInformation::FindAllAsyncAqsFilter(&selector)?;
    let devices = devices_op.await?;

    let mut res = vec![];
    for i in 0..devices.Size()? {
        let dev = devices.GetAt(i)?;
        let info = DeviceInfo {
            name: dev.Name()?.to_string(),
            address: dev.Id()?.to_string(),
        };
        res.push(info);
    }

    Ok(res)
}
