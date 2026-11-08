# CI/CD Fix — Manual Push Required

GitHub's OAuth token doesn't have `workflow` scope, so the agent cannot push
workflow file changes directly. These files must be pushed manually by Michael.

## How to apply

```bash
cd ci-cd-fix
bash apply-fix.sh
```

Or manually:
```bash
cp ci-cd-fix/ci-cd.yml .github/workflows/ci-cd.yml
cp ci-cd-fix/codeql.yml .github/workflows/codeql.yml
git add .github/workflows/
git commit -m "fix(ci): ATCLang native CI/CD + CodeQL"
git push origin main
```

## What changes

### ci-cd.yml
- Removed Solidity tests (contracts deleted)
- Removed `npm ci` (branch-protection issue)
- Added ATCLang parse check (validates all .atc files)
- ATCLang test suite with coverage
- Security scan (bandit on atclang/ only)
- Docker build + staging deploy

### codeql.yml
- Replaced broken German-translated workflow
- Standard CodeQL analysis for Python
