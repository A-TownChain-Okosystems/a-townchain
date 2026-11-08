// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import React from 'react';
import { OfficeSuiteView } from './OfficeSuiteView';

export function ATCWordView() {
  return <OfficeSuiteView defaultAppType="docs" />;
}

export function ATCExcelView() {
  return <OfficeSuiteView defaultAppType="sheets" />;
}

export function ATCPowerPointView() {
  return <OfficeSuiteView defaultAppType="slides" />;
}
