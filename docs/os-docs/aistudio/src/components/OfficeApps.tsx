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
