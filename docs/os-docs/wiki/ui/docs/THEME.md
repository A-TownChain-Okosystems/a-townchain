# ATC Dashboard — Neon Dark Theme

## CSS Custom Properties
```css
:root {
  --primary:       #a259ff;
  --secondary:     #00d1ff;
  --accent:        #00ffcc;
  --bg-dark:       #0a0a1a;
  --bg-card:       #111128;
  --bg-elevated:   #1a1a35;
  --text:          #e0e0ff;
  --text-muted:    #8888aa;
  --neon-green:    #00ff88;
  --neon-red:      #ff4466;
  --neon-yellow:   #ffcc00;
  --border:        #2a2a4a;
  --border-neon:   rgba(162, 89, 255, 0.5);
}
```

## Neon-Glow-Effekte
```css
/* Box-Glow */
.glow-purple { box-shadow: 0 0 15px #a259ff55, 0 0 30px #a259ff22; }
.glow-cyan   { box-shadow: 0 0 15px #00d1ff55, 0 0 30px #00d1ff22; }
.glow-green  { box-shadow: 0 0 15px #00ffcc55; }

/* Text-Glow */
.text-glow   { text-shadow: 0 0 8px var(--primary); }

/* Animiertes Pulse-Glow */
@keyframes pulse-glow {
  0%, 100% { box-shadow: 0 0 10px #a259ff66; }
  50%       { box-shadow: 0 0 25px #a259ffaa, 0 0 50px #a259ff44; }
}
.pulse { animation: pulse-glow 2s ease-in-out infinite; }
```

## Button-Styles
```css
.btn-primary {
  background: linear-gradient(135deg, #a259ff, #6a35cc);
  border: 1px solid #a259ff;
  color: #fff;
  padding: 8px 20px;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
}
.btn-primary:hover { box-shadow: 0 0 15px #a259ff88; transform: translateY(-1px); }

.btn-outline {
  background: transparent;
  border: 1px solid var(--primary);
  color: var(--primary);
}
```

## Rarity-Farben (Shivamon)
```css
.rarity-common    { color: #aaaaaa; }
.rarity-uncommon  { color: #00ff88; }
.rarity-rare      { color: #00d1ff; }
.rarity-epic      { color: #a259ff; }
.rarity-legendary { color: #ffcc00; text-shadow: 0 0 10px #ffcc0088; }
```
