import axios from "axios";
import { useCallback } from "react";

const ENDPOINT: string = "/api/todos";

export function useTodoApi() {
  const searchTodos = useCallback(async () => {
    return await axios.get(ENDPOINT, {
      headers: { "Content-Type": "application/json" },
      data: {},
    });
  }, []);

  const deleteTodo = useCallback(async (id: number) => {
    return await axios.delete(`${ENDPOINT}/${id}`, {
      headers: { "Content-Type": "application/json" },
    });
  }, []);

  return {
    searchTodos,
    deleteTodo,
  } as const;
}
