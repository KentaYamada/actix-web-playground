import { useCallback, useState } from "react";
import { useDisclosure } from "@mantine/hooks";
import { ConfirmDialogConfig } from "./ConfirmDialogConfig";

export function useConfirmDialog() {
  const [opened, { open, close }] = useDisclosure(false);
  const [config, setConfig] = useState<ConfirmDialogConfig>({
    content: null,
    onCancel: () => {}, // noop
  });

  const openConfirmDialog = useCallback(
    (payload: ConfirmDialogConfig) => {
      setConfig(payload);
      open();
    },
    [open],
  );

  const closeConfirmDialog = useCallback(() => {
    close();
  }, [close]);

  return {
    confirmDialogConfig: config,
    visibleConfirmDialog: opened,
    openConfirmDialog,
    closeConfirmDialog,
  };
}
