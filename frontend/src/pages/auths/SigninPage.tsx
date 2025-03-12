import {
  Box,
  Button,
  Container,
  Flex,
  Input,
  InputWrapper,
  PasswordInput,
} from "@mantine/core";

export function SigninPage() {
  return (
    <Container>
      <Box>
        <form>
          <InputWrapper label="メールアドレス" required>
            <Input type="text" placeholder="taro@email.com" />
          </InputWrapper>
          <InputWrapper label="パスワード" required>
            <PasswordInput />
          </InputWrapper>

          <Flex
            mt={12}
            align="center"
            direction="row"
            gap="md"
            justify="flex-end"
            wrap="wrap"
          >
            <Button>サインイン</Button>
          </Flex>
        </form>
      </Box>
    </Container>
  );
}
