import { Link } from "react-router";
import { Center, Container, Paper, Text, Title } from "@mantine/core";

export function NotfoundPage() {
  return (
    <Container>
      <Center h="100vh">
        <Paper withBorder p="xl" style={{ textAlign: "center" }}>
          <Title order={2}>404</Title>
          <Title order={3}>NotFound</Title>
          <Text mt="md">対象のページまたはデータが見つかりませんでした</Text>
          <Text mt="md">
            <Link to="/">トップページへ</Link>
          </Text>
        </Paper>
      </Center>
    </Container>
  );
}
