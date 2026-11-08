// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// A-TownChain OS — Frontend Test Config (K7.11)
module.exports = {
  testEnvironment: 'jsdom',
  testMatch: ['<rootDir>/**/*.test.{js,ts,tsx}'],
  collectCoverageFrom: ['<rootDir>/**/*.{js,ts,tsx}', '!**/*.test.*'],
  coverageDirectory: '<rootDir>/coverage',
  setupFilesAfterEnv: ['<rootDir>/jest.setup.js'],
  moduleNameMapper: {
    '\\.(css|less|scss)$': '<rootDir>/__mocks__/styleMock.js'
  },
  transform: {
    '^.+\\.(ts|tsx)$': 'ts-jest'
  }
};
