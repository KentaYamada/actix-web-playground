import { useCallback, useState } from "react";
import { useDisclosure } from "@mantine/hooks";
import { ErrorMessageConfig } from "./ErrorMessageConfig";

export function useErrorMessage() {
  const [opened, { open, close }] = useDisclosure(false);
  const [config, setConfig] = useState<ErrorMessageConfig>({
    title: "",
    message: "",
  });

  const hideErrorMessage = useCallback(() => {
    close();
  }, [close]);

  const showErrorMessage = useCallback(
    (config: ErrorMessageConfig) => {
      setConfig(config);
      open();
    },
    [open],
  );

  return {
    errorMessageConfig: config,
    visibleErrorMessage: opened,
    hideErrorMessage,
    showErrorMessage,
  };
}
