// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command(rename_all = "snake_case")]
fn r_connect_server(name:String, server_address:String, port:String, login_code:String) -> bool {
    println!("==================\nName: {}\nServer address: {}\nPort: {}\nLogin code: {}\n==================", name, server_address, port, login_code);
    return true;
}

#[tauri::command]
fn r_dis_connect_server() -> bool {
    let a: i32 = 10;
    let b: i32 = 10;
    if a==b {
        return true
    } else {
        false
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![r_connect_server, r_dis_connect_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
