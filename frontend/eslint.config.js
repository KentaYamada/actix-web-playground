import js from "@eslint/js";
import globals from "globals";
import mantine from "eslint-config-mantine";
import jsxA11y from "eslint-plugin-jsx-a11y";
import react from "eslint-plugin-react";
import reactHooks from "eslint-plugin-react-hooks";
import reactRefresh from "eslint-plugin-react-refresh";
import tseslint from "typescript-eslint";
import tsParser from "@typescript-eslint/parser";
import eslintPrettierRecommended from "eslint-plugin-prettier/recommended";
import vitest from "@vitest/eslint-plugin";

export default tseslint.config(
  ...mantine,
  eslintPrettierRecommended,
  jsxA11y.flatConfigs.recommended,
  {
    ignores: [
      "node_modules",
      "dist",
      "vite.config.ts",
      "vitest.config.ts",
      "vitest.setup.ts",
    ],
  },
  {
    extends: [js.configs.recommended, ...tseslint.configs.recommended],
    files: ["**/*.{ts,tsx}"],
    languageOptions: {
      globals: {
        ...globals.browser,
      },
      parser: tsParser,
      ecmaVersion: "latest",
      sourceType: "module",
      parserOptions: {
        ecmaFeatures: {
          jsx: true,
        },
        project: "./tsconfig.app.json",
      },
    },
    plugins: {
      react,
      "react-hooks": reactHooks,
      "react-refresh": reactRefresh,
    },
    rules: {
      complexity: ["error", 10],
      ...reactHooks.configs.recommended.rules,
      "no-console": "warn",
      "react/require-default-props": "off",
      "react/jsx-props-no-spreading": "off",
      "react/jsx-uses-react": "off",
      "react/jsx-key": [
        "error",
        {
          checkFragmentShorthand: true,
          warnOnDuplicates: true,
        },
      ],
      "react/react-in-jsx-scope": "off",
      "react-refresh/only-export-components": [
        "warn",
        { allowConstantExport: true },
      ],
      "react-hooks/rules-of-hooks": "error",
      "react-hooks/exhaustive-deps": "warn",
    },
    settings: {
      react: {
        version: "detect",
      },
    },
  },
  {
    files: ["src/**/*.test.{ts,tsx}", "tests/**"],
    plugins: { vitest },
    rules: {
      ...vitest.configs.recommended.rules,
      "vitest/max-nested-describe": ["error", { max: 2 }],
    },
    settings: {
      vitest: {
        typecheck: true,
      },
    },
    languageOptions: {
      globals: {
        ...vitest.environments.env.globals,
      },
    },
  },
);
