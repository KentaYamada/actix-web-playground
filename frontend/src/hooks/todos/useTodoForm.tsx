import { useCallback, useMemo } from "react";
import { zodResolver } from "mantine-form-zod-resolver";
import { useForm } from "@mantine/form";
import { Todo } from "@entity";
import { TodoFormValues } from "./schema";

export function useTodoForm() {
  const form = useForm({
    mode: "uncontrolled",
    initialValues: {
      id: 0,
      status: 0,
      title: "",
      detail: "",
    },
    validate: zodResolver(TodoFormValues),
  });
  const { errors, resetDirty, setValues } = form;

  const invalid = useMemo(() => Object.keys(errors).length > 0, [errors]);

  const updateFormValues = useCallback(
    (payload: Todo) => {
      setValues(payload);
      resetDirty(payload);
    },
    [setValues, resetDirty],
  );

  return {
    form,
    invalid,
    updateFormValues,
  };
}
