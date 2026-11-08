// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.

// NOT IMPLEMENTED — Platzhalter-Engine (SCR-0078): server.ts und P2PNetwork importieren
// AtcBlockchainEngine; die echte Engine lebt im atc-blockchain/a-townchain-Track.
// Diese Klasse liefert ein lauffaehiges, ehrliches Null-Backend (keine Fake-Blocks!).
export class AtcBlockchainEngine {
  private started = false;

  startConsensus(): void {
    this.started = true;
  }

  getChainHeight(): number {
    return 0; // Ehrlich: keine Chain implementiert (siehe REALITY_STATUS.md)
  }

  getLatestBlock(): { hash: string; height: number } {
    return { hash: "NOT-IMPLEMENTED", height: 0 };
  }

  getChain(): Array<Record<string, unknown>> {
    return [];
  }

  getTPS(): number {
    return 0;
  }
}
