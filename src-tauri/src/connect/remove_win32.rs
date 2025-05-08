use windows::Win32::Devices::Bluetooth::{
    BluetoothRemoveDevice, BLUETOOTH_ADDRESS, BLUETOOTH_ADDRESS_0,
};

fn parse_address(mac: String) -> Option<BLUETOOTH_ADDRESS> {
    let bytes: Vec<u8> = mac
        .split(|c| c == ':' || c == '-')
        .filter_map(|part| u8::from_str_radix(part, 16).ok())
        .collect();

    if bytes.len() != 6 {
        return None;
    }

    let mut address = [0u8; 6];
    for i in 0..6 {
        address[i] = bytes[5 - i]
    }

    return Some(BLUETOOTH_ADDRESS {
        Anonymous: BLUETOOTH_ADDRESS_0 { rgBytes: address },
    });
}

pub fn remove_classic(mac: String) {
    if let Some(address) = parse_address(mac) {
        let res = unsafe { BluetoothRemoveDevice(&address) };
        if res != 0 {
            println!("デバイスの削除に失敗 code:{}", res)
        }
    } else {
        println!("アドレスの書式エラー")
    }
}
