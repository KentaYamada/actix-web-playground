import axios from "axios";
import { useEffect, useState } from "react";
import { Todo } from "../../entity";

export function TodoListPage() {
  const [todos, setTodos] = useState<Todo[]>([]);

  useEffect(function () {
    axios
      .get("/api/todos", {
        headers: { "Content-Type": "application/json" },
        data: {},
      })
      .then(function (res) {
        setTodos(res.data.data.todos);
      })
      .catch(function (err) {
        console.error(err);
      });
  }, []);

  return (
    <>
      <h2>Todo list </h2>
      <ul>
        {todos.map(function (t) {
          return (
            <li key={t.id}>
              <h4>{t.status}</h4>
              <p>{t.title}</p>
            </li>
          );
        })}
      </ul>
    </>
  );
}
