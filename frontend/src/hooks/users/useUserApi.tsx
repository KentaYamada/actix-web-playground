import axios from "axios";
import { useCallback } from "react";
import { User } from "@entity";

const ENDPOINT: string = "/api/users";

export function useUserApi() {
  const getUserApi = useCallback(async (id: number) => {
    return await axios.get(`${ENDPOINT}/${id}`, {
      headers: { "Content-Type": "application/json" },
      data: {},
    });
  }, []);

  const updateUserApi = useCallback(async (payload: User) => {
    return await axios.put(`${ENDPOINT}/${payload.id}`, payload, {
      headers: { "Content-Type": "application/json" },
    });
  }, []);

  return {
    getUserApi,
    updateUserApi,
  };
}
