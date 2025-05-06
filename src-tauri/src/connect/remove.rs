use windows::{
    core::*,
    Devices::{Bluetooth::BluetoothLEDevice, Enumeration::DeviceInformation},
};

pub async fn remove_samename(rem: String) -> Result<()> {
    let selector = BluetoothLEDevice::GetDeviceSelector()?;
    let devices_op = DeviceInformation::FindAllAsyncAqsFilter(&selector)?;
    let devices = devices_op.await?;

    for i in 0..devices.Size()? {
        let device = devices.GetAt(i)?;
        let name = device.Name()?;
        if name.to_string().contains(&rem) {
            let pairing = device.Pairing()?;
            let _ = pairing.UnpairAsync()?.await?;
        }
    }

    Ok(())
}
