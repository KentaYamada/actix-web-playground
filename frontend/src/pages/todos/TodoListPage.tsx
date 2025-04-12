import { useCallback, useEffect, useState } from "react";
import { useNavigate } from "react-router";
import { Text, Title } from "@mantine/core";
import { DefaultLayout, StatusFilter, TodoListItem } from "@components";
import { ApiResponseError, Todo } from "@entity";
import {
  ConfirmDialog,
  ErrorMessage,
  useConfirmDialog,
  useErrorMessage,
} from "@shared/ui";
import { useTodoApi } from "@hooks/todos";

export function TodoListPage() {
  const navigate = useNavigate();
  const {
    errorMessageConfig,
    visibleErrorMessage,
    showErrorMessage,
    hideErrorMessage,
  } = useErrorMessage();
  const {
    confirmDialogConfig,
    visibleConfirmDialog,
    openConfirmDialog,
    closeConfirmDialog,
  } = useConfirmDialog();
  const { searchTodos, deleteTodo } = useTodoApi();
  const [todos, setTodos] = useState<Todo[]>([]);

  const handleDeleteTodo = useCallback(
    (id: number) => {
      openConfirmDialog({
        confirmButtonText: "削除",
        cancelButtonText: "閉じる",
        content: <Text>削除しますか？</Text>,
        onCancel: () => closeConfirmDialog(),
        onConfirm: () => {
          closeConfirmDialog();
          deleteTodo(id).catch((err: ApiResponseError) => {
            showErrorMessage({
              title: "システムエラー",
              message: err.message,
            });
          });
        },
      });
    },
    [closeConfirmDialog, openConfirmDialog, deleteTodo, showErrorMessage],
  );

  const handleNavigateToEditPage = useCallback(
    (id: number) => {
      navigate(`/todos/${id}/edit`);
    },
    [navigate],
  );

  useEffect(() => {
    hideErrorMessage();
    searchTodos()
      .then((res) => setTodos(res.data.todos))
      .catch((err: ApiResponseError) =>
        showErrorMessage({
          title: "システムエラー",
          message: err.message,
        }),
      );
  }, [searchTodos, showErrorMessage, hideErrorMessage]);

  return (
    <DefaultLayout>
      <ErrorMessage visible={visibleErrorMessage} config={errorMessageConfig} />
      <Title order={2}>Todo list </Title>
      <StatusFilter />
      {todos.map((todo: Todo) => (
        <TodoListItem
          key={todo.id}
          todo={todo}
          onDeleteTodo={handleDeleteTodo}
          onNavigateToEditTodoPage={handleNavigateToEditPage}
        />
      ))}
      <ConfirmDialog
        config={confirmDialogConfig}
        visible={visibleConfirmDialog}
      />
    </DefaultLayout>
  );
}
