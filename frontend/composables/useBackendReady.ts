import { ref, onMounted } from "vue";

const API_BASE = "http://127.0.0.1:8000";
const POLL_INTERVAL_MS = 300;
const MAX_WAIT_MS = 30_000;

/** Polls the backend until it responds, then flips `ready` to true. */
export function useBackendReady() {
  const ready = ref(false);
  const timedOut = ref(false);

  onMounted(() => {
    const start = Date.now();

    const interval = setInterval(async () => {
      try {
        const res = await fetch(`${API_BASE}/health`, { method: "GET" });
        if (res.ok || res.status === 405) {
          ready.value = true;
          clearInterval(interval);
        }
      } catch {
        // backend not up yet
      }

      if (!ready.value && Date.now() - start > MAX_WAIT_MS) {
        timedOut.value = true;
        clearInterval(interval);
      }
    }, POLL_INTERVAL_MS);
  });

  return { ready, timedOut };
}
