// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri_plugin_shell::ShellExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let caffeinate_i =
                MenuItem::with_id(app, "caffeinate", "Caffeinate", true, None::<&str>)?;
            let no_more_caffeine_i =
                MenuItem::with_id(app, "no-caffeine", "Sleep", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&caffeinate_i, &no_more_caffeine_i])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "caffeinate" => {
                        let shell = app.shell();
                        tauri::async_runtime::block_on(async move {
                            shell
                                .command("/bin/sh")
                                .args(["-c", "/usr/bin/caffeinate "])
                                .spawn()
                                .unwrap()
                        });
                    }
                    "no-caffeine" => {
                        let shell = app.shell();
                        tauri::async_runtime::block_on(async move {
                            shell
                                .command("/bin/sh")
                                .args(["-c", "pkill caffeinate"])
                                .spawn()
                                .unwrap()
                        });
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .show_menu_on_left_click(true)
                .icon(app.default_window_icon().unwrap().clone())
                .build(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
