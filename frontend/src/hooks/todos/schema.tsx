import { z } from "zod";

export const TodoFormValues = z.object({
  id: z.number(),
  status: z.number(),
  title: z.string().min(1, "タイトルを入力してください"),
  detail: z.string(),
});

export type TodoFormValuesType = z.infer<typeof TodoFormValues>;
