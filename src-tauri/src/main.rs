// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod connection;
mod events;
mod service;
mod types;
mod utils;
use commands::{clipboard, hotkey, settings, window};
use tauri_plugin_autostart::MacosLauncher;
use utils::tauri::{setup, tray};

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .setup(setup::setup)
        .system_tray(tray::system_tray())
        .on_system_tray_event(tray::system_tray_event)
        .invoke_handler(tauri::generate_handler![
            clipboard::get_clipboards,
            clipboard::delete_clipboard,
            clipboard::star_clipboard,
            clipboard::copy_clipboard,
            clipboard::clear_clipboards,
            clipboard::type_clipboard,
            hotkey::get_hotkeys,
            hotkey::update_hotkey,
            hotkey::stop_hotkeys,
            hotkey::start_hotkeys,
            settings::get_settings,
            settings::update_settings,
            window::window_display_toggle,
            window::get_db_size,
            window::get_db_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
