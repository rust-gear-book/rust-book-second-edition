use tauri::Emitter;
use tauri_plugin_dialog::DialogExt;

#[derive(Clone, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Entry {
    file_path: String,
    type_name: String,
}

#[derive(Clone, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct EntryList {
    entry_list: Vec<Entry>,
}

#[tauri::command]
fn read_file_content(entry: String) -> tauri::ipc::Response {
    let path_buf = std::path::PathBuf::from(entry);
    let mut data: Vec<u8> = vec![];
    if path_buf.is_file() {
        data = std::fs::read(path_buf).unwrap();
    }
    tauri::ipc::Response::new(data)
}

fn handle_open<R: tauri::Runtime>(handle: &tauri::AppHandle<R>) {
    let handle_cloned = handle.clone();
    handle.dialog().file().pick_folder(move |folder_path| {
        if let Some(folder_path) = folder_path {
            let read_dir_result = std::fs::read_dir(folder_path.as_path().unwrap());
            if let Ok(read_dir_result) = read_dir_result {
                let mut entry_list = EntryList { entry_list: vec![] };
                for dir_entry in read_dir_result {
                    let file_path = dir_entry.unwrap().path();
                    let entry = Entry {
                        file_path: file_path.to_string_lossy().to_string(),
                        type_name: if file_path.is_file() {
                            String::from("file")
                        } else {
                            String::from("dir")
                        },
                    };
                    entry_list.entry_list.push(entry);
                }
                handle_cloned.emit("file-list", entry_list).unwrap();
            }
        }
    });
}

fn handle_quit<R: tauri::Runtime>(handle: &tauri::AppHandle<R>) {
    handle.cleanup_before_exit();
    handle.exit(0);
}

fn init_menu<R: tauri::Runtime>(handle: &tauri::AppHandle<R>) {
    let open_menu_item = tauri::menu::MenuItemBuilder::new("Open")
        .build(handle)
        .unwrap();
    let quit_menu_item = tauri::menu::MenuItemBuilder::new("Quit")
        .accelerator("CmdOrCtrl+Q")
        .build(handle)
        .unwrap();
    let file_submenu = tauri::menu::SubmenuBuilder::new(handle, "File")
        .item(&open_menu_item)
        .separator()
        .item(&quit_menu_item)
        .build()
        .unwrap();
    let menu = tauri::menu::MenuBuilder::new(handle)
        .items(&[&file_submenu])
        .build()
        .unwrap();
    handle.set_menu(menu).unwrap();

    handle.on_menu_event(move |app_handle, event| {
        if event.id() == open_menu_item.id() {
            handle_open(app_handle);
        } else if event.id() == quit_menu_item.id() {
            handle_quit(app_handle);
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![read_file_content])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            init_menu(app.handle());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
