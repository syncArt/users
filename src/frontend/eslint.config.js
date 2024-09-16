import globals from "globals";
import reactHooks from "eslint-plugin-react-hooks";
import reactRefresh from "eslint-plugin-react-refresh";
import eslint from '@eslint/js';
import * as tseslint from "typescript-eslint";

export default tseslint.config(eslint.configs.recommended, {
  plugins: {
    "@typescript-eslint": tseslint.plugin,
    "react-hooks": reactHooks,
    "react-refresh": reactRefresh,
  },
  languageOptions: {
    parser: tseslint.parser,
    globals: globals.browser,
    parserOptions: {
      ecmaVersion: 2020,
      projectService: true,
      tsconfigRootDir: import.meta.dirname,
    },
  },
  files: ["**/*.{ts,tsx}"],
  rules: {
    ...reactHooks.configs.recommended.rules,
    "react-refresh/only-export-components": [
      "warn",
      { allowConstantExport: true },
    ],
    "@typescript-eslint/no-unsafe-argument": "error",
    "@typescript-eslint/no-unsafe-assignment": "error",
    "@typescript-eslint/no-unsafe-call": "error",
    "@typescript-eslint/no-unsafe-member-access": "error",
    "@typescript-eslint/no-unsafe-return": "error",
    "no-unused-vars": "warn"
  },
  ignores: ["dist"],
  linterOptions: {
    reportUnusedDisableDirectives: "warn",
  },
});
