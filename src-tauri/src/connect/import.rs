use super::device::DeviceInfo;
use windows::{
    core::*,
    Devices::{
        Bluetooth::{BluetoothDevice, BluetoothLEDevice},
        Enumeration::DeviceInformation,
    },
};

pub async fn get_device_info() -> Result<Vec<DeviceInfo>> {
    let selectors = vec![
        BluetoothLEDevice::GetDeviceSelector()?,
        BluetoothDevice::GetDeviceSelector()?,
    ];
    let mut res = vec![];

    for selector in selectors.iter() {
        let devices_op = DeviceInformation::FindAllAsyncAqsFilter(&selector)?;
        let devices = devices_op.await?;

        for i in 0..devices.Size()? {
            let dev = devices.GetAt(i)?;
            let info = DeviceInfo {
                name: dev.Name()?.to_string(),
                address: dev.Id()?.to_string(),
            };
            res.push(info);
        }
    }

    Ok(res)
}
