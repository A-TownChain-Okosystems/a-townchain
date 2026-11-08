# Contributing — A-TownChain

## Setup
```bash
git clone https://github.com/A-TownChain-Okosystems/a-townchain-os
cd a-townchain-os
pip install -r backend/requirements.txt
python -m pytest tests/ -v
python start.py --mode dev
```

## Branches
main (production) | develop (integration) | feature/* | fix/* | release/*

## Commit Convention
feat|fix|sec|docs|perf|test|refactor(scope): message
Scopes: atclang shivaos blockchain contracts gateway gaming bridge ai pkg

## Standards
- Python: PEP8 + Black + Type Hints + Docstrings
- ATCLang: ATC-0001 bis ATC-0008
- Tests: pytest, 80%+ Coverage
- Security: ATCLang Analyzer vor jedem PR
- NO POSIX-cloning (proprietary tech only)

## PR Process
1. Branch from develop
2. Write tests first (TDD)
3. CI must be green
4. 1+ core team review
5. Squash merge
