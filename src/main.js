let isPinned = false;
const capsule = document.getElementById('capsule');
const titleEl = document.querySelector('.title');
const artistEl = document.querySelector('.artist');

// Logika Pin State (S4)
window.togglePin = (event) => {
  if (event.target.tagName === 'SPAN') return;
  isPinned = !isPinned;
  if (isPinned) capsule.classList.add('pinned');
  else capsule.classList.remove('pinned');
};

window.addEventListener('keydown', (e) => {
  if (e.key === 'Escape' && isPinned) {
    isPinned = false;
    capsule.classList.remove('pinned');
  }
});

// Listen ke event dari Rust (Tauri v2 Global API)
window.__TAURI__.event.listen('smtc_update', (event) => {
    const data = event.payload;
    if (titleEl) titleEl.innerText = data.title;
    if (artistEl) artistEl.innerText = data.artist || "Unknown Artist";
});

console.log("Frontend loaded. Listeners active.");
