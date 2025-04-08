import { Checkbox, Group } from "@mantine/core";

export function StatusFilter() {
  return (
    <Checkbox.Group>
      <Group mt="xs">
        <Checkbox value="0" label="In Ready" />
        <Checkbox value="1" label="In Progress" />
        <Checkbox value="2" label="Completed" />
      </Group>
    </Checkbox.Group>
  );
}
