// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod caffeinate;
mod state;

use caffeinate::{start, stop};
use state::CaffeinateState;

use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(CaffeinateState::default())
        .setup(|app| {
            // On macOS, set the activation policy to Accessory to hide the dock icon
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let caffeinate_i =
                MenuItem::with_id(app, "caffeinate", "Caffeinate", true, None::<&str>)?;

            let no_more_caffeine_i =
                MenuItem::with_id(app, "no-caffeine", "Sleep", true, None::<&str>)?;

            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            let separator = PredefinedMenuItem::separator(app)?;

            let menu = Menu::with_items(
                app,
                &[&caffeinate_i, &no_more_caffeine_i, &separator, &quit_i],
            )?;
            let caffeinate_item = caffeinate_i.clone();

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(move |app, event| {
                    let state = app.state::<CaffeinateState>();
                    let caffeinate_state: &CaffeinateState = &*state;

                    match event.id.as_ref() {
                        "caffeinate" => start(app, caffeinate_state, &caffeinate_item),
                        "no-caffeine" => stop(app, caffeinate_state, &caffeinate_item),
                        "quit" => {
                            std::process::exit(0);
                        }
                        _ => println!("menu item {:?} not handled", event.id),
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
