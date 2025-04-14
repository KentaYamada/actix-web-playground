import { useCallback, useMemo } from "react";
import { zodResolver } from "mantine-form-zod-resolver";
import { useForm } from "@mantine/form";
import { User } from "@entity";
import { UserFormValues } from "./schema";

export function useUserForm() {
  const form = useForm({
    mode: "uncontrolled",
    initialValues: {
      id: 0,
      family_name: "",
      first_name: "",
      email: "",
      password: "",
    },
    validate: zodResolver(UserFormValues),
  });
  const { errors, resetDirty, setValues } = form;

  const invalid = useMemo(() => Object.keys(errors).length > 0, [errors]);

  const updateFormValues = useCallback(
    (payload: User) => {
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
