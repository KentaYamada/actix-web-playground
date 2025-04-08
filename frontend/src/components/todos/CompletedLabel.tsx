import { Badge } from "@mantine/core";

interface Props {
  visible: boolean;
}

export function CompletedLabel(props: Props) {
  const { visible } = props;

  if (!visible) {
    return null;
  }

  return (
    <Badge size="xs" color="green">
      Completed
    </Badge>
  );
}
