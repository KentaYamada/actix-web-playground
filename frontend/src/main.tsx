import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { BrowserRouter, Routes, Route } from "react-router";
import "./index.css";
import App from "./App.tsx";
import { TodoListPage, UserViewPage } from "@pages";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<App />} />
        <Route path="/todos" element={<TodoListPage />} />
        <Route path="/users/:id" element={<UserViewPage />} />
      </Routes>
    </BrowserRouter>
  </StrictMode>,
);
