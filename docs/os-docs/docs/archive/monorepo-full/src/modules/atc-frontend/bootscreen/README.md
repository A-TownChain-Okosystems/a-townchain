# 🌌 A-TownChain OS — Bootscreen

> PyQt6 + OpenGL · `frontend/bootscreen/bootscreen_complete.py`

## Features
- Rotierender 3D-Erdglobus mit Textur-Support (earth, clouds, city lights, moon)
- Dynamische Kamera-Orbit-Animation
- 20 Kometen · 50 Asteroiden · 3 Planeten · Nebel-Effekte
- Sternenfeld (100–500 Sterne je Qualitätsstufe)
- Atmosphären-Glow + Mond-Orbit-Animation
- Progress Bar mit 10 ATC-Status-Nachrichten
- 3 Qualitätsstufen: Low / Medium / High
- Auto-Launch Dashboard auf http://localhost:3000

## Start

```bash
pip install PyQt6 PyOpenGL PyOpenGL_accelerate Pillow numpy
python frontend/bootscreen/bootscreen_complete.py
```

## Verzeichnisstruktur

```
frontend/bootscreen/
├── bootscreen_complete.py
├── shaders/
│   ├── vertex_shader.glsl
│   ├── vertex_shader_quad.glsl
│   ├── frag_earth_aaa_bloom.glsl
│   └── frag_bloom.glsl
└── textures/
    ├── earth_surface.jpg    ← NASA Blue Marble empfohlen
    ├── clouds.png
    ├── city_lights.png
    ├── moon_texture.jpg
    └── stars.png
```

> Ohne Texturdateien läuft der Bootscreen mit Fallback-Farben — kein Absturz.

## Qualitätsstufen

| Setting | Stars | Bloom Passes | Cloud Blend |
|---------|-------|-------------|-------------|
| Low     | 100   | 4           | 0.2         |
| Medium  | 300   | 6           | 0.5         |
| High    | 500   | 10          | 0.7         |
