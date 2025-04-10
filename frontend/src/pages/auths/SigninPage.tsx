import {
  Box,
  Button,
  Container,
  Center,
  Paper,
  TextInput,
  Title,
} from "@mantine/core";

export function SigninPage() {
  return (
    <Container>
      <Center h="100vh">
        <Paper withBorder p="md">
          <Title order={4}>サインイン</Title>
          <form>
            <TextInput label="ユーザー名" placeholder="Taro" withAsterisk />
            <TextInput
              label="メールアドレス"
              placeholder="taro@email.com"
              withAsterisk
            />
            <Box mt="md">
              <Button fullWidth>サインイン</Button>
            </Box>
          </form>
        </Paper>
      </Center>
    </Container>
  );
}
