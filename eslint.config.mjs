import { FlatCompat } from '@eslint/eslintrc';
import eslintConfigPrettier from 'eslint-config-prettier/flat';
import { dirname } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const compat = new FlatCompat({
  baseDirectory: __dirname,
});

const base = compat.extends(
  'next/core-web-vitals',
  'next/typescript',
  'plugin:@typescript-eslint/recommended',
  'plugin:import/recommended',
  'plugin:import/typescript',
  'plugin:jsx-a11y/recommended',
  'plugin:react-hooks/recommended',
);

export default [
  {
    ignores: [
      'components/ui/',
      'node_modules/',
      '.pnp/',
      '.next/',
      'out/',
      'dist/',
      'src-tauri/',
      '*.log',
      '.env',
      '.env*.local',
      'coverage/',
      '*.lcov',
      '.vscode/',
      '.idea/',
      '*.bak',
      '*.tmp',
    ],
  },
  ...base,
  {
    files: ['**/*.{js,jsx,ts,tsx}'],
    settings: {
      'import/resolver': {
        typescript: { project: true },
        node: true,
      },
    },
    rules: {
      eqeqeq: ['error', 'smart'],
      curly: ['error', 'all'],
      'no-console': ['warn', { allow: ['warn', 'error'] }],
      'no-else-return': 'error',
      'prefer-const': 'error',
      '@typescript-eslint/no-unused-vars': [
        'error',
        { argsIgnorePattern: '^_', varsIgnorePattern: '^_' },
      ],
      '@typescript-eslint/consistent-type-imports': ['warn', { prefer: 'type-imports' }],
      'import/no-unresolved': 'error',
      'import/no-duplicates': 'error',
      'import/order': [
        'error',
        {
          groups: ['builtin', 'external', 'internal', 'parent', 'sibling', 'index', 'type'],
          'newlines-between': 'always',
          alphabetize: { order: 'asc', caseInsensitive: true },
        },
      ],
      'import/newline-after-import': ['error', { count: 1 }],
      'react-hooks/rules-of-hooks': 'error',
      'react-hooks/exhaustive-deps': 'warn',
    },
  },
  eslintConfigPrettier,
];
