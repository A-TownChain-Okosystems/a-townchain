"""Unit-Tests fuer den ECS-Kern. Kein pygame noetig — reine Logik."""
import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

from core.ecs import World, Position, Velocity, Sprite, MovementSystem


def test_create_and_destroy_entity():
    world = World()
    eid = world.create_entity()
    assert eid in world.entities
    world.destroy_entity(eid)
    assert eid not in world.entities


def test_add_and_query_components():
    world = World()
    eid = world.create_entity()
    world.add_component(eid, Position(1, 2))
    world.add_component(eid, Velocity(3, 4))

    results = list(world.query(Position, Velocity))
    assert len(results) == 1
    got_eid, (pos, vel) = results[0]
    assert got_eid == eid
    assert pos.x == 1 and pos.y == 2
    assert vel.dx == 3 and vel.dy == 4


def test_movement_system_moves_entity():
    world = World()
    world.add_system(MovementSystem())
    eid = world.create_entity()
    world.add_component(eid, Position(0, 0))
    world.add_component(eid, Velocity(10, 5))

    world.update(dt=1.0)

    pos = world.get_component(eid, Position)
    assert pos.x == 10
    assert pos.y == 5


def test_query_excludes_entities_missing_components():
    world = World()
    eid_full = world.create_entity()
    world.add_component(eid_full, Position(0, 0))
    world.add_component(eid_full, Velocity(1, 1))

    eid_partial = world.create_entity()
    world.add_component(eid_partial, Position(5, 5))  # keine Velocity

    results = list(world.query(Position, Velocity))
    assert len(results) == 1
    assert results[0][0] == eid_full


if __name__ == "__main__":
    test_create_and_destroy_entity()
    test_add_and_query_components()
    test_movement_system_moves_entity()
    test_query_excludes_entities_missing_components()
    print("Alle 4 Tests bestanden - OK")
