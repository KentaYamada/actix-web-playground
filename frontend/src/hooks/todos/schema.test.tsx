import { TodoFormValues } from "./schema";

describe("TodoFormValues tests", () => {
  it("validation successfully", () => {
    const data = {
      id: 1,
      status: 0,
      title: "This is test",
      detail: "test test test",
    };

    const validated = TodoFormValues.safeParse(data);

    expect(validated.success).toBe(true);
  });

  it("title required test", () => {
    const data = {
      id: 1,
      status: 0,
      title: "",
      detail: "test test test",
    };

    const validated = TodoFormValues.safeParse(data);

    expect(validated.success).toBe(false);

    if (!validated.success) {
      expect(validated.error.errors[0].message).toBe(
        "タイトルを入力してください",
      );
    }
  });
});
