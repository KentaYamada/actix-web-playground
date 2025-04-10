import { Link } from "react-router";
import { Breadcrumbs, Divider, Title } from "@mantine/core";
import { DefaultLayout, TodoForm } from "@components";

export function TodoCreatePage() {
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
      <TodoForm />
    </DefaultLayout>
  );
}
