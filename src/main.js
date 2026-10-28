// Visual test untuk memastikan klik tembus / berfungsi
document.getElementById('test-btn').addEventListener('click', () => {
  console.log("Capsule clicked!");
  document.getElementById('test-btn').innerText = "Berhasil!";
  setTimeout(() => {
    document.getElementById('test-btn').innerText = "Klik Saya";
  }, 1000);
});
