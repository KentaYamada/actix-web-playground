import { render, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { vi } from "vitest";
import { MantineProvider } from "@mantine/core";
import { ConfirmDialog } from "./ConfirmDialog";
import { ConfirmDialogConfig } from "./ConfirmDialogConfig";

describe("ConfirmDialog component tests", () => {
  const baseConfig: ConfirmDialogConfig = {
    title: "確認",
    content: "本当に削除しますか？",
    onCancel: vi.fn(),
    onConfirm: vi.fn(),
    cancelButtonText: "キャンセル",
    confirmButtonText: "削除",
  };

  it("should not render when visible is false", () => {
    const { queryByText } = render(
      <ConfirmDialog config={baseConfig} visible={false} />,
      {
        wrapper: MantineProvider,
      },
    );

    expect(queryByText("本当に削除しますか？")).toBeNull();
  });

  it("should render title, content, and buttons when visible is true", () => {
    const { getByText } = render(
      <ConfirmDialog config={baseConfig} visible />,
      {
        wrapper: MantineProvider,
      },
    );

    expect(getByText("確認")).toBeInTheDocument();
    expect(getByText("本当に削除しますか？")).toBeInTheDocument();
    expect(getByText("キャンセル")).toBeInTheDocument();
    expect(getByText("削除")).toBeInTheDocument();
  });

  it("should call onCancel when cancel button is clicked", async () => {
    const { getByText } = render(
      <ConfirmDialog config={baseConfig} visible />,
      {
        wrapper: MantineProvider,
      },
    );

    const cancelButton = getByText("キャンセル");

    await userEvent.click(cancelButton);

    await waitFor(() => {
      expect(baseConfig.onCancel).toHaveBeenCalled();
    });
  });

  it("should call onConfirm when confirm button is clicked", async () => {
    const { getByText } = render(
      <ConfirmDialog config={baseConfig} visible />,
      {
        wrapper: MantineProvider,
      },
    );

    const confirmButton = getByText("削除");

    await userEvent.click(confirmButton);

    await waitFor(() => {
      expect(baseConfig.onConfirm).toHaveBeenCalled();
    });
  });
});
