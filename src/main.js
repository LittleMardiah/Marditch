let isPinned = false;
const capsule = document.getElementById('capsule');
const titleEl = document.querySelector('.title');
const artistEl = document.querySelector('.artist');
const progressBar = document.getElementById('progress-bar');
const marquee = document.getElementById('marquee');

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

// Update Meta & Progress
window.__TAURI__.event.listen('smtc_update', (event) => {
    const data = event.payload;
    if (titleEl) titleEl.innerText = data.title;
    if (artistEl) artistEl.innerText = data.artist || "Unknown Artist";
    
    if (data.duration_ms > 0) {
        const percent = (data.position_ms / data.duration_ms) * 100;
        if (progressBar) progressBar.style.width = `${percent}%`;
    }
});

// Title Peek Marquee
window.__TAURI__.event.listen('smtc_title_changed', (event) => {
    const data = event.payload;
    if (marquee) {
        marquee.innerText = data.title;
        marquee.classList.remove('active');
        void marquee.offsetWidth; // trigger reflow
        marquee.classList.add('active');
    }
});

console.log("Fase 2 Frontend Loaded.");
