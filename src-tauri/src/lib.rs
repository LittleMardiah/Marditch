mod hit_test;
mod smtc;
mod fullscreen;
mod foreground;
mod wsl_hook;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::Manager;

#[tauri::command]
fn media_control(action: String) { smtc::control(&action); }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            hit_test::start_polling(handle.clone());
            smtc::start_listening();
            fullscreen::start_polling(handle.clone());
            foreground::start_polling();
            wsl_hook::start_server();

            // Setup Tray
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
        .invoke_handler(tauri::generate_handler![media_control])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
