import { AxiosError } from "axios";

interface ApiErrorResponseBody {
  message: string;
}

export type ApiResponseError = AxiosError<ApiErrorResponseBody>;
