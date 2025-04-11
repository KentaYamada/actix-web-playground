import { Alert } from "@mantine/core";
import { IconInfoCircle } from "@tabler/icons-react";

interface Props {
  title: string;
  message: string;
  visible: boolean;
}

export function ErrorMessage(props: Props) {
  const { title, message, visible } = props;

  if (!visible) {
    return null;
  }

  return (
    <Alert color="red" title={title} icon={<IconInfoCircle />}>
      {message}
    </Alert>
  );
}
