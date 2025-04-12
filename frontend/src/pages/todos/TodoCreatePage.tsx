import { useEffect } from "react";
import { Link } from "react-router";
import { Breadcrumbs, Divider, Title } from "@mantine/core";
import { DefaultLayout, TodoForm } from "@components";
import { useTodoForm, TodoFormValuesType } from "@hooks";
import { ErrorMessage, useErrorMessage } from "@shared/ui";

export function TodoCreatePage() {
  const {
    errorMessageConfig,
    visibleErrorMessage,
    showErrorMessage,
    hideErrorMessage,
  } = useErrorMessage();
  const { form, invalid } = useTodoForm();

  const handleSave = form.onSubmit((formValues: TodoFormValuesType) => {
    hideErrorMessage();
    console.log(formValues);
  });

  useEffect(() => {
    if (invalid) {
      showErrorMessage({
        title: "入力エラー",
        message: "入力内容に誤りがあります",
      });
    }
  }, [invalid, showErrorMessage]);

  return (
    <DefaultLayout>
      <Breadcrumbs separator="/">
        <Link to="/todos">ToDo一覧</Link>
        <span>ToDo追加</span>
      </Breadcrumbs>
      <Title order={2} mt="md">
        ToDo追加
      </Title>
      <Divider my="md" />
      <ErrorMessage config={errorMessageConfig} visible={visibleErrorMessage} />
      <TodoForm form={form} onSave={handleSave} />
    </DefaultLayout>
  );
}
