import { ReactElement } from "react";
import { render } from "@testing-library/react";
import { MantineProvider, Text } from "@mantine/core";
import { ErrorMessage } from "./ErrorMessage";
import { ErrorMessageConfig } from "./ErrorMessageConfig";

describe("ErrorMessage component tests", () => {
  it.each(["An error occured", <Text key="a">An error occured</Text>])(
    "should render when visible is true",
    (message: string | ReactElement) => {
      const config: ErrorMessageConfig = {
        title: "Error",
        message,
      };

      const { getByText } = render(<ErrorMessage config={config} visible />, {
        wrapper: MantineProvider,
      });

      expect(getByText("Error")).toBeInTheDocument();
      expect(getByText("An error occured")).toBeInTheDocument();
    },
  );

  it.each(["An error occured", <Text key="a">An error occured</Text>])(
    "should not render when visible is false",
    (message: string | ReactElement) => {
      const config: ErrorMessageConfig = {
        title: "Error",
        message,
      };

      const { queryByRole } = render(
        <ErrorMessage config={config} visible={false} />,
        {
          wrapper: MantineProvider,
        },
      );

      expect(queryByRole("alert")).toBeNull();
    },
  );
});
