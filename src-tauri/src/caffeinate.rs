use crate::state::CaffeinateState;
use tauri::menu::MenuItem;
use tauri::{AppHandle, Wry};
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_shell::ShellExt;

pub fn start(app: &AppHandle, state: &CaffeinateState, menu_item: &MenuItem<Wry>) {
    if state.is_running() {
        return;
    }

    if let Err(err) = execute_shell_command(app, "/usr/bin/caffeinate") {
        eprintln!("failed to start caffeinate: {err}");
        return;
    }

    state.set_running(true);
    update_menu_label(menu_item, "☕️Caffeinating...");
    notification(app, "Caffeinate", "Don't sleep.");
}

pub fn stop(app: &AppHandle, state: &CaffeinateState, menu_item: &MenuItem<Wry>) {
    if !state.is_running() {
        return;
    }

    if let Err(err) = execute_shell_command(app, "pkill caffeinate") {
        eprintln!("failed to stop caffeinate: {err}");
    }

    state.set_running(false);
    update_menu_label(menu_item, "Caffeinate");
    notification(app, "Caffeinate", "Take a rest!");
}

fn execute_shell_command(app: &AppHandle, script: &str) -> Result<(), String> {
    let handle = app.clone();
    tauri::async_runtime::block_on(async move {
        handle
            .shell()
            .command("/bin/sh")
            .args(["-c", script])
            .spawn()
            .map(|_| ())
            .map_err(|err| err.to_string())
    })
}

fn update_menu_label(item: &MenuItem<Wry>, text: &str) {
    if let Err(err) = item.set_text(text) {
        eprintln!("failed to update menu text: {err}");
    }
}

fn notification(app: &AppHandle, title: &str, body: &str) {
    app.notification()
        .builder()
        .title(title)
        .body(body)
        .show()
        .unwrap();
}
