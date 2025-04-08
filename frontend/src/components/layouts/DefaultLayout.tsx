import { ReactNode } from "react";
import { Link } from "react-router";
import { AppShell, Burger, NavLink } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";

interface Props {
  children: ReactNode;
}

export function DefaultLayout(props: Props) {
  const { children } = props;
  const [opened, { toggle }] = useDisclosure();

  return (
    <AppShell
      header={{ height: 60 }}
      navbar={{
        width: 300,
        breakpoint: "sm",
        collapsed: { mobile: !opened },
      }}
      padding="md"
    >
      <AppShell.Header>
        <Burger hiddenFrom="sm" size="sm" opened={opened} onClick={toggle} />
        <div>Logo</div>
      </AppShell.Header>

      <AppShell.Navbar p="md">
        <NavLink label="Todo" defaultOpened>
          <NavLink label="一覧" component={Link} to="/todos" />
          <NavLink label="追加" component={Link} to="/todos/create" />
        </NavLink>
        <NavLink label="アカウント">
          <NavLink label="詳細" component={Link} to="/users/1" />
        </NavLink>
      </AppShell.Navbar>

      <AppShell.Main>{children}</AppShell.Main>
    </AppShell>
  );
}
