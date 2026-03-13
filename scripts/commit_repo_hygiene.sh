#!/usr/bin/env bash
# =============================================================================
# commit_repo_hygiene.sh  —  MAINTAINER VERSION
#
# Second cleanup pass: docs, CI, repo hygiene, and removing committed
# dev artefacts that are visible on the live GitHub repo.
#
# Run AFTER commit_oss_fixes.sh, from the stratarouter repo root.
# Commits everything as one squashed commit on the current branch.
#
# Usage:
#   bash scripts/commit_repo_hygiene.sh
# =============================================================================

set -euo pipefail

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
CYAN='\033[0;36m'; BOLD='\033[1m'; NC='\033[0m'

step() { echo -e "\n${CYAN}${BOLD}▶  $*${NC}"; }
ok()   { echo -e "  ${GREEN}✓${NC}  $*"; }
warn() { echo -e "  ${YELLOW}⚠${NC}   $*"; }
die()  { echo -e "\n${RED}✗ ERROR: $*${NC}\n" >&2; exit 1; }

[[ -d ".git" ]] || die "Run from the stratarouter repo root."

CURRENT=$(git rev-parse --abbrev-ref HEAD)
ok "On branch: $CURRENT"

# ── Stage changed/new files ───────────────────────────────────────────────────
step "Staging changed files"

CHANGED_FILES=(
    "README.md"
    "CHANGELOG.md"
    "MIGRATION.md"
    "CONTRIBUTING.md"
    ".gitignore"
    ".github/workflows/ci.yml"
    ".github/dependabot.yml"
    ".github/pull_request_template.md"
    ".github/ISSUE_TEMPLATE/bug_report.md"
    ".github/ISSUE_TEMPLATE/feature_request.md"
    "docs/assets/logo.png"
)

for f in "${CHANGED_FILES[@]}"; do
    [[ -f "$f" ]] || die "File not found: $f"
    ok "$f"
done

git add -- "${CHANGED_FILES[@]}"

# ── Remove committed dev artefacts from tracking ─────────────────────────────
step "Removing dev artefacts from git tracking"

# These files are present on GitHub but should never have been committed.
# 'git rm --cached' removes them from tracking without deleting local copies.
DEV_ARTEFACTS=("py2.py" "v1.py" "test.py" "sample.sh" "test2.sh" "pyvenv.cfg" "lib64")

for f in "${DEV_ARTEFACTS[@]}"; do
    if git ls-files --error-unmatch "$f" 2>/dev/null; then
        git rm --cached "$f" --quiet
        ok "Untracked from git: $f"
    else
        warn "Not tracked (skipping): $f"
    fi
done

echo ""
git diff --cached --stat
echo ""

if git diff --cached --quiet; then
    warn "Nothing to commit — all changes already at HEAD."
    exit 0
fi

# ── Commit ────────────────────────────────────────────────────────────────────
step "Committing"

git commit -m "chore: repo hygiene — docs, CI, issue templates, remove dev artefacts

## Documentation
- README.md: corrected architecture diagram (LinearIndex/vector_ops, not
  HNSW/simd_ops); updated Quick Start to use canonical Route(name, utterances)
  API; removed false HNSW/AVX2/isotonic claims from benchmark table; fixed
  clone URL (was wrong org); unified footer branding to StrataRouter Contributors
- CHANGELOG.md: added v0.2.1 entry listing all OSS-review fixes; corrected
  internal module names (vector_ops.rs, LinearIndex, ScoreNormalizer); fixed
  Route API description to match canonical name/utterances schema
- MIGRATION.md: complete rewrite — was backwards after API unification;
  now correctly documents RouteLayer (no change needed) vs Router (new),
  and RouteChoice vs RouteResult fields
- CONTRIBUTING.md: removed Python triple-quote wrapper that made the entire
  file render as a code block on GitHub; updated email to hello@stratarouter.dev;
  updated clone URL to ai-deeptech org; improved PR checklist

## CI / GitHub Actions
- ci.yml: removed continue-on-error: true from pytest and cargo audit jobs
  (CI was never blocking merges — any broken commit could land on main);
  added concurrency cancel-in-progress; added mypy type-check step;
  added pip-audit Python dependency vulnerability scan; pinned pip install
  to python/pyproject.toml working-directory
- dependabot.yml: added weekly automated dependency updates for pip (python/),
  cargo (core/), and github-actions
- .github/pull_request_template.md: standardised PR checklist
- .github/ISSUE_TEMPLATE/bug_report.md: structured bug report template
- .github/ISSUE_TEMPLATE/feature_request.md: structured feature request template

## Repo hygiene
- .gitignore: added py2.py, v1.py, test.py, sample.sh, test2.sh, pyvenv.cfg,
  lib64, .python-version (dev/virtualenv artefacts that were committed to main)
- Removed from git tracking (git rm --cached): py2.py, v1.py, test.py,
  sample.sh, test2.sh, pyvenv.cfg, lib64 — these files are still on disk
  but will no longer appear in the repository"

COMMIT_SHA=$(git rev-parse --short HEAD)

echo ""
echo -e "${GREEN}${BOLD}══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}${BOLD}  ✅  Committed  [${COMMIT_SHA}]  on branch: ${CURRENT}${NC}"
echo -e "${GREEN}${BOLD}══════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "  ${BOLD}Verify:${NC}"
echo -e "    ${CYAN}git show --stat HEAD${NC}"
echo -e "    ${CYAN}git log --oneline -5${NC}"
echo ""
echo -e "  ${BOLD}Push and tag:${NC}"
echo -e "    ${CYAN}git push origin ${CURRENT}${NC}"
echo -e "    ${CYAN}git tag -a v0.2.1 -m 'v0.2.1 — OSS quality pass'${NC}"
echo -e "    ${CYAN}git push origin v0.2.1${NC}"
echo ""
