# ATC Dashboard — Design System

## Farb-Palette (Neon Dark Theme)
| Variable | Hex | Verwendung |
|----------|-----|-----------|
| `--primary` | #a259ff | Haupt-Akzent, Buttons |
| `--secondary` | #00d1ff | Sekundär-Akzent |
| `--accent` | #00ffcc | Highlights, Erfolg |
| `--bg-dark` | #0a0a1a | Haupt-Hintergrund |
| `--bg-card` | #111128 | Card-Hintergrund |
| `--text` | #e0e0ff | Fließtext |
| `--neon-green` | #00ff88 | Erfolgs-Farbe |
| `--neon-red` | #ff4466 | Fehler-Farbe |
| `--border` | #2a2a4a | Rahmen |

## Typografie
- Headings: `Orbitron` (Google Fonts), monospace
- Body: `Space Grotesk`, sans-serif
- Code: `JetBrains Mono`, monospace

## Neon-Effekte (CSS)
```css
.neon-box  { box-shadow: 0 0 20px var(--primary), 0 0 40px rgba(162,89,255,.3); }
.neon-text { text-shadow: 0 0 10px var(--primary); }
.neon-border { border: 1px solid var(--primary); box-shadow: 0 0 8px var(--primary); }
```

## Komponenten
- WalletPanel — Balance, Send-Dialog, QR-Code
- ChainExplorer — Block-Liste live, TX-Suche
- ShivamonGallery — NFT-Grid, Rarity-Filter
- GeminiChat — BYOK, ATCLang-Generierung
- GatewayStatus — Health, Latenz, Rate-Limit
