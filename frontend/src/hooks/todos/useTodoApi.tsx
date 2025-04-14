import axios from "axios";
import { useCallback } from "react";
import { Todo } from "@entity";

const ENDPOINT: string = "/api/todos";

export function useTodoApi() {
  const createTodoApi = useCallback(async (payload: Todo) => {
    return await axios.post(ENDPOINT, payload, {
      headers: { "Content-Type": "application/json" },
    });
  }, []);

  const deleteTodo = useCallback(async (id: number) => {
    return await axios.delete(`${ENDPOINT}/${id}`, {
      headers: { "Content-Type": "application/json" },
    });
  }, []);

  const searchTodos = useCallback(async () => {
    return await axios.get(ENDPOINT, {
      headers: { "Content-Type": "application/json" },
      data: {},
    });
  }, []);

  return {
    createTodoApi,
    searchTodos,
    deleteTodo,
  } as const;
}
