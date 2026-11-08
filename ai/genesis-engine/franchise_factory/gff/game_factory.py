"""Game Factory dependency graph and fail-closed orchestration contract."""

from __future__ import annotations

# ruff: isort: skip_file

from dataclasses import dataclass

from gff.artifacts import ArtifactKind, ArtifactRef

@dataclass(frozen=True, slots=True)
class GameFactoryNode:
    id: str
    produces: ArtifactKind
    requires: tuple[ArtifactKind, ...] = ()

GAME_FACTORY_GRAPH: tuple[GameFactoryNode, ...] = (
    GameFactoryNode("concept", ArtifactKind.GAME_BIBLE),
    GameFactoryNode("world", ArtifactKind.WORLD_BIBLE, (ArtifactKind.GAME_BIBLE,)),
    GameFactoryNode("lore", ArtifactKind.LORE, (ArtifactKind.WORLD_BIBLE,)),
    GameFactoryNode("character", ArtifactKind.CHARACTER, (ArtifactKind.GAME_BIBLE, ArtifactKind.LORE)),
    GameFactoryNode("creature", ArtifactKind.CREATURE, (ArtifactKind.WORLD_BIBLE, ArtifactKind.LORE)),
    GameFactoryNode("combat", ArtifactKind.COMBAT, (ArtifactKind.GAME_BIBLE, ArtifactKind.CHARACTER, ArtifactKind.CREATURE)),
    GameFactoryNode("quest", ArtifactKind.QUEST, (ArtifactKind.WORLD_BIBLE, ArtifactKind.LORE, ArtifactKind.CHARACTER)),
    GameFactoryNode("level", ArtifactKind.LEVEL, (ArtifactKind.WORLD_BIBLE, ArtifactKind.QUEST, ArtifactKind.CREATURE)),
    GameFactoryNode("item", ArtifactKind.ITEM, (ArtifactKind.GAME_BIBLE,)),
    GameFactoryNode("weapon", ArtifactKind.WEAPON, (ArtifactKind.CHARACTER, ArtifactKind.COMBAT)),
    GameFactoryNode("animation", ArtifactKind.ANIMATION, (ArtifactKind.CHARACTER, ArtifactKind.CREATURE, ArtifactKind.WEAPON)),
    GameFactoryNode("audio", ArtifactKind.AUDIO, (ArtifactKind.GAME_BIBLE, ArtifactKind.WEAPON)),
    GameFactoryNode("vfx", ArtifactKind.VFX, (ArtifactKind.COMBAT, ArtifactKind.WEAPON)),
    GameFactoryNode("ai-npc", ArtifactKind.NPC_AI, (ArtifactKind.CHARACTER, ArtifactKind.LORE)),
    GameFactoryNode("economy", ArtifactKind.ECONOMY, (ArtifactKind.GAME_BIBLE, ArtifactKind.ITEM)),
    GameFactoryNode("multiplayer", ArtifactKind.MULTIPLAYER, (ArtifactKind.GAME_BIBLE, ArtifactKind.COMBAT, ArtifactKind.ECONOMY)),
    GameFactoryNode("build", ArtifactKind.BUILD, (ArtifactKind.GAME_BIBLE, ArtifactKind.WORLD_BIBLE, ArtifactKind.CHARACTER, ArtifactKind.QUEST, ArtifactKind.LEVEL, ArtifactKind.ITEM, ArtifactKind.WEAPON, ArtifactKind.ANIMATION, ArtifactKind.AUDIO, ArtifactKind.VFX, ArtifactKind.NPC_AI, ArtifactKind.MULTIPLAYER)),
    GameFactoryNode("testing", ArtifactKind.QA_REPORT, (ArtifactKind.BUILD, ArtifactKind.COMBAT, ArtifactKind.ECONOMY)),
    GameFactoryNode("liveops", ArtifactKind.LIVEOPS_PLAN, (ArtifactKind.QA_REPORT, ArtifactKind.MULTIPLAYER, ArtifactKind.ECONOMY)),
)

class GameFactoryGraphError(ValueError):
    pass

def validate_game_factory_graph(graph: tuple[GameFactoryNode, ...] = GAME_FACTORY_GRAPH) -> None:
    produced = {node.produces for node in graph}
    if len(produced) != len(graph):
        raise GameFactoryGraphError("Each Game Factory node must produce a unique artifact kind")
    ids = {node.id for node in graph}
    if len(ids) != len(graph):
        raise GameFactoryGraphError("Game Factory node ids must be unique")
    for node in graph:
        for requirement in node.requires:
            if requirement not in produced:
                raise GameFactoryGraphError(f"Node {node.id} requires {requirement}, but no producer exists")

def topological_order(graph: tuple[GameFactoryNode, ...] = GAME_FACTORY_GRAPH) -> tuple[str, ...]:
    validate_game_factory_graph(graph)
    remaining = {node.produces: node for node in graph}
    done: set[ArtifactKind] = set()
    order: list[str] = []
    while remaining:
        ready = [node for node in remaining.values() if all(req in done for req in node.requires)]
        if not ready:
            raise GameFactoryGraphError("Game Factory dependency cycle detected")
        ready.sort(key=lambda node: node.id)
        for node in ready:
            order.append(node.id)
            done.add(node.produces)
            del remaining[node.produces]
    return tuple(order)

def dependency_refs(node: GameFactoryNode, artifacts: tuple[ArtifactRef, ...]) -> tuple[ArtifactRef, ...]:
    by_kind = {artifact.kind: artifact for artifact in artifacts}
    missing = [kind for kind in node.requires if kind not in by_kind]
    if missing:
        raise GameFactoryGraphError(f"Missing dependencies for {node.id}: {', '.join(missing)}")
    return tuple(by_kind[kind] for kind in node.requires)

validate_game_factory_graph()
