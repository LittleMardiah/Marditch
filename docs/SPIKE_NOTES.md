# SPIKE NOTES - Phase 0

## Results
- E1 (Env): OK (Rust 1.99, VS Build Tools 2022)
- E2 (Window): OK (Transparent, frameless, 400x120)
- E3 (Hit-testing): Option A (Polling 16ms) implemented.
- E4 (SMTC): Implemented (Read metadata, control Play/Next/Prev).
- E5 (Animasi): Implemented (CSS Hover 180x32 -> 320x80, Marquee).
- E6 (Fullscreen & Tray): Implemented (Win32 GetForegroundWindow rect check, Tray Menu).
- E7 (Foreground): Implemented (Win32 GetWindowTextW).
- E8 (WSL Hook): Implemented (Axum Server on port 9182).

## Decisions
- Hit-testing: Option A (Polling). Karena Option B (WM_NCHITTEST) butuh raw Win32 WndProc subclassing yang lebih kompleks.
- WSL Hook: Menggunakan Axum (ringan, async).

## Next
- User harus menjalankan `npm run tauri dev` untuk verifikasi T8, T12, SMTC, WSL Hook.
