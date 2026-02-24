import { z } from "zod";

export const SumRequestSchema = z.object({
  a: z.number().int("A must be an integer"),
  b: z.number().int("B must be an integer"),
});

export const SumResponseSchema = z.object({
  result: z.number().int(),
});

export type SumRequest = z.infer<typeof SumRequestSchema>;
export type SumResponse = z.infer<typeof SumResponseSchema>;
