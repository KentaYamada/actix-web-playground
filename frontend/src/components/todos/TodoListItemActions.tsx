import { ActionIcon, Menu } from "@mantine/core";
import { IconDotsVertical, IconEdit, IconTrash } from "@tabler/icons-react";

interface Props {
  onEditTodo: () => void;

  onDeleteTodo: () => void;
}

export function TodoListItemActions(props: Props) {
  const { onDeleteTodo, onEditTodo } = props;

  return (
    <Menu position="bottom-end">
      <Menu.Target>
        <ActionIcon variant="subtle" color="gray" aria-label="Actions">
          <IconDotsVertical size="1.25rem" />
        </ActionIcon>
      </Menu.Target>
      <Menu.Dropdown>
        <Menu.Item
          color="red"
          leftSection={<IconTrash size="1rem" />}
          onClick={onDeleteTodo}
        >
          削除
        </Menu.Item>
        <Menu.Item
          color="blue"
          leftSection={<IconEdit size="1rem" />}
          onClick={onEditTodo}
        >
          編集
        </Menu.Item>
      </Menu.Dropdown>
    </Menu>
  );
}
