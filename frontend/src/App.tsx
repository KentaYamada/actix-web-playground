import "@mantine/core/styles.css";
import { StrictMode } from "react";
import { BrowserRouter, Routes, Route } from "react-router";
import { MantineProvider } from "@mantine/core";
import {
  TodoCreatePage,
  TodoEditPage,
  TodoListPage,
  UserViewPage,
  SigninPage,
  SignupPage,
} from "@pages";

export default function App() {
  return (
    <StrictMode>
      <MantineProvider withGlobalClasses>
        <BrowserRouter>
          <Routes>
            <Route path="/auth/signin" element={<SigninPage />} />
            <Route path="/auth/signup" element={<SignupPage />} />
            <Route path="/todos" element={<TodoListPage />} />
            <Route path="/todos/create" element={<TodoCreatePage />} />
            <Route path="/todos/:id/edit" element={<TodoEditPage />} />
            <Route path="/users/:id" element={<UserViewPage />} />
          </Routes>
        </BrowserRouter>
      </MantineProvider>
    </StrictMode>
  );
}
