mod connect;

use connect::device::DeviceInfo;
use connect::import::get_device_info;
use connect::remove::remove_samename;

#[tauri::command]
async fn import_device() -> Vec<DeviceInfo> {
    return get_device_info().await.unwrap_or_default();
}

#[tauri::command]
async fn remove_all(name: String) -> bool {
    match remove_samename(name).await {
        Ok(_) => return true,
        Err(e) => {
            eprintln!("一括削除でエラーが発生 {}", e);
            return false;
        }
    }
}

#[tauri::command]
async fn remove_device(address: String) -> bool {
    return false;
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
