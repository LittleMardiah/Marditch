mod smtc;
mod foreground;
mod wsl_hook;
mod config;
// mod hit_test; // DISABLED: Menyebabkan crash
// mod fullscreen; // DISABLED: Menyebabkan crash

use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{Manager, PhysicalPosition};

#[tauri::command]
fn media_control(action: String) { smtc::control(&action); }

#[tauri::command]
fn save_window_pos(app: tauri::AppHandle, x: i32, y: i32) {
    let mut cfg = config::load_config(&app);
    cfg.window_x = Some(x);
    cfg.window_y = Some(y);
    config::save_config(&app, &cfg);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            
            let cfg = config::load_config(&handle);
            if let Some(win) = app.get_webview_window("main") {
                if let (Some(x), Some(y)) = (cfg.window_x, cfg.window_y) {
                    let _ = win.set_position(PhysicalPosition::new(x, y));
                } else {
                    if let Ok(Some(monitor)) = win.current_monitor() {
                        let size = monitor.size();
                        let win_size = win.outer_size().unwrap_or_default();
                        let x = (size.width as i32 / 2) - (win_size.width as i32 / 2);
                        let _ = win.set_position(PhysicalPosition::new(x, 0));
                    }
                }
            }

            // hit_test::start_polling(handle.clone()); // DISABLED
            smtc::start_listening();
            // fullscreen::start_polling(handle.clone()); // DISABLED
            foreground::start_polling();
            wsl_hook::start_server();

            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Show/Hide", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => std::process::exit(0),
                    "show" => {
                        let win = app.get_webview_window("main").unwrap();
                        if win.is_visible().unwrap() { let _ = win.hide(); } 
                        else { let _ = win.show(); }
                    }
                    _ => {}
                })
                .build(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![media_control, save_window_pos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
