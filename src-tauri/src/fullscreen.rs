use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowRect, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use windows::Win32::Foundation::RECT;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Manager};

pub fn start_polling(app: AppHandle) {
    thread::spawn(move || {
        let window = app.get_webview_window("main").unwrap();
        loop {
            thread::sleep(Duration::from_millis(500));
            unsafe {
                let hwnd = GetForegroundWindow();
                if hwnd.0 != std::ptr::null_mut() {
                    let mut rect = RECT::default();
                    if GetWindowRect(hwnd, &mut rect).is_ok() {
                        let screen_w = GetSystemMetrics(SM_CXSCREEN);
                        let screen_h = GetSystemMetrics(SM_CYSCREEN);
                        let win_w = rect.right - rect.left;
                        let win_h = rect.bottom - rect.top;
                        
                        // Jika window foreground menutupi seluruh layar, anggap fullscreen
                        let is_fs = win_w >= screen_w && win_h >= screen_h;
                        if is_fs {
                            let _ = window.hide();
                        } else {
                            let _ = window.show();
                        }
                    }
                }
            }
        }
    });
}
