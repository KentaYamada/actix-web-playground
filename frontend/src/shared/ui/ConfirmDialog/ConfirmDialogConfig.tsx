import { ReactNode } from "react";

export interface ConfirmDialogConfig {
  content: ReactNode | null;

  confirmButtonText?: string;

  cancelButtonText?: string;

  title?: string | ReactNode;

  onCancel: () => void;

  onConfirm?: () => void;
}
