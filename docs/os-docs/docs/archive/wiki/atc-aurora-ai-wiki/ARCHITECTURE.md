# ARCHITECTURE.md — atc-aurora-ai
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
├── .gitignore
├── CHANGELOG.md
├── COMPONENT_PLAN.md
├── FILE_REGISTER.md
├── LICENSE
├── README.md
├── ROADMAP.md
├── STATUS.md
├── agents/
│   ├── architect_agent.atc
│   ├── coding_agent.atc
│   ├── knowledge_agent.atc
│   ├── orchestrator_agent.atc
│   ├── research_agent.atc
│   └── security_agent.atc
├── core/
│   ├── agent_registry.atc
│   ├── aurora_core.atc
│   └── model_hub.atc
├── memory/
│   ├── agent_memory.atc
│   └── context_window.atc
├── orchestration/
│   ├── decision_handler.atc
│   ├── sync_orchestrator.atc
│   └── workflow_engine.atc
├── requirements.txt
├── runtime/
│   ├── agent_runtime.atc
│   ├── llm_router.atc
│   └── tool_executor.atc
└── src/
    ├── __init__.py
    ├── agents/
    │   └── __init__.py
    ├── core/
    │   └── __init__.py
    └── memory/
        └── __init__.py
```

## Module Descriptions
- **src/core/**: Core AI orchestrator, model hub dispatcher, prompt manager, and LLM inference engine.
- **src/memory/**: Vector store abstraction, persistent long-term memory retrieval, and embedding index.
- **src/agents/**: Specialized autonomous AI agents (Architect, Coding, Knowledge, Research, Security).
- **requirements.txt**: Python package manifest specifying AI framework dependencies.

## Build System
Python 3.10+ packaging system (`setuptools` / `poetry`), Python venv virtual environment.

## Dependencies
Python 3.10+, `pydantic`, `numpy`, `chromadb` / `faiss-cpu`, `langchain` / `llama-index`, `openai` / `anthropic`.
