import {
  Box,
  Button,
  Container,
  Center,
  Paper,
  TextInput,
  Title,
} from "@mantine/core";

export function SignupPage() {
  return (
    <Container>
      <Center h="100vh">
        <Paper withBorder p="md">
          <Title order={4}>アカウント作成</Title>
          <form>
            <TextInput label="ユーザー名" placeholder="Taro" withAsterisk />
            <TextInput
              label="メールアドレス"
              placeholder="taro@email.com"
              withAsterisk
            />
            <Box mt="md">
              <Button fullWidth>アカウント作成</Button>
            </Box>
          </form>
        </Paper>
      </Center>
    </Container>
  );
}
