import { Link } from "react-router";
import { Breadcrumbs, Divider, Title } from "@mantine/core";
import { DefaultLayout, ErrorMessage, TodoForm } from "@components";
import { useTodoForm, TodoFormValuesType } from "@hooks";

export function TodoCreatePage() {
  const { form, invalid } = useTodoForm();
  const handleSave = form.onSubmit((formValues: TodoFormValuesType) => {
    console.log(formValues);
  });

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
      <ErrorMessage
        title="入力エラー"
        message="入力内容に誤りがあります。"
        visible={invalid}
      />
      <TodoForm form={form} onSave={handleSave} />
    </DefaultLayout>
  );
}
