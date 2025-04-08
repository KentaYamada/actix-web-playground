import { Badge } from "@mantine/core";

interface Props {
  visible: boolean;
}

export function InProgressLabel(props: Props) {
  const { visible } = props;

  if (!visible) {
    return null;
  }

  return (
    <Badge size="xs" color="orange">
      In Progress
    </Badge>
  );
}
