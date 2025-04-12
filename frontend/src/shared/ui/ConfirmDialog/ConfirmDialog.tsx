import { Button, Divider, Flex, Group, Modal } from "@mantine/core";
import { ConfirmDialogConfig } from "./ConfirmDialogConfig";

interface Props {
  config: ConfirmDialogConfig;
  visible: boolean;
}

export function ConfirmDialog(props: Props) {
  const { config, visible } = props;

  return (
    <Modal
      opened={visible}
      onClose={config.onCancel}
      title={config.title && <Group gap="xs">{config.title}</Group>}
      centered
    >
      <Modal.Body>{config.content}</Modal.Body>
      <Divider mt="sm" />
      <Flex justify="flex-end" direction="row" align="center" gap="sm" mt="sm">
        <Button variant="outline" color="dark" onClick={config.onCancel}>
          {config.cancelButtonText ?? "いいえ"}
        </Button>
        <Button variant="outline" color="red" onClick={config.onConfirm}>
          {config.confirmButtonText ?? "はい"}
        </Button>
      </Flex>
    </Modal>
  );
}
