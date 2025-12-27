// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::sync::atomic::{AtomicBool, Ordering};

use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::Manager;
use tauri_plugin_shell::ShellExt;

struct CaffeinateState {
    running: AtomicBool,
}

impl Default for CaffeinateState {
    fn default() -> Self {
        Self {
            running: AtomicBool::new(false),
        }
    }
}

impl CaffeinateState {
    fn set_running(&self, is_running: bool) {
        self.running.store(is_running, Ordering::SeqCst);
    }

    fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(CaffeinateState::default())
        .setup(|app| {
            let caffeinate_i =
                MenuItem::with_id(app, "caffeinate", "Caffeinate", true, None::<&str>)?;
            let no_more_caffeine_i =
                MenuItem::with_id(app, "no-caffeine", "Sleep", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&caffeinate_i, &no_more_caffeine_i])?;
            let caffeinate_item = caffeinate_i.clone();

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(move |app, event| {
                    let state = app.state::<CaffeinateState>();

                    match event.id.as_ref() {
                        "caffeinate" => {
                            if state.is_running() {
                                return;
                            }

                            let shell = app.shell();
                            if let Err(err) = tauri::async_runtime::block_on(async move {
                                shell
                                    .command("/bin/sh")
                                    .args(["-c", "/usr/bin/caffeinate"])
                                    .spawn()
                            }) {
                                eprintln!("failed to start caffeinate: {err}");
                                return;
                            }

                            state.set_running(true);
                            if let Err(err) = caffeinate_item.set_text("Caffeinating...") {
                                eprintln!("failed to update menu text: {err}");
                            }
                        }
                        "no-caffeine" => {
                            if !state.is_running() {
                                return;
                            }

                            let shell = app.shell();
                            if let Err(err) = tauri::async_runtime::block_on(async move {
                                shell
                                    .command("/bin/sh")
                                    .args(["-c", "pkill caffeinate"])
                                    .spawn()
                            }) {
                                eprintln!("failed to stop caffeinate: {err}");
                            }

                            state.set_running(false);
                            if let Err(err) = caffeinate_item.set_text("Caffeinate") {
                                eprintln!("failed to reset menu text: {err}");
                            }
                        }
                        _ => {
                            println!("menu item {:?} not handled", event.id);
                        }
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
