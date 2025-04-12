import { useCallback, useState } from "react";
import { ErrorMessageConfig } from "./ErrorMessageConfig";

export function useErrorMessage() {
  const [config, setConfig] = useState<ErrorMessageConfig>({
    title: "",
    message: "",
  });
  const [visible, setVisible] = useState<boolean>(false);

  const hideErrorMessage = useCallback(() => {
    setVisible(false);
  }, []);

  const showErrorMessage = useCallback((config: ErrorMessageConfig) => {
    setConfig(config);
    setVisible(true);
  }, []);

  return {
    errorMessageConfig: config,
    visibleErrorMessage: visible,
    hideErrorMessage,
    showErrorMessage,
  };
}
