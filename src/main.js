let isPinned = false;
const capsule = document.getElementById('capsule');

// Logika Pin State (S4)
window.togglePin = (event) => {
  if (event.target.tagName === 'SPAN') return;
  
  isPinned = !isPinned;
  if (isPinned) {
    capsule.classList.add('pinned');
  } else {
    capsule.classList.remove('pinned');
  }
};

// ESC untuk lepas pin
window.addEventListener('keydown', (e) => {
  if (e.key === 'Escape' && isPinned) {
    isPinned = false;
    capsule.classList.remove('pinned');
  }
});

console.log("Frontend loaded. Safe Mode Active.");
