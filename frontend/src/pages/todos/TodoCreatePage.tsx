import { useMemo } from "react";
import { Controller } from "react-hook-form";
import { useTodoForm, TodoFormValuesType } from "@hooks/todos";

export function TodoCreatePage() {
  const { control, errors, invalid, isSubmitted, handleSubmit } = useTodoForm();

  const visibleErrorMessage = useMemo(
    function () {
      return isSubmitted && invalid;
    },
    [isSubmitted, invalid],
  );

  const handleCreateTodo = handleSubmit(function (data: TodoFormValuesType) {
    console.log(data);
  });

  return (
    <>
      <h2>Todo作成</h2>
      {visibleErrorMessage && <p>入力エラー</p>}
      <form autoComplete="off">
        <div>
          <div>
            <label htmlFor="status">ステータス</label>
          </div>
          <div>
            <input
              type="radio"
              id="not-ready"
              name="status"
              value="0"
              checked
            />
            <label htmlFor="not-ready">未着手</label>
            <input type="radio" id="in-progress" name="status" value="1" />
            <label htmlFor="in-progress">対応中</label>
            <input type="radio" id="done" name="status" value="2" />
            <label htmlFor="done">完了</label>
          </div>
        </div>
        <div>
          <div>
            <label htmlFor="title">タイトル</label>
          </div>
          <Controller
            control={control}
            name="title"
            render={({ field }) => (
              <>
                <input type="text" id="title" {...field} />
                {errors && <small>{errors.title?.message}</small>}
              </>
            )}
          />
        </div>
        <div>
          <div>
            <label htmlFor="title">詳細</label>
          </div>
          <Controller
            control={control}
            name="detail"
            render={({ field }) => (
              <>
                <textarea id="detail" {...field} />
                {errors && <small>{errors.detail?.message}</small>}
              </>
            )}
          />
        </div>
        <div>
          <button type="submit" onClick={handleCreateTodo}>
            保存
          </button>
        </div>
      </form>
    </>
  );
}
