import { useEffect } from "react";
import { Link, useNavigate } from "react-router";
import { Breadcrumbs, Divider, Title } from "@mantine/core";
import { DefaultLayout, TodoForm } from "@components";
import { useTodoApi, useTodoForm, TodoFormValuesType } from "@hooks";
import { ErrorMessage, useErrorMessage } from "@shared/ui";
import { ApiResponseError, Todo } from "@entity";

export function TodoCreatePage() {
  const navigate = useNavigate();
  const {
    errorMessageConfig,
    visibleErrorMessage,
    showErrorMessage,
    hideErrorMessage,
  } = useErrorMessage();
  const { createTodoApi } = useTodoApi();
  const { form, invalid } = useTodoForm();

  const handleSave = form.onSubmit((formValues: TodoFormValuesType) => {
    const payload: Todo = { ...formValues };

    hideErrorMessage();
    createTodoApi(payload)
      .then(() => navigate("/todos"))
      .catch((err: ApiResponseError) => {
        showErrorMessage({
          title: "システムエラー",
          message: err.message,
        });
      });
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
