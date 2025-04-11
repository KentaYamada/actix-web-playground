import { Button, Flex, Select, Textarea, TextInput } from "@mantine/core";
import { UseFormReturnType } from "@mantine/form";
import { IconPlus } from "@tabler/icons-react";

interface FormValues {
  status: number;
  title: string;
  detail: string;
}

interface Props {
  form: UseFormReturnType<FormValues>;
  onSave: () => void;
}

export function TodoForm(props: Props) {
  const { form, onSave } = props;

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
      <TextInput
        label="タイトル"
        withAsterisk
        key={form.key("title")}
        {...form.getInputProps("title")}
      />
      <Textarea label="詳細" rows={10} />
      <Flex mt="md" justify="end" align="center">
        <Button leftSection={<IconPlus size={14} />} onClick={onSave}>
          追加
        </Button>
      </Flex>
    </form>
  );
}
