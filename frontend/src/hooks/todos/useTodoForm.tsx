import { useMemo } from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { TodoFormValues, TodoFormValuesType } from "./schema";

export function useTodoForm() {
  const {
    control,
    formState: { errors, isSubmitted },
    handleSubmit,
  } = useForm<TodoFormValuesType>({
    mode: "onSubmit",
    defaultValues: {
      id: 0,
      status: 0,
      title: "",
      detail: "",
    },
    resolver: zodResolver(TodoFormValues),
  });

  const invalid = useMemo(
    function () {
      return Object.keys(errors).length > 0;
    },
    [errors],
  );

  return {
    control,
    errors,
    invalid,
    isSubmitted,
    handleSubmit,
  };
}
