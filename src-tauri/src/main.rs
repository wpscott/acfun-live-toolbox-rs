#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use acfun_live_toolbox::sdk;
use acfun_live_toolbox::sdk::prelude::*;

#[cfg(any(target_os = "macos", target_os = "windows"))]
use window_shadows::set_shadow;
#[cfg(target_os = "windows")]
use window_vibrancy::apply_acrylic;
#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

fn main() {
    let conn = db::initialize().unwrap();
    let result = db::load_user(conn);

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
            Ok(user) => UserState::new(user),
            Err(_) => UserState::new(None),
        })
        .manage(Uuid::new_v4().hyphenated())
        .manage(TokenState::new(None))
        .manage(ClientState::new(None))
        .menu(if cfg!(target_os = "macos") {
            tauri::Menu::os_default(&context.package_info().name)
        } else {
            tauri::Menu::default()
        })
        .invoke_handler(tauri::generate_handler![
            sdk::is_login,
            sdk::get_user,
            sdk::check_live_auth,
            sdk::check_live_status,
            sdk::get_stream_config,
            sdk::start_push,
            sdk::stop_push,
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
