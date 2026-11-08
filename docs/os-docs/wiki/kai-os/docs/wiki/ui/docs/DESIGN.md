# ATC Design System

## Farb-Palette
| Variable | Hex | Verwendung |
|----------|-----|-----------|
| `--primary` | #a259ff | Haupt-Akzent, Buttons |
| `--secondary` | #00d1ff | Sekundär-Akzent |
| `--accent` | #00ffcc | Highlights |
| `--bg-dark` | #0a0a1a | Haupt-Hintergrund |
| `--bg-card` | #111128 | Card-Hintergrund |
| `--text` | #e0e0ff | Fließtext |
| `--neon-green` | #00ff88 | Erfolgs-Farbe |
| `--neon-red` | #ff4466 | Fehler-Farbe |

## Typography
- Heading: `Orbitron`, monospace
- Body: `Space Grotesk`, sans-serif
- Code: `JetBrains Mono`, monospace

## Neon-Effekte
```css
.neon-box { box-shadow: 0 0 20px var(--primary), 0 0 40px rgba(162,89,255,0.3); }
.neon-text { text-shadow: 0 0 10px var(--primary); }
```
