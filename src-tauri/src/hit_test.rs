use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Manager};

// Fungsi untuk memulai thread polling kursor
pub fn start_polling(app: AppHandle) {
    let app_arc = Arc::new(Mutex::new(app));
    
    thread::spawn(move || {
        // State awal: click-through aktif (margin tembus)
        let mut is_ignoring = false;
        let window = app_arc.lock().unwrap().get_webview_window("main").unwrap();

        loop {
            thread::sleep(Duration::from_millis(16)); // ~60 FPS

            // Dapatkan posisi kursor global
            let cursor_pos = window.cursor_position();

            if let Ok(cursor) = cursor_pos {
                // Dapatkan posisi window di layar
                let win_pos = window.outer_position().unwrap();
                let scale = window.scale_factor().unwrap();
                
                // Konversi koordinat kursor global ke koordinat lokal window
                let local_x = (cursor.x - win_pos.x as f64) / scale;
                let local_y = (cursor.y - win_pos.y as f64) / scale;

                // Area kapsul: 180x32 di tengah atas (top: 8px)
                // Window width = 400, jadi x kapsul mulai dari (400-180)/2 = 110px
                let capsule_x_start = 110.0;
                let capsule_x_end = 290.0;
                let capsule_y_start = 8.0;
                let capsule_y_end = 40.0;

                let is_inside = local_x >= capsule_x_start
                    && local_x <= capsule_x_end
                    && local_y >= capsule_y_start
                    && local_y <= capsule_y_end;

                // Logika Hit-testing:
                // Jika di DALAM kapsul -> tangkap klik (ignore = false)
                // Jika di LUAR kapsul -> tembus klik (ignore = true)
                let should_ignore = !is_inside;

                if should_ignore != is_ignoring {
                    let _ = window.set_ignore_cursor_events(should_ignore);
                    is_ignoring = should_ignore;
                }
            }
        }
    });
}
