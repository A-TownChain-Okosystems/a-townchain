# 📄 Issue #3 — Shivamon Battle UI

> **Labels:** enhancement · frontend · priority:high
> **Priorität:** 🔴 High · **Milestone:** v2.1.0
> **Referenz:** [GitHub Issue #3](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/3)

---

## Ziel

Visuelles, animiertes Battle-System im ShivaOS Dashboard. Shivamon kämpfen Runde für Runde mit HP-Bars, Schadensanzeigen und XP-Gewinn — vollständig im Browser, verdrahtet mit dem bestehenden `ShivamonContract.battle()`.

---

## UI-Layout

```
┌──────────────────────────────────────────────────┐
│  ⚔️  Shivamon BATTLE ARENA                        │
├──────────────────┬────────┬─────────────────────┤
│  ATTACKER        │   VS   │  DEFENDER            │
│  🔥 Ignarex-001  │        │  💧 Aquarix-007      │
│  HP ████████░░   │        │  HP ██████░░░░       │
│  Lv.3 · Rare     │        │  Lv.5 · Epic         │
├──────────────────┴────────┴─────────────────────┤
│  BATTLE LOG                                      │
│  > Runde 1: Ignarex trifft für 52 Schaden!      │
│  > Runde 1: Aquarix antwortet mit 38 Schaden!   │
│  > Runde 2: Ignarex trifft für 47 Schaden!      │
│  > 🏆 IGNAREX GEWINNT! +60 XP · Level Up! ⬆️   │
├─────────────────────────────────────────────────┤
│  [▶ KAMPF STARTEN]  [🔄 NEUE AUSWAHL]           │
└─────────────────────────────────────────────────┘
```

---

## Technische Spezifikation

### HTML-Struktur

```html
<div id="page-battle" class="page-content">

  <!-- Auswahl-Phase -->
  <div id="battle-select">
    <div class="battle-slot" id="slot-attacker">
      <!-- Dropdown: eigene Shivamon -->
    </div>
    <div class="vs-divider">⚔️ VS</div>
    <div class="battle-slot" id="slot-defender">
      <!-- Input: Gegner Token-ID oder eigene Collection -->
    </div>
  </div>

  <!-- Kampf-Arena -->
  <div id="battle-arena" style="display:none">
    <div class="fighter" id="fighter-a">...</div>
    <div class="fighter" id="fighter-d">...</div>
  </div>

  <!-- Battle Log -->
  <div id="battle-log"></div>

  <!-- Ergebnis -->
  <div id="battle-result" style="display:none">...</div>

</div>
```

### Animierter Kampfablauf (JavaScript)

```javascript
async function runBattle(attackerId, defenderId) {
  const result = await ATC_API.battleShivamon(attackerId, defenderId);
  const rounds = result.rounds;

  // Runden animiert durchspielen
  for (const round of rounds) {
    await sleep(800);
    appendLog(`Runde ${round.round}: ${round.attacker} trifft für ${round.damage} Schaden!`);
    updateHPBar(round.attacker === attackerName ? "defender" : "attacker", round.defender_hp);
    playHitAnimation(round.attacker === attackerName ? "attacker" : "defender");
  }

  await sleep(1000);
  showResult(result.winner, result.xp_gained);
}

function updateHPBar(side, currentHp) {
  const bar  = document.getElementById(`hp-bar-${side}`);
  const pct  = Math.max(0, (currentHp / maxHp[side]) * 100);
  bar.style.width    = pct + "%";
  bar.style.background = pct > 50 ? "var(--green)" : pct > 25 ? "var(--orange)" : "var(--pink)";
}
```

### CSS — Kampf-Animationen

```css
@keyframes hit-shake {
  0%,100% { transform: translateX(0); }
  25%      { transform: translateX(-8px); }
  75%      { transform: translateX(8px); }
}
@keyframes damage-float {
  0%   { opacity: 1; transform: translateY(0); }
  100% { opacity: 0; transform: translateY(-40px); }
}
.fighter.hit { animation: hit-shake 0.3s ease; }
.damage-number {
  position: absolute; font-family: 'Orbitron', sans-serif;
  font-size: 20px; color: var(--pink); font-weight: 900;
  animation: damage-float 1s ease forwards;
}
```

---

## Aufgaben

- [ ] Battle-Seite in `frontend/index.html` — Sidebar `⚔️ Battle`
- [ ] Shivamon-Auswahl aus eigener Collection (Dropdown)
- [ ] Gegner-Eingabe (Token-ID oder eigene Collection)
- [ ] Animierter Kampfablauf Runde für Runde (800ms Delay)
- [ ] HP-Bar Echtzeit-Update mit Farbwechsel (Grün → Orange → Rot)
- [ ] Floating Damage Numbers
- [ ] Level-Up Animation nach XP-Gewinn
- [ ] Battle-Log als scrollbares Protokoll
- [ ] Battle-History Tabelle (letzte 20 Kämpfe via API)
- [ ] API-Call `POST /api/game/shivamon/battle` verdrahten

---

## Akzeptanzkriterien

- [ ] Kampf läuft animiert Runde für Runde ab
- [ ] HP-Bars aktualisieren sich korrekt
- [ ] Gewinner und XP werden angezeigt
- [ ] Level-Up wird visuell kommuniziert
- [ ] Funktioniert ohne Backend (Mock-Modus)
