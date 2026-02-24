import { ref } from "vue";
import { SumRequestSchema, SumResponseSchema } from "~/schemas/sum";

const API_BASE = "http://127.0.0.1:8000";

export function useSum() {
  const result = ref<number | null>(null);
  const error = ref<string | null>(null);
  const loading = ref(false);

  async function compute(a: number, b: number) {
    loading.value = true;
    error.value = null;
    result.value = null;

    try {
      const body = SumRequestSchema.parse({ a, b });

      const response = await fetch(`${API_BASE}/sum`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(body),
      });

      if (!response.ok) {
        throw new Error(`Request failed (${response.status})`);
      }

      const data = await response.json();
      const parsed = SumResponseSchema.parse(data);
      result.value = parsed.result;
    } catch (e) {
      error.value =
        e instanceof Error ? e.message : "Something went wrong";
    } finally {
      loading.value = false;
    }
  }

  return { result, error, loading, compute };
}
