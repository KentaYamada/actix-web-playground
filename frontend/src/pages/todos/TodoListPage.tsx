import axios from "axios";
import { useCallback, useEffect, useState } from "react";
import { useNavigate } from "react-router";
import { Alert } from "@mantine/core";
import { IconInfoCircle } from "@tabler/icons-react";
import { DefaultLayout, StatusFilter, TodoListItem } from "@components";
import { Todo } from "@entity";

export function TodoListPage() {
  const navigate = useNavigate();
  const [todos, setTodos] = useState<Todo[]>([]);
  // const [todos] = useState<Todo[]>([
  //   {
  //     id: 1,
  //     status: 0,
  //     title: "test",
  //     detail: "this is test",
  //   },
  //   {
  //     id: 2,
  //     status: 1,
  //     title: "test",
  //     detail: "this is test",
  //   },
  //   {
  //     id: 3,
  //     status: 2,
  //     title: "test",
  //     detail: "this is test",
  //   },
  // ]);

  useEffect(() => {
    axios
      .get("/api/todos", {
        headers: { "Content-Type": "application/json" },
        data: {},
      })
      .then((res) => {
        setTodos(res.data.todos);
      })
      .catch((err) => {
        console.error(err);
      });
  }, []);

  const handleNavigateToEditPage = useCallback(
    (id: number) => {
      navigate(`/todos/${id}/edit`);
    },
    [navigate],
  );

  return (
    <DefaultLayout>
      <Alert color="red" title="システムエラー" icon={<IconInfoCircle />}>
        データの取得に失敗しました
      </Alert>
      <h2>Todo list </h2>
      <StatusFilter />
      {todos.map((todo: Todo) => (
        <TodoListItem
          key={todo.id}
          todo={todo}
          onNavigateToEditTodoPage={handleNavigateToEditPage}
        />
      ))}
    </DefaultLayout>
  );
}
