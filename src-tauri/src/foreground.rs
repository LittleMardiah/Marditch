use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

pub fn start_polling(app: AppHandle) {
    thread::spawn(move || {
        let mut last_title = String::new();
        let mut time_spent = 0;

        loop {
            thread::sleep(Duration::from_secs(1));
            let mut title = String::new();

            unsafe {
                let hwnd = GetForegroundWindow();
                if hwnd.0 != std::ptr::null_mut() {
                    let mut text: [u16; 512] = [0; 512];
                    let len = GetWindowTextW(hwnd, &mut text);
                    if len > 0 {
                        title = String::from_utf16_lossy(&text[..len as usize]);
                    }
                }
            }

            if title != last_title {
                last_title = title.clone();
                time_spent = 0;
            } else {
                time_spent += 1;
            }

            let lower = title.to_lowercase();
            let is_social = lower.contains("instagram") || lower.contains("tiktok") ||
                            lower.contains("twitter") || lower.contains("facebook") ||
                            (lower.contains("youtube") && !lower.contains("music"));

            let payload = serde_json::json!({
                "title": title,
                "is_social": is_social,
                "time_spent": time_spent
            });

            let _ = app.emit("time_tick", payload);
        }
    });
}
