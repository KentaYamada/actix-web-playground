import { Badge } from "@mantine/core";

interface Props {
  visible: boolean;
}

export function ReadyLabel(props: Props) {
  const { visible } = props;

  if (!visible) {
    return null;
  }

  return (
    <Badge size="xs" color="gray">
      In Ready
    </Badge>
  );
}
