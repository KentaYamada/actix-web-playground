import { act, renderHook } from "@testing-library/react";
import { vi } from "vitest";
import { useConfirmDialog } from "./useConfirmDialog";
import { ConfirmDialogConfig } from "./ConfirmDialogConfig";

describe("useConfirmDialog", () => {
  it("should initialize with default config and hidden state", () => {
    const { result } = renderHook(() => useConfirmDialog());

    expect(result.current.visibleConfirmDialog).toBe(false);
    expect(result.current.confirmDialogConfig).toEqual({
      content: null,
      onCancel: expect.any(Function),
    });
  });

  it("should update config and show dialog when openConfirmDialog is called", () => {
    const { result } = renderHook(() => useConfirmDialog());

    const mockOnCancel = vi.fn();
    const payload: ConfirmDialogConfig = {
      content: "Are you sure?",
      onCancel: mockOnCancel,
    };

    act(() => {
      result.current.openConfirmDialog(payload);
    });

    expect(result.current.visibleConfirmDialog).toBe(true);
    expect(result.current.confirmDialogConfig).toEqual(payload);
  });

  it("should hide dialog when closeConfirmDialog is called", () => {
    const { result } = renderHook(() => useConfirmDialog());

    const payload: ConfirmDialogConfig = {
      content: "Confirm this?",
      onCancel: vi.fn(),
    };

    act(() => {
      result.current.openConfirmDialog(payload);
    });

    expect(result.current.visibleConfirmDialog).toBe(true);

    act(() => {
      result.current.closeConfirmDialog();
    });

    expect(result.current.visibleConfirmDialog).toBe(false);
  });
});
