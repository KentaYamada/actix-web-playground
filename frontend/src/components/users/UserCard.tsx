import { Button, Card, Divider, Text, Title } from "@mantine/core";
import { User } from "@entity";

interface Props {
  user: User;

  onNavigateToEditPage: () => void;
}

export function UserCard(props: Props) {
  const { user, onNavigateToEditPage } = props;

  return (
    <Card>
      <Title order={3}>{`${user.family_name} ${user.first_name}`}</Title>
      <Text>{user.email}</Text>
      <Divider mt="md" />
      <Button color="blue" mt="md" onClick={onNavigateToEditPage}>
        編集
      </Button>
    </Card>
  );
}
