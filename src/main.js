let isPinned = false;
const capsule = document.getElementById('capsule');
const titleEl = document.querySelector('.title');
const artistEl = document.querySelector('.artist');
const progressBar = document.getElementById('progress-bar');
const marquee = document.getElementById('marquee');
const timerEl = document.getElementById('tiny-timer');
const timerText = document.getElementById('timer-text');
const notifyEl = document.getElementById('async-notify');
const asyncText = document.getElementById('async-text');

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

window.__TAURI__.event.listen('smtc_update', (event) => {
    const data = event.payload;
    if (titleEl) titleEl.innerText = data.title;
    if (artistEl) artistEl.innerText = data.artist || "Unknown Artist";

    if (data.duration_ms > 0) {
        const percent = (data.position_ms / data.duration_ms) * 100;
        if (progressBar) progressBar.style.width = `${percent}%`;
    }
});

window.__TAURI__.event.listen('smtc_title_changed', (event) => {
    const data = event.payload;
    if (marquee) {
        marquee.innerText = data.title;
        marquee.classList.remove('active');
        void marquee.offsetWidth;
        marquee.classList.add('active');
    }
});

window.__TAURI__.event.listen('time_tick', (event) => {
    const data = event.payload;
    if (timerEl && timerText) {
        if (data.is_social && data.time_spent > 0) {
            timerEl.classList.remove('hidden');
            const m = Math.floor(data.time_spent / 60).toString().padStart(2, '0');
            const s = (data.time_spent % 60).toString().padStart(2, '0');
            timerText.innerText = `${m}:${s}`;

            if (data.time_spent >= 3600) timerEl.className = 'danger';
            else if (data.time_spent >= 2400) timerEl.className = 'warning';
            else timerEl.className = '';
        } else {
            timerEl.classList.add('hidden');
        }
    }
});

window.__TAURI__.event.listen('async_task_complete', (event) => {
    const data = event.payload;
    if (notifyEl && asyncText) {
        const m = Math.floor(data.duration_seconds / 60);
        const s = data.duration_seconds % 60;
        const durStr = m > 0 ? `${m}m ${s}s` : `${s}s`;
        let srcName = data.source.charAt(0).toUpperCase() + data.source.slice(1);

        asyncText.innerText = `${srcName} done - ${durStr}`;
        notifyEl.classList.remove('hidden');
        setTimeout(() => { notifyEl.classList.add('hidden'); }, 4000);
    }
});

console.log("Fase 3 & 4 Frontend Loaded.");
