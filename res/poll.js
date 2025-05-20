async function pollState() {
  try {
    const res = await fetch('/state');
    if (!res.ok) throw new Error('Network response not ok');
    const newState = await res.text();
    // For example, reload the page if state changed:
    if (window.currentState !== newState) {
      window.currentState = newState;
      console.log('State changed, reloading...');
      window.location.reload();
    }
  } catch (err) {
    console.error('Polling error:', err);
  }
}
setInterval(pollState, 300); // poll every 3 seconds
pollState(); // initial poll