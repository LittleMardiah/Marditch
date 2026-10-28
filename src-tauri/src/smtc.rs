use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;
use std::thread;
use std::time::Duration;

pub fn start_listening() {
    thread::spawn(move || {
        let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap();
        loop {
            thread::sleep(Duration::from_secs(2));
            if let Ok(session) = manager.GetCurrentSession() {
                if let Ok(media_props) = session.TryGetMediaPropertiesAsync().unwrap().get() {
                    let title = media_props.Title().unwrap_or_default().to_string();
                    let artist = media_props.Artist().unwrap_or_default().to_string();
                    if !title.is_empty() {
                        println!("[SMTC] Now Playing: {} - {}", title, artist);
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
