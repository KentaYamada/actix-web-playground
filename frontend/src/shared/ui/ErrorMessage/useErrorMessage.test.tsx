import { act, renderHook } from "@testing-library/react";
import { useErrorMessage } from "./useErrorMessage";

describe("useErrorMessage hook tests", () => {
  it("should initialize with empty config and invisible state", () => {
    const { result } = renderHook(() => useErrorMessage());

    expect(result.current.errorMessageConfig).toEqual({
      title: "",
      message: "",
    });
    expect(result.current.visibleErrorMessage).toBe(false);
  });

  it("should show error message and update config when showErrorMessage is called", () => {
    const { result } = renderHook(() => useErrorMessage());

    act(() => {
      result.current.showErrorMessage({
        title: "Error",
        message: "Something went wrong",
      });
    });

    expect(result.current.errorMessageConfig).toEqual({
      title: "Error",
      message: "Something went wrong",
    });
    expect(result.current.visibleErrorMessage).toBe(true);
  });

  it("should hide error message when hideErrorMessage is called", () => {
    const { result } = renderHook(() => useErrorMessage());

    act(() => {
      result.current.showErrorMessage({
        title: "Error",
        message: "Something went wrong",
      });
    });

    act(() => {
      result.current.hideErrorMessage();
    });

    expect(result.current.visibleErrorMessage).toBe(false);
  });
});
