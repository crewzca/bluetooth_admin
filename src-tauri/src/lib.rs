mod connect;

use connect::device::DeviceInfo;
use connect::import::get_device_info;
use connect::remove::remove_ble;
use connect::remove_win32::remove_classic;

#[tauri::command]
async fn import_device() -> Vec<DeviceInfo> {
    return get_device_info().await.unwrap_or_default();
}

#[tauri::command]
async fn remove_all(name: String) -> bool {
    println!("削除するデバイスの名前：{}", name);
    match remove_ble(name, true).await {
        Ok(_) => return true,
        Err(e) => {
            eprintln!("一括削除でエラーが発生 {}", e);
            return false;
        }
    }
}

#[tauri::command]
async fn remove_device(address: String) -> bool {
    println!("削除するデバイスのアドレス：{}", address);
    if address[0..11].eq("BluetoothLE") {
        match remove_ble(address, false).await {
            Ok(_) => return true,
            Err(e) => {
                eprintln!("デバイス削除でエラーが発生 {}", e);
                return false;
            }
        }
    } else {
        remove_classic(address);
        return true;
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            remove_all,
            remove_device,
            import_device
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
