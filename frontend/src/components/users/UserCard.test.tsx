import { render } from "@testing-library/react";
import { MantineProvider } from "@mantine/core";
import { User } from "@entity";
import { UserCard } from "./UserCard";

describe("UserCard component tests", () => {
  it("should render fullname & email", () => {
    const user: User = {
      id: 1,
      first_name: "Hanako",
      family_name: "Todo",
      email: "todo.hanako@email.com",
      password: "foobar",
    };

    const { getByText } = render(<UserCard user={user} />, {
      wrapper: MantineProvider,
    });

    expect(getByText("Todo Hanako")).toBeInTheDocument();
    expect(getByText("todo.hanako@email.com")).toBeInTheDocument();
    expect(() => getByText("foobar")).toThrow();
  });
});
