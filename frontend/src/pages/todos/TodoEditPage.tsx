import { Link } from "react-router";
import { Breadcrumbs, Divider, Title } from "@mantine/core";
import { DefaultLayout, TodoForm } from "@components";

export function TodoEditPage() {
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
      <TodoForm />
    </DefaultLayout>
  );
}
