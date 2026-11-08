"""
Genesis Engine — MVP Milestone 1 Demo
Erzeugt 5 bewegte, farbige Entities und rendert sie via Renderer2D.

Ausfuehrung mit Display: python main.py
Ausfuehrung headless (Test/CI, kein SDL-Fenster): python main.py --headless
"""
import sys
import time
from core.ecs import World, Position, Velocity, Sprite, MovementSystem
from render.renderer2d import Renderer2D


def build_demo_world() -> World:
    world = World()
    world.add_system(MovementSystem())

    colors = [(255, 80, 80), (80, 255, 120), (80, 160, 255), (255, 220, 80), (200, 80, 255)]
    for i in range(5):
        eid = world.create_entity()
        world.add_component(eid, Position(x=50 + i * 100, y=50 + i * 60))
        world.add_component(eid, Velocity(dx=20 + i * 5, dy=10))
        world.add_component(eid, Sprite(color=colors[i], width=32, height=32))
    return world


def run(headless: bool = False, frames: int = 60):
    world = build_demo_world()
    renderer = Renderer2D(world, headless=headless)

    dt = 1 / 60
    for frame in range(frames):
        world.update(dt)
        drawn = renderer.render_frame()
        if headless and frame == frames - 1:
            print(f"[headless] Frame {frame}: {len(drawn)} Entities gerendert")
            for x, y, w, h, color in drawn:
                print(f"  Entity @ ({x:.1f}, {y:.1f}) size={w}x{h} color={color}")
        if not headless:
            time.sleep(dt)

    renderer.close()
    return world


if __name__ == "__main__":
    headless = "--headless" in sys.argv
    run(headless=headless)
