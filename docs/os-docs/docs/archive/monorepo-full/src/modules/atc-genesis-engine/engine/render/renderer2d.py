# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
Genesis Engine — Minimaler 2D-Renderer (pygame-basiert)
MVP Milestone 1 — ECHTER CODE.

Rendert alle Entities mit Position + Sprite als farbige Rechtecke.
Bewusst simpel gehalten: kein Asset-Pipeline, keine Shader — nur der
Beweis, dass ECS World -> sichtbares Bild funktioniert.
"""
from core.ecs import World, Position, Sprite


class Renderer2D:
    def __init__(self, world: World, width: int = 800, height: int = 600, headless: bool = False):
        self.world = world
        self.width = width
        self.height = height
        self.headless = headless
        self._pygame = None
        self._screen = None

        if not headless:
            import pygame
            self._pygame = pygame
            pygame.init()
            self._screen = pygame.display.set_mode((width, height))
            pygame.display.set_caption("Genesis Engine — MVP Milestone 1")

    def render_frame(self) -> list:
        """Zeichnet einen Frame. Gibt eine Liste der gezeichneten
        (x, y, w, h, color)-Tupel zurueck — nuetzlich fuer Tests ohne Display."""
        drawn = []
        for eid, (pos, sprite) in self.world.query(Position, Sprite):
            drawn.append((pos.x, pos.y, sprite.width, sprite.height, sprite.color))

        if not self.headless and self._screen is not None:
            self._screen.fill((10, 10, 20))
            for x, y, w, h, color in drawn:
                self._pygame.draw.rect(self._screen, color, (x, y, w, h))
            self._pygame.display.flip()

        return drawn

    def close(self):
        if self._pygame is not None:
            self._pygame.quit()
