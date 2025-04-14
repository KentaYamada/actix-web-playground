import { z } from "zod";

export const UserFormValues = z.object({
  id: z.number(),
  family_name: z.string().min(1, "名字を入力してください"),
  first_name: z.string().min(1, "名前を入力してください"),
  email: z
    .string()
    .min(1, "メールアドレスを入力してください")
    .email("メールアドレスの形式で入力してください"),
  password: z.string().min(1, ""),
});

export type UserFormValuesType = z.infer<typeof UserFormValues>;
