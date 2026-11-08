#!/bin/bash

# A-TownChain OS — Dokumentation Sync-Script
# Synchronisiert a-townchain-os-docs Dateien in a-townchain-os/docs/
# Usage: ./scripts/sync-docs.sh [--commit] [--push]

set -e

# Farben
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Konfiguration
SOURCE_REPO="${SOURCE_REPO:-./../a-townchain-os-docs}"  # Kann via ENV überschrieben werden
TARGET_DOCS="docs"
COMMIT_FLAG=false
PUSH_FLAG=false

# Optionen parsen
while [[ $# -gt 0 ]]; do
  case $1 in
    --commit)
      COMMIT_FLAG=true
      shift
      ;;
    --push)
      PUSH_FLAG=true
      COMMIT_FLAG=true
      shift
      ;;
    --source)
      SOURCE_REPO="$2"
      shift 2
      ;;
    *)
      echo "Unbekannte Option: $1"
      echo "Usage: $0 [--commit] [--push] [--source <path>]"
      exit 1
      ;;
  esac
done

echo -e "${GREEN}=== A-TownChain OS Docs Sync ===${NC}"
echo "Source: $SOURCE_REPO"
echo "Target: $TARGET_DOCS/"

# Prüfe ob Source existiert
if [ ! -d "$SOURCE_REPO" ]; then
  echo -e "${RED}✗ Source-Repo nicht gefunden: $SOURCE_REPO${NC}"
  echo "  Hinweis: Wenn das Repo an anderer Stelle ist, nutze:"
  echo "  SOURCE_REPO=/path/to/a-townchain-os-docs $0"
  exit 1
fi

# Stelle sicher, dass docs/ Verzeichnis existiert
mkdir -p "$TARGET_DOCS"

echo ""
echo -e "${YELLOW}Synchronisiere Dateien...${NC}"

# Kopiere alle .md Dateien
echo "  → Main Documentation (.md Dateien)..."
if [ -d "$SOURCE_REPO/docs" ]; then
  rsync -av --delete "$SOURCE_REPO/docs/" "$TARGET_DOCS/" \
    --exclude=".git" \
    --exclude=".gitignore" \
    --exclude="node_modules" \
    --exclude="*.tmp"
  echo -e "    ${GREEN}✓ docs/ synchronisiert${NC}"
else
  echo -e "    ${YELLOW}⚠ $SOURCE_REPO/docs/ nicht gefunden (optional)${NC}"
fi

# Kopiere Root-Dateien (ausgewählte)
echo "  → Root-Dateien..."
for file in KONSOLIDIERUNGS_ROADMAP.md NAMING_CONVENTIONS.md DECISIONS_REGISTER.md AGENT_POLICY.md CHANGELOG.md; do
  if [ -f "$SOURCE_REPO/$file" ]; then
    cp "$SOURCE_REPO/$file" "$TARGET_DOCS/$file"
    echo -e "    ${GREEN}✓ $file${NC}"
  fi
done

# Kopiere Standards
echo "  → Standards Registry..."
if [ -d "$SOURCE_REPO/docs/standards" ]; then
  rsync -av "$SOURCE_REPO/docs/standards/" "$TARGET_DOCS/standards/" \
    --exclude=".git"
  echo -e "    ${GREEN}✓ standards/ synchronisiert${NC}"
else
  echo -e "    ${YELLOW}⚠ Standards-Verzeichnis nicht gefunden${NC}"
fi

# Kopiere CI-Templates
echo "  → CI-Templates..."
if [ -d "$SOURCE_REPO/docs/ci-templates" ]; then
  rsync -av "$SOURCE_REPO/docs/ci-templates/" "$TARGET_DOCS/ci-templates/" \
    --exclude=".git"
  echo -e "    ${GREEN}✓ ci-templates/ synchronisiert${NC}"
else
  echo -e "    ${YELLOW}⚠ CI-Templates nicht gefunden${NC}"
fi

echo ""
echo -e "${GREEN}✓ Synchronisierung abgeschlossen${NC}"

# Status anzeigen
echo ""
echo "Geänderte Dateien:"
git diff --name-only --no-pager docs/ 2>/dev/null || echo "  (Keine git-Repo oder kein Diff)"

# Commit (optional)
if [ "$COMMIT_FLAG" = true ]; then
  echo ""
  echo -e "${YELLOW}Erstelle Commit...${NC}"
  
  git add "docs/" "$TARGET_DOCS"/*.md 2>/dev/null || true
  
  STATUS=$(git status --porcelain 2>/dev/null | wc -l)
  if [ "$STATUS" -gt 0 ]; then
    git commit -m "docs: Sync from a-townchain-os-docs ($(date +%Y-%m-%d\ %H:%M:%S))

Automated sync of:
- Technical documentation
- Standards registry
- Decisions register
- Naming conventions
- CI templates

Agent: Sync-Script" 2>/dev/null || {
      echo -e "${YELLOW}⚠ Commit fehlgeschlagen (möglicherweise keine Änderungen)${NC}"
    }
  else
    echo -e "${YELLOW}⚠ Keine Änderungen zum Commit${NC}"
  fi
  
  # Push (optional)
  if [ "$PUSH_FLAG" = true ]; then
    echo ""
    echo -e "${YELLOW}Pushe zu main...${NC}"
    
    CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null)
    if [ "$CURRENT_BRANCH" = "main" ]; then
      git push origin main 2>/dev/null && \
        echo -e "  ${GREEN}✓ Push erfolgreich${NC}" || \
        echo -e "  ${RED}✗ Push fehlgeschlagen${NC}"
    else
      echo -e "  ${YELLOW}⚠ Nicht auf main Branch ($CURRENT_BRANCH). Nutze: git push origin $CURRENT_BRANCH${NC}"
    fi
  fi
fi

echo ""
echo -e "${GREEN}=== Sync abgeschlossen ===${NC}"
