use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW};
use std::thread;
use std::time::Duration;

pub fn start_polling() {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));
            unsafe {
                let hwnd = GetForegroundWindow();
                if hwnd.0 != std::ptr::null_mut() {
                    let mut text: [u16; 512] = [0; 512];
                    let len = GetWindowTextW(hwnd, &mut text);
                    if len > 0 {
                        let title = String::from_utf16_lossy(&text[..len as usize]);
                        println!("[Foreground] {}", title);
                    }
                }
            }
        }
    });
}
