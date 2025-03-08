import { renderHook } from "@testing-library/react";
import { useTodoForm } from "./useTodoForm";
import { vi } from "vitest";

const zodResolver = vi.hoisted(() => vi.fn());

vi.mock("@hookform/resolvers/zod", async () => {
  const actual = await vi.importActual("@hookform/resolvers/zod");

  return {
    ...actual,
    zodResolver,
  };
});

describe("useTodoForm hook tests", () => {
  it("should return initial form values", () => {
    const { result } = renderHook(() => useTodoForm());

    expect(result.current.control).toBeDefined();
    expect(result.current.errors).toEqual({});
    expect(result.current.invalid).toBe(false);
    expect(result.current.isSubmitted).toBe(false);
  });
});
