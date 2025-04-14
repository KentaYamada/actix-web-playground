import { Button, Flex, PasswordInput, TextInput } from "@mantine/core";
import { UseFormReturnType } from "@mantine/form";
import { IconPlus } from "@tabler/icons-react";
import { UserFormValuesType } from "@hooks";

interface Props {
  form: UseFormReturnType<UserFormValuesType>;
  onSave: () => void;
}

export function UserForm(props: Props) {
  const { form, onSave } = props;

  return (
    <form autoComplete="off">
      <TextInput
        label="姓"
        withAsterisk
        key={form.key("family_name")}
        {...form.getInputProps("family_name")}
      />
      <TextInput
        label="名"
        withAsterisk
        key={form.key("first_name")}
        {...form.getInputProps("first_name")}
      />
      <TextInput
        label="メールアドレス"
        withAsterisk
        key={form.key("email")}
        {...form.getInputProps("email")}
      />
      <PasswordInput
        label="パスワード"
        withAsterisk
        key={form.key("password")}
        {...form.getInputProps("password")}
      />
      <Flex mt="md" justify="end" align="center">
        <Button leftSection={<IconPlus size={14} />} onClick={onSave}>
          保存
        </Button>
      </Flex>
    </form>
  );
}
