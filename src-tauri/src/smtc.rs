use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

pub fn start_listening(app: AppHandle) {
    thread::spawn(move || {
        let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap();
        let mut last_title = String::new();

        loop {
            thread::sleep(Duration::from_secs(1));
            if let Ok(session) = manager.GetCurrentSession() {
                if let Ok(media_props) = session.TryGetMediaPropertiesAsync().unwrap().get() {
                    let title = media_props.Title().unwrap_or_default().to_string();
                    let artist = media_props.Artist().unwrap_or_default().to_string();

                    if !title.is_empty() && title != last_title {
                        last_title = title.clone();
                        let payload = serde_json::json!({
                            "title": title,
                            "artist": artist
                        });
                        let _ = app.emit("smtc_update", payload);
                        println!("[SMTC] Update Sent: {} - {}", title, artist);
                    }
                }
            }
        }
    });
}

pub fn control(action: &str) {
    if let Ok(manager) = GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get() {
        if let Ok(session) = manager.GetCurrentSession() {
            match action {
                "play_pause" => { let _ = session.TryTogglePlayPauseAsync().map(|a| a.get()); }
                "next" => { let _ = session.TrySkipNextAsync().map(|a| a.get()); }
                "prev" => { let _ = session.TrySkipPreviousAsync().map(|a| a.get()); }
                _ => {}
            }
        }
    }
}
