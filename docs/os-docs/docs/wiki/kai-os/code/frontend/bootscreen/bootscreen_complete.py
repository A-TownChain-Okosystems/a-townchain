# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import sys, math, random, os, time
from PyQt6 import QtWidgets, QtCore, QtGui
from PyQt6.QtOpenGLWidgets import QOpenGLWidget
from OpenGL.GL import *
from OpenGL.GLU import *
from OpenGL.GL.shaders import compileProgram, compileShader
from PIL import Image
import numpy as np

BASE_PATH     = os.path.dirname(os.path.abspath(__file__))
SHADERS_PATH  = os.path.join(BASE_PATH, "shaders")
TEXTURES_PATH = os.path.join(BASE_PATH, "textures")

# ── WATERMARK ─────────────────────────────────────────
def close_source_notice():
    print("\n=== GlobusOS & A-TownChain Ecosystem – PROPRIETARY CODE ===\n")
close_source_notice()

# ── SHADER LOADER ─────────────────────────────────────
def load_shader(vertex_file, fragment_file):
    vp = os.path.join(SHADERS_PATH, vertex_file)
    fp = os.path.join(SHADERS_PATH, fragment_file)
    return compileProgram(
        compileShader(open(vp).read(), GL_VERTEX_SHADER),
        compileShader(open(fp).read(), GL_FRAGMENT_SHADER)
    )

# ── GLOBE LOADER ──────────────────────────────────────
class GlobeLoader(QOpenGLWidget):
    progress_updated = QtCore.pyqtSignal(float)
    load_complete    = QtCore.pyqtSignal()

    def __init__(self, quality="High"):
        super().__init__()
        self.progress      = 0.0
        self.rotation      = 0.0
        self.time          = 0.0
        self.cam_angle     = 0.0
        self.cam_distance  = 7.0
        self.quality       = quality
        self.set_quality_parameters()

        # Kosmische Objekte
        self.comets    = [(random.uniform(-5,5), random.uniform(-5,5),
                           random.uniform(-5,5), random.uniform(0.01,0.05))
                          for _ in range(20)]
        self.asteroids = [(random.uniform(-8,8), random.uniform(-8,8),
                           random.uniform(-8,8), random.uniform(0.01,0.03))
                          for _ in range(50)]
        self.planets   = [(random.uniform(-6,6), random.uniform(-6,6),
                           random.uniform(-6,6), random.uniform(0.2,1.0))
                          for _ in range(3)]
        self.supernova = None
        self._initialized = False

        # Timer
        self.timer = QtCore.QTimer(self)
        self.timer.timeout.connect(self._tick)
        self.timer.start(16)  # ~60fps

        self.load_timer = QtCore.QTimer(self)
        self.load_timer.timeout.connect(self._simulate_loading)
        self.load_timer.start(50)

    def set_quality_parameters(self):
        q = self.quality
        self.fbo_size    = 720  if q=="Low" else 1080
        self.cloud_blend = 0.2  if q=="Low" else (0.5 if q=="Medium" else 0.7)
        self.bloom_passes= 4    if q=="Low" else (6   if q=="Medium" else 10)
        self.star_count  = 100  if q=="Low" else (300 if q=="Medium" else 500)

    def _tick(self):
        self.rotation   += 0.3
        self.time       += 0.016
        self.cam_angle  += 0.1
        self.update()

    def _simulate_loading(self):
        if self.progress < 1.0:
            self.progress = min(1.0, self.progress + random.uniform(0.005, 0.015))
            self.progress_updated.emit(self.progress)
        else:
            self.load_timer.stop()
            QtCore.QTimer.singleShot(800, self.load_complete.emit)

    # ── OpenGL ──────────────────────────────────────────
    def initializeGL(self):
        glEnable(GL_DEPTH_TEST)
        glEnable(GL_BLEND)
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA)
        glClearColor(0.0, 0.0, 0.02, 1.0)

        # Texturen laden (Fallback: einfarbig wenn fehlt)
        self.earth_tex   = self._load_tex("earth_surface.jpg",   (0.1,0.3,0.7))
        self.clouds_tex  = self._load_tex("clouds.png",           (0.9,0.9,0.9))
        self.city_tex    = self._load_tex("city_lights.png",      (1.0,0.9,0.0))
        self.moon_tex    = self._load_tex("moon_texture.jpg",     (0.6,0.6,0.6))
        self.stars_tex   = self._load_tex("stars.png",            (0.05,0.05,0.15))

        self._initialized = True

    def _load_tex(self, fname, fallback_color=(1,1,1)):
        path = os.path.join(TEXTURES_PATH, fname)
        tid  = glGenTextures(1)
        glBindTexture(GL_TEXTURE_2D, tid)
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR)
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR)
        try:
            img  = Image.open(path).convert("RGBA")
            data = img.tobytes("raw","RGBA",0,-1)
            glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA,
                         img.width, img.height, 0, GL_RGBA, GL_UNSIGNED_BYTE, data)
        except Exception:
            # Fallback: 1×1 Pixel in fallback_color
            r,g,b = [int(c*255) for c in fallback_color]
            glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, 1, 1, 0,
                         GL_RGBA, GL_UNSIGNED_BYTE, bytes([r,g,b,255]))
        return tid

    def paintGL(self):
        if not self._initialized:
            return
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)
        w, h = self.width(), self.height()
        glViewport(0, 0, w, h)

        glMatrixMode(GL_PROJECTION)
        glLoadIdentity()
        gluPerspective(45.0, w/max(h,1), 0.1, 100.0)

        glMatrixMode(GL_MODELVIEW)
        glLoadIdentity()

        # Kamera kreist langsam
        cx = math.sin(math.radians(self.cam_angle)) * self.cam_distance
        cz = math.cos(math.radians(self.cam_angle)) * self.cam_distance
        gluLookAt(cx, 1.5, cz,  0.0, 0.0, 0.0,  0.0, 1.0, 0.0)

        # Sterne (Hintergrund)
        self._draw_stars()

        # Erde
        self._draw_earth()

        # Mond
        self._draw_moon()

        # Kosmische Objekte
        self._draw_cosmic_objects()

        # Nebel-Effekt (einfach)
        self._draw_nebula()

    def _draw_stars(self):
        glDisable(GL_DEPTH_TEST)
        glPointSize(1.5)
        random.seed(42)
        glBegin(GL_POINTS)
        for _ in range(self.star_count):
            x = random.uniform(-15, 15)
            y = random.uniform(-15, 15)
            z = random.uniform(-15, -5)
            b = random.uniform(0.4, 1.0)
            glColor3f(b, b, b*1.1)
            glVertex3f(x, y, z)
        glEnd()
        glEnable(GL_DEPTH_TEST)

    def _draw_earth(self):
        glPushMatrix()
        glRotatef(self.rotation, 0.0, 1.0, 0.0)
        glRotatef(-23.5, 0.0, 0.0, 1.0)  # Achsneigung

        # Atmosphären-Glow
        glDisable(GL_DEPTH_TEST)
        glBlendFunc(GL_SRC_ALPHA, GL_ONE)
        glow_steps = 8
        for i in range(glow_steps):
            alpha = 0.03 * (1.0 - i/glow_steps)
            scale = 1.02 + i * 0.01
            glColor4f(0.2, 0.5, 1.0, alpha)
            q = gluNewQuadric()
            gluSphere(q, scale, 32, 32)
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA)
        glEnable(GL_DEPTH_TEST)

        # Erde-Textur
        glEnable(GL_TEXTURE_2D)
        glBindTexture(GL_TEXTURE_2D, self.earth_tex)
        glColor3f(1.0, 1.0, 1.0)
        q = gluNewQuadric()
        gluQuadricTexture(q, GL_TRUE)
        gluSphere(q, 1.0, 48, 48)

        # Wolken (leicht versetzt rotiert)
        glRotatef(self.rotation * 0.3, 0.0, 1.0, 0.0)
        glBindTexture(GL_TEXTURE_2D, self.clouds_tex)
        glColor4f(1.0, 1.0, 1.0, self.cloud_blend)
        q2 = gluNewQuadric()
        gluQuadricTexture(q2, GL_TRUE)
        gluSphere(q2, 1.01, 48, 48)
        glDisable(GL_TEXTURE_2D)

        glPopMatrix()

    def _draw_moon(self):
        glPushMatrix()
        angle = math.radians(self.time * 20)
        mx = math.cos(angle) * 2.5
        my = math.sin(angle) * 0.3
        mz = math.sin(angle) * 2.5
        glTranslatef(mx, my, mz)
        glRotatef(self.rotation * 0.5, 0.0, 1.0, 0.0)

        glEnable(GL_TEXTURE_2D)
        glBindTexture(GL_TEXTURE_2D, self.moon_tex)
        glColor3f(1.0, 1.0, 1.0)
        q = gluNewQuadric()
        gluQuadricTexture(q, GL_TRUE)
        gluSphere(q, 0.27, 24, 24)
        glDisable(GL_TEXTURE_2D)
        glPopMatrix()

    def _draw_cosmic_objects(self):
        t = self.time
        glPointSize(2)
        glBegin(GL_POINTS)
        for i, (x,y,z,speed) in enumerate(self.comets):
            nx = x + math.sin(t * speed * 10 + i) * 0.5
            ny = y + math.cos(t * speed * 8  + i) * 0.3
            glColor4f(1.0, 0.6, 0.2, 0.8)
            glVertex3f(nx, ny, z)
        for i, (x,y,z,speed) in enumerate(self.asteroids):
            nx = x + math.sin(t * speed * 5 + i*0.3) * 0.2
            ny = y + math.cos(t * speed * 4 + i*0.2) * 0.2
            glColor4f(0.5, 0.5, 0.5, 0.6)
            glVertex3f(nx, ny, z)
        glEnd()

        # Planeten
        for i, (x,y,z,size) in enumerate(self.planets):
            glPushMatrix()
            angle = t * (0.1 + i*0.05)
            glTranslatef(
                x + math.cos(angle)*0.3,
                y,
                z + math.sin(angle)*0.3
            )
            colors_list = [(0.8,0.4,0.2),(0.6,0.8,0.6),(0.7,0.5,0.9)]
            r,g,b = colors_list[i % 3]
            glColor3f(r,g,b)
            q = gluNewQuadric()
            gluSphere(q, size*0.3, 16, 16)
            glPopMatrix()

    def _draw_nebula(self):
        glDisable(GL_DEPTH_TEST)
        glBlendFunc(GL_SRC_ALPHA, GL_ONE)
        t = self.time
        for i in range(5):
            x = math.sin(t*0.2 + i*1.2) * 6
            y = math.cos(t*0.15 + i*0.8) * 4
            r,g,b = [(0.5,0.1,0.8),(0.1,0.3,0.9),(0.8,0.1,0.5),
                     (0.1,0.7,0.5),(0.9,0.4,0.1)][i]
            glColor4f(r,g,b,0.015)
            glBegin(GL_TRIANGLE_FAN)
            glVertex3f(x, y, -8)
            for a in range(37):
                ax = math.cos(math.radians(a*10)) * 3
                ay = math.sin(math.radians(a*10)) * 3
                glVertex3f(x+ax, y+ay, -8)
            glEnd()
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA)
        glEnable(GL_DEPTH_TEST)

    def resizeGL(self, w, h):
        glViewport(0, 0, w, h)


# ── LAUNCHER WINDOW ───────────────────────────────────
class LauncherWindow(QtWidgets.QWidget):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("A-TownChain OS — Boot Screen")
        self.setFixedSize(900, 620)
        self.setStyleSheet("background: #05080f;")

        layout = QtWidgets.QVBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.setSpacing(0)

        # ── Globe ──
        self.globe = GlobeLoader(quality="High")
        self.globe.setMinimumHeight(460)
        layout.addWidget(self.globe)

        # ── Bottom Panel ──
        panel = QtWidgets.QWidget()
        panel.setFixedHeight(160)
        panel.setStyleSheet("background: rgba(5,8,15,0.98);")
        panel_layout = QtWidgets.QVBoxLayout(panel)
        panel_layout.setContentsMargins(40, 12, 40, 20)
        panel_layout.setSpacing(6)

        # Logo
        logo = QtWidgets.QLabel("🌌  A-TownChain OS")
        logo.setStyleSheet("""
            font-family: 'Orbitron', 'Courier New', monospace;
            font-size: 22px; font-weight: 900;
            color: qlineargradient(x1:0,y1:0,x2:1,y2:0,
                stop:0 #a259ff, stop:1 #00d1ff);
            letter-spacing: 4px;
        """)
        logo.setAlignment(QtCore.Qt.AlignmentFlag.AlignCenter)
        panel_layout.addWidget(logo)

        # Status Label
        self.status_label = QtWidgets.QLabel("Initialisiere A-TownChain Ökosystem...")
        self.status_label.setStyleSheet("""
            font-family: 'Courier New', monospace;
            font-size: 11px; color: #00d1ff;
            letter-spacing: 2px;
        """)
        self.status_label.setAlignment(QtCore.Qt.AlignmentFlag.AlignCenter)
        panel_layout.addWidget(self.status_label)

        # Progress Bar
        self.progress_bar = QtWidgets.QProgressBar()
        self.progress_bar.setRange(0, 1000)
        self.progress_bar.setValue(0)
        self.progress_bar.setTextVisible(False)
        self.progress_bar.setFixedHeight(8)
        self.progress_bar.setStyleSheet("""
            QProgressBar {
                background: rgba(162,89,255,0.1);
                border: 1px solid rgba(162,89,255,0.3);
                border-radius: 4px;
            }
            QProgressBar::chunk {
                background: qlineargradient(x1:0,y1:0,x2:1,y2:0,
                    stop:0 #a259ff, stop:0.5 #00d1ff, stop:1 #ff2d78);
                border-radius: 3px;
            }
        """)
        panel_layout.addWidget(self.progress_bar)

        # Percent + Version
        bottom_row = QtWidgets.QHBoxLayout()
        self.pct_label = QtWidgets.QLabel("0%")
        self.pct_label.setStyleSheet("font-family:'Courier New'; font-size:10px; color:#a259ff;")
        ver_label = QtWidgets.QLabel("v2.0.0 Genesis Release  ·  GlobusOS & A-TownChain Ecosystem")
        ver_label.setStyleSheet("font-family:'Courier New'; font-size:9px; color:#2a3a5a; letter-spacing:1px;")
        ver_label.setAlignment(QtCore.Qt.AlignmentFlag.AlignRight)
        bottom_row.addWidget(self.pct_label)
        bottom_row.addWidget(ver_label)
        panel_layout.addLayout(bottom_row)

        layout.addWidget(panel)

        # Signals
        self.globe.progress_updated.connect(self._on_progress)
        self.globe.load_complete.connect(self._on_complete)

        # Status Messages
        self._status_msgs = [
            "Initialisiere A-TownChain Ökosystem...",
            "Lade ATC-8300 Token Standard...",
            "Verbinde mit Blockchain Node...",
            "Starte API Gateway :4000...",
            "Lade Shivamon NFT Contract (ATC-9000)...",
            "Initialisiere Hybrid Consensus (PoW+PoS+PoH)...",
            "Verbinde Wallet System...",
            "Starte AI Orchestrator...",
            "Synchronisiere Chain...",
            "A-TownChain OS bereit. ✅",
        ]

    def _on_progress(self, pct: float):
        self.progress_bar.setValue(int(pct * 1000))
        self.pct_label.setText(f"{int(pct*100)}%")
        idx = min(int(pct * (len(self._status_msgs)-1)), len(self._status_msgs)-1)
        self.status_label.setText(self._status_msgs[idx])

    def _on_complete(self):
        self.status_label.setText("✅  A-TownChain OS vollständig geladen — Starte Dashboard...")
        self.status_label.setStyleSheet("""
            font-family: 'Courier New', monospace;
            font-size: 11px; color: #00c896; letter-spacing: 2px;
        """)
        QtCore.QTimer.singleShot(1500, self._launch_dashboard)

    def _launch_dashboard(self):
        import webbrowser
        webbrowser.open("http://localhost:3000")
        # Optional: Bootscreen schließen
        # self.close()


# ── MAIN ──────────────────────────────────────────────
def main():
    app = QtWidgets.QApplication(sys.argv)
    app.setApplicationName("A-TownChain OS")

    # OpenGL Surface Format
    fmt = QtGui.QSurfaceFormat()
    fmt.setDepthBufferSize(24)
    fmt.setSamples(4)  # MSAA
    fmt.setVersion(2, 1)
    QtGui.QSurfaceFormat.setDefaultFormat(fmt)

    window = LauncherWindow()
    window.show()
    sys.exit(app.exec())

if __name__ == "__main__":
    main()
