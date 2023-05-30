/* eslint-disable @typescript-eslint/no-var-requires */
/* eslint-disable @typescript-eslint/no-require-imports */
/* eslint-disable no-undef */
import type { Config } from '@jest/types';
// const nextJest = ('next/jest');
import nextJest from 'next/jest';
// import type { JestConfigWithTsJest } from 'ts-jest';

const createJestConfig = nextJest({
  dir: './',
});

// /** @type {import('ts-jest/dist/types').InitialOptionsTsJest} */
const customJestConfig: Config.InitialOptions = {
  collectCoverageFrom: ['**/*.{js,ts,jsx,tsx}', '!**/*.d.ts', '!**/node_modules/**'],
  modulePaths: ['<rootDir>/src'],
  moduleFileExtensions: ['ts', 'js', 'tsx', 'jsx', 'json', 'node'],
  moduleNameMapper: {
    '@/(.*)$': '<rootDir>/src/$1',
  },
  // preset: 'ts-jest',
  rootDir: './',
  roots: ['<rootDir>/src'],
  setupFiles: ['dotenv/config'],
  testEnvironment: 'jest-environment-jsdom',
  testPathIgnorePatterns: ['node_modules/', 'coverage/', '.next/'],
  transform: {
    '^.+\\.(ts|tsx)$': 'ts-jest',
  },
};

export default createJestConfig(customJestConfig);
// module.exports = createJestConfig(customJestConfig);
