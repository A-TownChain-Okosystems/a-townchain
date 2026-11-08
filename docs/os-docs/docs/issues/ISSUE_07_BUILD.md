# 📄 Issue #7 — Build System (EXE / AppImage)

> **Labels:** enhancement · build · priority:medium
> **Priorität:** 🟡 Medium · **Milestone:** v2.2.0
> **Referenz:** [GitHub Issue #7](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/7)

---

## Ziel

A-TownChain OS als **standalone Desktop-Anwendung** verpacken — ein einziger Doppelklick startet Gateway, Backend und öffnet das ShivaOS Dashboard im Browser. Kein Python, keine Installation nötig.

---

## Ziel-Architektur

```
ATownChainOS.exe / ATownChainOS.AppImage
│
├── Embedded Python Runtime (via PyInstaller)
├── gateway/main.py      → startet automatisch auf Port 4000
├── backend/main.py      → startet automatisch auf Port 5000
├── frontend/index.html  → wird in Standard-Browser geöffnet
└── data/atcoin.db       → lokale SQLite-Datenbank
```

---

## Build-Konfiguration

### PyInstaller Spec

```python
# build/atownchain.spec
block_cipher = None

a = Analysis(
    ['launcher.py'],
    pathex=['.'],
    binaries=[],
    datas=[
        ('frontend/', 'frontend/'),
        ('config/', 'config/'),
        ('blockchain/wallet/wordlist.py', 'blockchain/wallet/'),
    ],
    hiddenimports=['flask', 'flask_cors', 'requests', 'cryptography'],
    hookspath=[],
)

pyz = PYZ(a.pure, a.zipped_data, cipher=block_cipher)

exe = EXE(
    pyz, a.scripts, a.binaries, a.zipfiles, a.datas,
    name='ATownChainOS',
    icon='assets/icon.ico',
    onefile=True,
    console=False,
)
```

### Launcher

```python
# launcher.py — Startet alle Services + öffnet Browser
import subprocess, threading, time, webbrowser, sys

def start_gateway():
    subprocess.Popen([sys.executable, "gateway/main.py"])

def start_backend():
    subprocess.Popen([sys.executable, "backend/main.py"])

def open_dashboard():
    time.sleep(3)  # Warten bis Services starten
    webbrowser.open("http://localhost:3000")

if __name__ == "__main__":
    threading.Thread(target=start_gateway, daemon=True).start()
    threading.Thread(target=start_backend, daemon=True).start()
    open_dashboard()
```

### GitHub Actions CI/CD

```yaml
# .github/workflows/build.yml
name: Build Release
on:
  push:
    tags: ['v*']
jobs:
  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
      - run: pip install pyinstaller
      - run: pyinstaller build/atownchain.spec
      - uses: actions/upload-artifact@v4
        with:
          name: ATownChainOS-Windows
          path: dist/ATownChainOS.exe

  build-linux:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: pip install pyinstaller
      - run: pyinstaller build/atownchain.spec
      - run: chmod +x dist/ATownChainOS
```

---

## Aufgaben

- [ ] `launcher.py` — Service-Starter implementieren
- [ ] `build/atownchain.spec` — PyInstaller Konfiguration
- [ ] `build/build.py` — Build-Script (lokaler Build)
- [ ] `assets/icon.ico` + `assets/icon.png` — App-Icon
- [ ] `.github/workflows/build.yml` — CI/CD Pipeline
- [ ] Windows EXE testen
- [ ] Linux AppImage testen
- [ ] Installer-Größe optimieren (< 100 MB Ziel)

---

## Akzeptanzkriterien

- [ ] Doppelklick auf EXE öffnet Dashboard im Browser
- [ ] Kein Python / pip nötig auf dem Zielrechner
- [ ] Windows 10+ und Ubuntu 20.04+ unterstützt
- [ ] GitHub Actions baut automatisch bei neuem Release-Tag
