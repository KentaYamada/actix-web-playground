import axios from "axios";
import { useCallback } from "react";
import { Todo } from "@entity";

const ENDPOINT: string = "/api/todos";

export function useTodoApi() {
  const createTodoApi = useCallback(async (payload: Todo) => {
    return await axios.post(ENDPOINT, payload, {
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
    });
  }, []);

  const deleteTodo = useCallback(async (id: number) => {
    return await axios.delete(`${ENDPOINT}/${id}`, {
      headers: { Accept: "application/json" },
    });
  }, []);

  const getTodoApi = useCallback(async (id: number) => {
    return await axios.get(`${ENDPOINT}/${id}`, {
      headers: { Accept: "application/json" },
    });
  }, []);

  // todo: search parameters
  const searchTodos = useCallback(async () => {
    return await axios.get(ENDPOINT, {
      headers: { Accept: "application/json" },
    });
  }, []);

  const updateTodoApi = useCallback(async (payload: Todo) => {
    return await axios.put(`${ENDPOINT}/${payload.id}`, payload, {
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
    });
  }, []);

  return {
    createTodoApi,
    deleteTodo,
    getTodoApi,
    searchTodos,
    updateTodoApi,
  } as const;
}
