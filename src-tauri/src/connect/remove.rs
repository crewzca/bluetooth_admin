use windows::{
    core::*,
    Devices::{Bluetooth::BluetoothLEDevice, Enumeration::DeviceInformation},
};

pub async fn remove_ble(rem: String, is_name: bool) -> Result<()> {
    let selector = BluetoothLEDevice::GetDeviceSelector()?;

    let devices_op = DeviceInformation::FindAllAsyncAqsFilter(&selector)?;
    let devices = devices_op.await?;

    for i in 0..devices.Size()? {
        let device = devices.GetAt(i)?;
        let target = if is_name {
            device.Name()?
        } else {
            device.Id()?
        };
        if target.to_string().contains(&rem) {
            let pairing = device.Pairing()?;
            let _ = pairing.UnpairAsync()?.await?;
        }
    }

    Ok(())
}
