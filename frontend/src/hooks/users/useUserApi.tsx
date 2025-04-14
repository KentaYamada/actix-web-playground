import axios from "axios";
import { useCallback } from "react";
import { User } from "@entity";

const ENDPOINT: string = "/api/users";

export function useUserApi() {
  const getUserApi = useCallback(async (id: number) => {
    return await axios.get(`${ENDPOINT}/${id}`, {
      headers: { Accept: "application/json" },
    });
  }, []);

  const updateUserApi = useCallback(async (payload: User) => {
    return await axios.put(`${ENDPOINT}/${payload.id}`, payload, {
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
    });
  }, []);

  return {
    getUserApi,
    updateUserApi,
  };
}
