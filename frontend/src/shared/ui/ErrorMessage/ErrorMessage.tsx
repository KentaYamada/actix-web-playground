import { Alert } from "@mantine/core";
import { IconInfoCircle } from "@tabler/icons-react";
import { ErrorMessageConfig } from "./ErrorMessageConfig";

interface Props {
  visible: boolean;
  config: ErrorMessageConfig;
}

export function ErrorMessage(props: Props) {
  const { config, visible } = props;

  if (!visible) {
    return null;
  }

  return (
    <Alert color="red" title={config.title} icon={<IconInfoCircle />}>
      {config.message}
    </Alert>
  );
}
