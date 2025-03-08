import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { BrowserRouter, Routes, Route } from "react-router";
import { TodoCreatePage, TodoListPage, UserViewPage } from "@pages";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/todos" element={<TodoListPage />} />
        <Route path="/todos/create" element={<TodoCreatePage />} />
        <Route path="/users/:id" element={<UserViewPage />} />
      </Routes>
    </BrowserRouter>
  </StrictMode>,
);
