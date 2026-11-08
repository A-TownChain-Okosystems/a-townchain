// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import React from 'react';
import { Layers } from 'lucide-react';

// NOT IMPLEMENTED — Platzhalter-View (SCR-0078): Referenziert in App/DesktopApp, fachliche
// Umsetzung folgt im jeweiligen Sprint. Ehrlicher Platzhalter statt Build-Brecher.
export function BlockchainEcosystemView(_props: any) {
  return (
    <div className="flex flex-col items-center justify-center h-full text-slate-400 gap-2 p-8">
      <Layers className="w-8 h-8" />
      <h2 className="text-base font-semibold text-slate-200">BlockchainEcosystemView</h2>
      <span className="text-sm">NICHT IMPLEMENTIERT — Umsetzung folgt (siehe REALITY_STATUS.md).</span>
    </div>
  );
}
