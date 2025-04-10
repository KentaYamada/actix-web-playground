import { Button, Flex, Select, Textarea, TextInput } from "@mantine/core";
import { IconPlus } from "@tabler/icons-react";

export function TodoForm() {
  return (
    <form autoComplete="off">
      <Select
        defaultValue="0"
        label="ステータス"
        withAsterisk
        data={[
          { value: "0", label: "未着手" },
          { value: "1", label: "対応中" },
          { value: "2", label: "完了" },
        ]}
      />
      <TextInput label="タイトル" withAsterisk />
      <Textarea label="詳細" rows={10} />
      <Flex mt="md" justify="end" align="center">
        <Button leftSection={<IconPlus size={14} />}>追加</Button>
      </Flex>
    </form>
  );
}
