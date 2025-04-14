import { useEffect } from "react";
import { Link, useNavigate, useParams } from "react-router";
import { Breadcrumbs, Divider, Title } from "@mantine/core";
import { DefaultLayout, TodoForm } from "@components";
import { ApiResponseError, Todo } from "@entity";
import { useTodoApi, useTodoForm, TodoFormValuesType } from "@hooks";
import { ErrorMessage, useErrorMessage } from "@shared/ui";

export function TodoEditPage() {
  const navigate = useNavigate();
  const { id } = useParams();
  const {
    errorMessageConfig,
    visibleErrorMessage,
    showErrorMessage,
    hideErrorMessage,
  } = useErrorMessage();
  const { getTodoApi, updateTodoApi } = useTodoApi();
  const { form, invalid, updateFormValues } = useTodoForm();

  const handleSave = form.onSubmit((formValues: TodoFormValuesType) => {
    const payload: Todo = { ...formValues };

    hideErrorMessage();
    updateTodoApi(payload)
      .then(() => navigate("/todos"))
      .catch((reason: ApiResponseError) => {
        showErrorMessage({
          title: "システムエラー",
          message: reason.message,
        });
      });
  });

  useEffect(() => {
    if (id !== undefined && id !== "") {
      getTodoApi(Number(id))
        .then((res) => updateFormValues(res.data.todo))
        .catch((reason: ApiResponseError) => {
          if (reason.status === 404) {
            navigate("/notfound");
          }

          showErrorMessage({
            title: "システムエラー",
            message: reason.message,
          });
        });
    }
  }, [id, navigate, updateFormValues, showErrorMessage, getTodoApi]);

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
      <Breadcrumbs separator=">">
        <Link to="/todos">ToDo一覧</Link>
        <span>ToDo編集</span>
      </Breadcrumbs>
      <Title order={2} mt="md">
        ToDo編集
      </Title>
      <Divider my="md" />
      <ErrorMessage config={errorMessageConfig} visible={visibleErrorMessage} />
      <TodoForm form={form} onSave={handleSave} />
    </DefaultLayout>
  );
}
