// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import React from 'react';
import { Database, Layers } from 'lucide-react';

// NOT IMPLEMENTED — Platzhalter-View (SCR-0078): Registriert in DesktopApp, fachliche
// Umsetzung folgt im Blockchain-Sprint. Ehrlich statt Build-Brecher.
export function BlockchainLedgerView({ language = 'DE' }: { language?: string }) {
  const t = {
    DE: { title: 'Blockchain-Ledger', sub: 'NICHT IMPLEMENTIERT — Fachliche Umsetzung folgt (siehe REALITY_STATUS.md).' },
    EN: { title: 'Blockchain Ledger', sub: 'NOT IMPLEMENTED — implementation pending (see REALITY_STATUS.md).' },
  }[language] ?? { title: 'Blockchain Ledger', sub: 'NOT IMPLEMENTED' };
  return (
    <div className="flex flex-col items-center justify-center h-full text-slate-400 gap-3 p-8">
      <Layers className="w-10 h-10" />
      <h2 className="text-lg font-semibold text-slate-200">{t.title}</h2>
      <div className="flex items-center gap-2 text-sm">
        <Database className="w-4 h-4" />
        <span>{t.sub}</span>
      </div>
    </div>
  );
}
