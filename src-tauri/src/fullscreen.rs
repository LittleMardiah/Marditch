use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowRect, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use windows::Win32::Foundation::RECT;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Manager};

pub fn start_polling(app: AppHandle) {
    thread::spawn(move || {
        let window = app.get_webview_window("main").unwrap();
        let mut last_state = false;
        
        loop {
            thread::sleep(Duration::from_secs(2)); // Aman: 2 detik
            unsafe {
                let hwnd = GetForegroundWindow();
                if hwnd.0 != std::ptr::null_mut() {
                    let mut rect = RECT::default();
                    if GetWindowRect(hwnd, &mut rect).is_ok() {
                        let screen_w = GetSystemMetrics(SM_CXSCREEN);
                        let screen_h = GetSystemMetrics(SM_CYSCREEN);
                        let win_w = rect.right - rect.left;
                        let win_h = rect.bottom - rect.top;
                        
                        let is_fs = win_w >= screen_w && win_h >= screen_h;
                        
                        // Hanya eksekusi jika status berubah (Anti-Crash)
                        if is_fs != last_state {
                            last_state = is_fs;
                            if is_fs {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                            }
                        }
                    }
                }
            }
        }
    });
}
