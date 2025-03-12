import {
  Box,
  Button,
  Container,
  Flex,
  Input,
  InputWrapper,
} from "@mantine/core";

export function SignupPage() {
  return (
    <Container>
      <Box>
        <form>
          <InputWrapper label="ユーザー名" required>
            <Input type="text" placeholder="Taro" />
          </InputWrapper>
          <InputWrapper label="メールアドレス" required>
            <Input type="text" placeholder="taro@email.com" />
          </InputWrapper>

          <Flex
            mt={12}
            align="center"
            direction="row"
            gap="md"
            justify="flex-end"
            wrap="wrap"
          >
            <Button>アカウント作成</Button>
          </Flex>
        </form>
      </Box>
    </Container>
  );
}
