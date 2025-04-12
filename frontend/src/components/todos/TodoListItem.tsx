import { useCallback } from "react";
import { Flex, Paper, Text } from "@mantine/core";
import { Todo } from "@entity";
import { CompletedLabel } from "./CompletedLabel";
import { InProgressLabel } from "./InProgressLabel";
import { ReadyLabel } from "./ReadyLabel";
import { TodoListItemActions } from "./TodoListItemActions";

interface Props {
  todo: Todo;

  onDeleteTodo: (id: number) => void;

  onNavigateToEditTodoPage: (id: number) => void;
}

export function TodoListItem(props: Props) {
  const { todo, onDeleteTodo, onNavigateToEditTodoPage } = props;

  const handleDeleteTodo = useCallback(() => {
    onDeleteTodo(todo.id);
  }, [onDeleteTodo, todo.id]);

  const handleEditTodo = useCallback(() => {
    onNavigateToEditTodoPage(todo.id);
  }, [onNavigateToEditTodoPage, todo.id]);

  return (
    <Paper withBorder p="md">
      <Flex direction="row" justify="space-between">
        <ReadyLabel visible={todo.status === 0} />
        <InProgressLabel visible={todo.status === 1} />
        <CompletedLabel visible={todo.status === 2} />
        <TodoListItemActions
          onDeleteTodo={handleDeleteTodo}
          onEditTodo={handleEditTodo}
        />
      </Flex>
      <Text>{todo.title}</Text>
    </Paper>
  );
}
