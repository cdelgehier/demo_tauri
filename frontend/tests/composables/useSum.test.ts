import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { useSum } from "~/composables/useSum";

describe("useSum", () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("returns the result for two positive integers", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ result: 5 }),
      })
    );

    const { result, error, compute } = useSum();
    await compute(2, 3);

    expect(result.value).toBe(5);
    expect(error.value).toBeNull();
  });

  it("handles negative numbers", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ result: -3 }),
      })
    );

    const { result, compute } = useSum();
    await compute(-1, -2);

    expect(result.value).toBe(-3);
  });

  it("sets error on HTTP error", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue({
        ok: false,
        status: 500,
      })
    );

    const { result, error, compute } = useSum();
    await compute(1, 2);

    expect(result.value).toBeNull();
    expect(error.value).toContain("500");
  });

  it("sets error when response does not match Zod schema", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ result: "not_a_number" }),
      })
    );

    const { result, error, compute } = useSum();
    await compute(1, 2);

    expect(result.value).toBeNull();
    expect(error.value).toBeTruthy();
  });

  it("resets the previous result before a new calculation", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn()
        .mockResolvedValueOnce({
          ok: true,
          json: () => Promise.resolve({ result: 10 }),
        })
        .mockResolvedValueOnce({
          ok: false,
          status: 503,
        })
    );

    const { result, error, compute } = useSum();
    await compute(5, 5);
    expect(result.value).toBe(10);

    await compute(1, 1);
    expect(result.value).toBeNull();
    expect(error.value).toBeTruthy();
  });
});
