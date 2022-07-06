#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use tauri::Manager;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use window_shadows::set_shadow;
#[cfg(target_os = "windows")]
use window_vibrancy::apply_acrylic;
#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

use log::LevelFilter;
use tauri_plugin_log::{LogTarget, LoggerBuilder, RotationStrategy};

use acfun_live_toolbox::sdk;

fn main() {
    let conn = sdk::db::initialize().unwrap();
    let result = sdk::db::load_user(conn);

    let context = tauri::generate_context!();
    tauri::Builder::default()
        .plugin(
            LoggerBuilder::new()
                .targets([LogTarget::LogDir, LogTarget::Stdout])
                .level(LevelFilter::Debug)
                .rotation_strategy(RotationStrategy::KeepOne)
                .build(),
        )
        .manage(match result {
            Ok(user) => Mutex::new(user),
            Err(_) => Mutex::new(Option::<sdk::User>::None),
        })
        .menu(if cfg!(target_os = "macos") {
            tauri::Menu::os_default(&context.package_info().name)
        } else {
            tauri::Menu::default()
        })
        .invoke_handler(tauri::generate_handler![
            sdk::is_login,
            sdk::get_user,
            sdk::user::qr_login,
        ])
        .setup(|app| {
            let win = app.get_window("main").unwrap();

            #[cfg(any(target_os = "macos", target_os = "windows"))]
            set_shadow(&win, true).expect("Unsupported platform!");

            #[cfg(target_os = "macos")]
            apply_vibrancy(&win, NSVisualEffectMaterial::HudWindow)
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            #[cfg(target_os = "windows")]
            apply_acrylic(&win, Some((238, 238, 238, 125)))
                .expect("Unsupported platform! 'apply_acrylic' is only supported on Windows");

            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}