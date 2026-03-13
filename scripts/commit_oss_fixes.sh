#!/usr/bin/env bash
# =============================================================================
# commit_oss_fixes.sh  —  MAINTAINER VERSION (single squashed commit)
#
# Stages all OSS-review fixes and lands them as ONE commit on main.
# Run this as the repo maintainer — no PR needed.
#
# Usage (from the stratarouter repo root):
#   bash scripts/commit_oss_fixes.sh
#
# Requirements:
#   - git >= 2.28
#   - Run from the repo root (where .git/ lives)
#   - All fixed files already on disk (written by Claude via MCP)
# =============================================================================

set -euo pipefail

# ── Colours ───────────────────────────────────────────────────────────────────
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
CYAN='\033[0;36m'; BOLD='\033[1m'; NC='\033[0m'

step() { echo -e "\n${CYAN}${BOLD}▶  $*${NC}"; }
ok()   { echo -e "  ${GREEN}✓${NC}  $*"; }
warn() { echo -e "  ${YELLOW}⚠${NC}   $*"; }
die()  { echo -e "\n${RED}✗ ERROR: $*${NC}\n" >&2; exit 1; }

# ── Guard ─────────────────────────────────────────────────────────────────────
[[ -d ".git" ]] || die "Run from the stratarouter repo root (where .git/ lives)."

# ── Config ────────────────────────────────────────────────────────────────────
MAINTAINER_NAME="${GIT_AUTHOR_NAME:-}"
MAINTAINER_EMAIL="${GIT_AUTHOR_EMAIL:-}"

# Use whatever identity is already configured if not overridden
if [[ -z "$MAINTAINER_NAME" ]]; then
    MAINTAINER_NAME=$(git config user.name 2>/dev/null || echo "")
fi
if [[ -z "$MAINTAINER_EMAIL" ]]; then
    MAINTAINER_EMAIL=$(git config user.email 2>/dev/null || echo "")
fi

[[ -n "$MAINTAINER_NAME" ]]  || die "Git user.name not set. Run: git config user.name 'Your Name'"
[[ -n "$MAINTAINER_EMAIL" ]] || die "Git user.email not set. Run: git config user.email 'you@example.com'"

step "Git identity"
ok "name:  $MAINTAINER_NAME"
ok "email: $MAINTAINER_EMAIL"

# ── Ensure we are on main (or master) ─────────────────────────────────────────
step "Checking branch"
CURRENT=$(git rev-parse --abbrev-ref HEAD)
if [[ "$CURRENT" != "main" && "$CURRENT" != "master" ]]; then
    warn "You are on '$CURRENT', not main/master."
    read -rp "  Continue anyway? (y/N) " confirm
    [[ "$confirm" =~ ^[Yy]$ ]] || { echo "Aborted."; exit 0; }
fi
ok "On branch: $CURRENT"

# ── Verify all expected files are present ─────────────────────────────────────
step "Verifying all fixed files exist on disk"

REQUIRED_FILES=(
    # Rust core
    "core/src/index/hnsw.rs"
    "core/src/index/mod.rs"
    "core/src/router.rs"
    "core/src/algorithms/vector_ops.rs"
    "core/src/algorithms/simd_ops.rs"
    "core/src/algorithms/mod.rs"
    "core/src/algorithms/calibration.rs"
    "core/src/algorithms/hybrid_scoring.rs"
    "core/src/route.rs"
    "core/src/similarity.rs"
    "core/src/cache.rs"
    "core/src/lib.rs"
    "core/Cargo.toml"
    # Python SDK
    "python/stratarouter/route.py"
    "python/stratarouter/types.py"
    "python/stratarouter/layer.py"
    "python/stratarouter/__init__.py"
    "python/stratarouter/__version__.py"
    "python/stratarouter/encoders/cohere.py"
    "python/stratarouter/encoders/__init__.py"
    "python/stratarouter/router.py"
    "python/stratarouter/integrations/generic.py"
    "python/stratarouter/integrations/langchain_new.py"
    "python/tests/test_route.py"
    "python/tests/test_router.py"
    "python/pyproject.toml"
    "python/backup_setup.py"
    # gitignore update
    ".gitignore"
)

MISSING=0
for f in "${REQUIRED_FILES[@]}"; do
    if [[ -f "$f" ]]; then
        ok "$f"
    else
        warn "MISSING: $f"
        MISSING=$((MISSING + 1))
    fi
done

[[ $MISSING -eq 0 ]] || die "$MISSING file(s) missing. Ensure all MCP-written files are present before running."

# ── Stage everything ──────────────────────────────────────────────────────────
step "Staging all changed files"
git add -- "${REQUIRED_FILES[@]}"

# Show what is about to be committed
echo ""
git diff --cached --stat
echo ""

# Guard: nothing to commit means script was already run
if git diff --cached --quiet; then
    warn "Nothing to commit — all files already match HEAD."
    echo "  If you expected changes, check 'git status'."
    exit 0
fi

# ── Single squashed commit ────────────────────────────────────────────────────
step "Creating commit"

COMMIT_MSG="refactor: OSS quality pass — fix all 29 review issues (v0.2.0)

Addresses every item from the OSS code-review conducted against Google
open-source best practices and PyPI/crates.io publishing standards.

## Critical fixes (5)
- C1: Unified dual Python Route schemas (types.py Route→RouteConfig,
      route.py is now the single canonical public Route with name/utterances)
- C2: Neutralised orphaned route.rs + similarity.rs that referenced
      non-existent crate::error::StrataError (compile errors on --features python)
- C3: Added RouteLayer (layer.py) and exported it from __init__.py;
      fixed MockEncoder in tests to implement BaseEncoder ABC
- C4: Fixed cohere.py top-level import that crashed ALL users without
      cohere installed; now lazy-imported inside __init__
- C5: Version unified — single source via importlib.metadata in __version__.py;
      removed inline duplicate from __init__.py

## High severity fixes (10)
- H1: Renamed HnswIndex → LinearIndex with honest O(N) brute-force docs
      (real HNSW planned in ROADMAP.md)
- H2: Renamed simd_ops.rs → vector_ops.rs; removed false SIMD/AVX2 claims;
      removed unused 'wide' crate from Cargo.toml
- H3: Removed 'Proprietary' doc comment from hybrid_scoring.rs
- H4: LinearIndex::new/add/search now return Result<_> instead of assert! panics
- H5: Router errors propagate cleanly from core; ffi.rs defers to router
- H6: Added /// doc comments to all public methods in cache.rs
- H7: langchain-core added as optional dependency in pyproject.toml
- H8: Fixed repository/homepage URLs to https://github.com/ai-deeptech/stratarouter
- H9: Unified author/branding across Cargo.toml and pyproject.toml
- H10: Removed overflow-checks = false from [profile.release]

## Medium severity fixes (8)
- M1: Rewrote test_route.py (was corrupted with embedded duplicate content)
- M2: Removed Arc<RwLock<>> from LinearIndex (Router owns it via &mut self)
- M3: backup_setup.py retired and added to .gitignore
- M4: langchain_new.py emptied (was verbatim duplicate of langchain.py)
- M5: Added cloud and langchain optional-dependency groups to pyproject.toml
- M6: Renamed IsotonicCalibrator → ScoreNormalizer; removed false isotonic claim
- M7: Added __all__ to all public Python modules
- M8: Documented auto-build side-effect in router.route() docstring

## Low severity fixes (6)
- L1: route.rs and similarity.rs emptied (dead code with compile errors)
- L2: LinearIndex::new no longer uses assert! (see H4)
- L3: algorithms/mod.rs now declares vector_ops instead of simd_ops
- L4: CalibrationManager::calibrate_for_route kept &mut self for future fitting
- L5: CohereEncoder uses ClientV2 fallback for cohere SDK v5+ compatibility
- L6: router.save() reads version from __version__ (was hardcoded '0.2.0')

## Files changed
Rust core (13 files): index/hnsw.rs, index/mod.rs, router.rs,
  algorithms/vector_ops.rs, algorithms/simd_ops.rs, algorithms/mod.rs,
  algorithms/calibration.rs, algorithms/hybrid_scoring.rs,
  route.rs, similarity.rs, cache.rs, lib.rs, Cargo.toml

Python SDK (14 files): route.py, types.py, layer.py (NEW), __init__.py,
  __version__.py, encoders/cohere.py, encoders/__init__.py, router.py,
  integrations/generic.py, integrations/langchain_new.py,
  tests/test_route.py, tests/test_router.py, pyproject.toml, backup_setup.py

Config (1 file): .gitignore (added backup_setup.py)"

git commit -m "$COMMIT_MSG"

# ── Done ──────────────────────────────────────────────────────────────────────
COMMIT_SHA=$(git rev-parse --short HEAD)

echo ""
echo -e "${GREEN}${BOLD}══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}${BOLD}  ✅  Committed  ${COMMIT_SHA}  on branch: ${CURRENT}${NC}"
echo -e "${GREEN}${BOLD}══════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "  ${BOLD}Verify the commit:${NC}"
echo -e "    ${CYAN}git show --stat HEAD${NC}"
echo ""
echo -e "  ${BOLD}Push to remote:${NC}"
echo -e "    ${CYAN}git push origin ${CURRENT}${NC}"
echo ""
echo -e "  ${BOLD}Create a GitHub release (recommended):${NC}"
echo -e "    ${CYAN}git tag -a v0.2.0 -m 'v0.2.0 — OSS quality pass'${NC}"
echo -e "    ${CYAN}git push origin v0.2.0${NC}"
echo ""
