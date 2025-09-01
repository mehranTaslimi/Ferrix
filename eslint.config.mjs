import { dirname } from 'path';
import { fileURLToPath } from 'url';
import { FlatCompat } from '@eslint/eslintrc';
import eslintConfigPrettier from 'eslint-config-prettier/flat';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const compat = new FlatCompat({
  baseDirectory: __dirname,
});

// Base Next.js + TypeScript configs
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
  ...base,
  {
    files: ['**/*.{js,jsx,ts,tsx}'],
    rules: {
      /* General best practices */
      eqeqeq: ['error', 'smart'],
      curly: ['error', 'all'],
      'no-console': ['warn', { allow: ['warn', 'error'] }],
      'no-else-return': 'error',
      'prefer-const': 'error',

      /* TypeScript */
      '@typescript-eslint/no-unused-vars': [
        'error',
        { argsIgnorePattern: '^_', varsIgnorePattern: '^_' },
      ],
      '@typescript-eslint/consistent-type-imports': ['warn', { prefer: 'type-imports' }],

      /* Imports */
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

      /* React hooks */
      'react-hooks/rules-of-hooks': 'error',
      'react-hooks/exhaustive-deps': 'warn',
    },
  },
  eslintConfigPrettier,
];
