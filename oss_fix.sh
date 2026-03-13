#!/usr/bin/env bash
# =============================================================================
# commit_oss_fixes.sh
#
# Replays all 10 OSS-review fix commits for StrataRouter.
# Contributor: natarajanchandra02-afk
#
# Usage (run from the repo root):
#   bash scripts/commit_oss_fixes.sh
#
# What it does:
#   1. Configures git identity to the contributor for this repo only.
#   2. Creates branch  fix/oss-review  from the current HEAD of main.
#   3. Stages exactly the right files per commit and commits them in order.
#   4. Prints a push command at the end — you run that manually.
#
# Requirements:
#   - git >= 2.28
#   - Run from the stratarouter repo root (where .git/ lives)
#   - All fixed files are already on disk (written by Claude via MCP)
# =============================================================================

set -euo pipefail

# ── Colours for output ────────────────────────────────────────────────────────
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
CYAN='\033[0;36m'; BOLD='\033[1m'; NC='\033[0m'

step()  { echo -e "\n${CYAN}${BOLD}▶ $*${NC}"; }
ok()    { echo -e "  ${GREEN}✓${NC} $*"; }
warn()  { echo -e "  ${YELLOW}⚠${NC}  $*"; }
die()   { echo -e "\n${RED}✗ ERROR: $*${NC}\n" >&2; exit 1; }

# ── Guard: must be run from repo root ─────────────────────────────────────────
[[ -d ".git" ]] || die "Run this script from the stratarouter repo root (where .git/ lives)."

# ── Contributor identity (local to this repo only) ────────────────────────────
CONTRIBUTOR_NAME="natarajanchandra02-afk"
CONTRIBUTOR_EMAIL="natarajanchandra02@users.noreply.github.com"
BRANCH="fix/oss-review"

step "Configuring git identity (local to this repo)"
git config user.name  "$CONTRIBUTOR_NAME"
git config user.email "$CONTRIBUTOR_EMAIL"
ok "name:  $CONTRIBUTOR_NAME"
ok "email: $CONTRIBUTOR_EMAIL"

# ── Switch to / create branch ─────────────────────────────────────────────────
step "Preparing branch: $BRANCH"
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)

if git show-ref --quiet "refs/heads/$BRANCH"; then
    warn "Branch '$BRANCH' already exists — switching to it."
    git checkout "$BRANCH"
else
    git checkout -b "$BRANCH"
    ok "Created and switched to $BRANCH"
fi

# =============================================================================
# Helper: stage + commit
# Usage: commit_files "message" file1 file2 ...
# =============================================================================
commit_files() {
    local msg="$1"; shift
    local files=("$@")

    for f in "${files[@]}"; do
        [[ -f "$f" ]] || die "Expected file not found: $f\n  Make sure all MCP-written files are present."
    done

    git add -- "${files[@]}"

    if git diff --cached --quiet; then
        warn "Nothing new to stage for: $msg"
        return
    fi

    git commit -m "$msg"
    ok "Committed: $msg"
}

# =============================================================================
# COMMIT 1 — LinearIndex (rename HnswIndex, return Result, remove Arc)
# =============================================================================
step "Commit 1/10 — LinearIndex: rename HnswIndex, Result not panic, remove Arc"
commit_files \
    "fix(rust/index): rename HnswIndex → LinearIndex; return Result instead of panic

- Renamed HnswIndex → LinearIndex (honest O(N) brute-force description).
- LinearIndex::new/add/search now return Result<_> (no assert! panics).
- Removed Arc<RwLock<>> — Router owns index exclusively via &mut self.
- Updated index/mod.rs re-export and router.rs to use ? propagation.
- All tests updated to unwrap() Results.

Fixes: H1 (false HNSW claim), H4 (assert! panics), L2, M2" \
    "core/src/index/hnsw.rs" \
    "core/src/index/mod.rs" \
    "core/src/router.rs"

# =============================================================================
# COMMIT 2 — vector_ops (rename simd_ops, honest scalar doc)
# =============================================================================
step "Commit 2/10 — vector_ops: rename simd_ops, honest scalar docs"
commit_files \
    "fix(rust/algorithms): rename simd_ops → vector_ops; remove false SIMD claims

- New file algorithms/vector_ops.rs replaces simd_ops.rs.
- Module doc is honest: scalar ops now, AVX2 path planned.
- Added .clamp(-1.0, 1.0) on cosine_similarity return value.
- Added test_cosine_zero_vector and test_cosine_result_clamped.
- simd_ops.rs emptied with redirect comment (not compiled).
- algorithms/mod.rs updated to declare vector_ops.
- Removed unused wide = \"0.7\" and ndarray/nalgebra from Cargo.toml.

Fixes: H2 (false SIMD claim), L3 (mod.rs re-export)" \
    "core/src/algorithms/vector_ops.rs" \
    "core/src/algorithms/simd_ops.rs" \
    "core/src/algorithms/mod.rs"

# =============================================================================
# COMMIT 3 — ScoreNormalizer (rename IsotonicCalibrator)
# =============================================================================
step "Commit 3/10 — ScoreNormalizer: rename IsotonicCalibrator, honest docs"
commit_files \
    "fix(rust/algorithms): rename IsotonicCalibrator → ScoreNormalizer; honest docs

- IsotonicCalibrator renamed to ScoreNormalizer — the implementation is
  piecewise-linear interpolation over a fixed table, not fitted isotonic
  regression. Online fitting is planned for a future release.
- Module doc updated to remove false isotonic regression claim.
- Uncertainty constant 0.05 documented as a placeholder.
- CalibrationManager field renamed calibrators → normalizers.
- Monotonicity test updated with a clear assertion message.

Fixes: M6 (misleading name and docs)" \
    "core/src/algorithms/calibration.rs"

# =============================================================================
# COMMIT 4 — Remove "Proprietary" from hybrid_scoring.rs
# =============================================================================
step "Commit 4/10 — hybrid_scoring: remove Proprietary comment"
commit_files \
    "fix(rust/algorithms): remove \"Proprietary\" doc; describe algorithm accurately

- Top-level doc now reads: BM25 keyword matching + dense cosine similarity
  + pattern rules combined with learned weights and sigmoid normalisation.
- Extracted BM25 k1/b constants with explanatory comments.
- Added test_fuse_scores_monotone: higher inputs must produce higher output.

Fixes: H3 (\"Proprietary\" in open-source library)" \
    "core/src/algorithms/hybrid_scoring.rs"

# =============================================================================
# COMMIT 5 — Neutralise orphaned route.rs and similarity.rs
# =============================================================================
step "Commit 5/10 — neutralise orphaned route.rs and similarity.rs"
commit_files \
    "fix(rust): neutralise orphaned route.rs and similarity.rs (compile errors)

Both files referenced crate::error::StrataError which does not exist
(correct type is crate::error::Error). Not declared in lib.rs but
caused confusion and would cause compile errors if accidentally declared.

- Overwritten with empty redirect comments pointing to current homes.
- NOT declared as modules — NOT compiled.
- Marked safe-to-delete in a future clean-up PR.

Fixes: C2 (StrataError compile errors), L1 (dead code)" \
    "core/src/route.rs" \
    "core/src/similarity.rs"

# =============================================================================
# COMMIT 6 — cache.rs docs, lib.rs honest claims, Cargo.toml cleanup
# =============================================================================
step "Commit 6/10 — cache.rs docs; lib.rs honest claims; Cargo.toml metadata fix"
commit_files \
    "fix(rust): add cache.rs docs; honest lib.rs claims; fix Cargo.toml metadata

cache.rs:
- Added /// doc comments to all public methods.
  Resolves missing_docs lint warnings from #![warn(missing_docs)].
- Added test_clone_shares_storage verifying Arc-shared behaviour.

lib.rs:
- Removed false AVX2/HNSW/SIMD claims from crate-level docs.
- Accurate architecture overview: LinearIndex (O(N)), HybridScorer, etc.
- Added LinearIndex to top-level re-exports.

Cargo.toml:
- repository/homepage fixed to https://github.com/ai-deeptech/stratarouter.
- authors unified to StrataRouter Contributors <hello@stratarouter.dev>.
- Removed unused wide, ndarray, nalgebra dependencies.
- Removed overflow-checks = false from [profile.release] (safety risk).

Fixes: H6 (missing docs), H8 (wrong URLs), H9 (branding), H10 (overflow)" \
    "core/src/cache.rs" \
    "core/src/lib.rs" \
    "core/Cargo.toml"

# =============================================================================
# COMMIT 7 — Unify Python Route + add RouteLayer + single version
# =============================================================================
step "Commit 7/10 — unify Python Route; add RouteLayer; single __version__ source"
commit_files \
    "fix(python): unify Route schema; add RouteLayer; single __version__ source

route.py (canonical public Route):
- Route uses name + utterances fields (semantic-router compatible).
- RouteChoice with is_match property and __bool__ support.
- __all__, full docstrings, field validators.

types.py:
- Renamed Route → RouteConfig (internal; used only by Rust FFI layer).

layer.py (NEW):
- RouteLayer — pure-Python, no Rust core required.
- Compatible with semantic-router's RouteLayer API.
- __call__, route_batch, add, remove, clear, num_routes, list_route_names.
- Validates encoder has encode() and dimension at construction time.

__init__.py:
- Exports: Route, RouteChoice, RouteLayer, Router, DeploymentMode.
- Explicit __all__. Version from __version__ only.

__version__.py:
- Single source of truth via importlib.metadata.version('stratarouter').
- Falls back to '0.0.0.dev0' when package is not installed.

Fixes: C1 (dual Route), C3 (RouteLayer missing), C5 (3 version values), M7 (__all__)" \
    "python/stratarouter/route.py" \
    "python/stratarouter/types.py" \
    "python/stratarouter/layer.py" \
    "python/stratarouter/__init__.py" \
    "python/stratarouter/__version__.py"

# =============================================================================
# COMMIT 8 — Fix cohere encoder
# =============================================================================
step "Commit 8/10 — cohere encoder: lazy import, BaseEncoder ABC, cohere v5"
commit_files \
    "fix(python/encoders): cohere lazy import; implement BaseEncoder ABC; cohere v5

- Moved 'import cohere' inside __init__ (was at module top-level causing
  ImportError for ALL users without cohere installed).
- CohereEncoder now properly inherits BaseEncoder and implements:
    encode(text: str | list[str]) -> np.ndarray  (was __call__)
    dimension: int property                       (was dim)
- Cohere SDK v5+ uses ClientV2; added getattr fallback for v4 compat.
- Added _KNOWN_DIMS table for all current Cohere model variants.
- Added CohereEncoder to encoders/__init__.py conditional imports.

Fixes: C4 (top-level import), L5 (deprecated cohere.Client)" \
    "python/stratarouter/encoders/cohere.py" \
    "python/stratarouter/encoders/__init__.py"

# =============================================================================
# COMMIT 9 — router.py import fix, version in save(), document auto-build
# =============================================================================
step "Commit 9/10 — router.py: import fix, dynamic version in save(), doc auto-build"
commit_files \
    "fix(python/router): fix Route import; dynamic version in save(); doc auto-build

router.py:
- Fixed stale 'from .types import Route' → uses Route from route.py.
- save() now reads version from __version__ (was hardcoded '0.2.0').
- Added note in route() docstring documenting auto-build side-effect.

integrations/generic.py:
- Fixed 'from ..types import Route' → 'from ..route import Route'.

Fixes: M8 (silent auto-build undocumented), L6 (hardcoded version in save)" \
    "python/stratarouter/router.py" \
    "python/stratarouter/integrations/generic.py"

# =============================================================================
# COMMIT 10 — Tests, pyproject.toml, retire artefacts
# =============================================================================
step "Commit 10/10 — fix tests; pyproject.toml; retire stale artefacts"
commit_files \
    "fix(python): fix tests; pyproject.toml URLs+extras; retire stale artefacts

tests/test_route.py:
- Complete rewrite — removed copy-pasted test_router.py block embedded
  mid-file (file was unparseable before this fix).
- 18 clean tests covering Route validation and RouteChoice.

tests/test_router.py:
- MockEncoder now implements encode() + dimension (satisfies BaseEncoder ABC).
- All tests use RouteLayer (was importing non-existent symbol → ImportError).
- Added edge-case tests: invalid encoder, batch routing, remove nonexistent.

pyproject.toml:
- All URLs fixed to https://github.com/ai-deeptech/stratarouter.
- Author: StrataRouter Contributors <hello@stratarouter.dev>.
- Added missing extras: langchain = [langchain-core], cloud = [httpx].
- Added ruff.lint config, mypy.ignore_missing_imports, pytest addopts.

integrations/langchain_new.py: emptied (verbatim duplicate of langchain.py).
backup_setup.py: emptied (stale dev artefact).

Fixes: C3 (RouteLayer tests), M1 (corrupted test_route.py), M3 (backup_setup),
       M4 (langchain_new.py), M5 (missing extras), H7 (langchain-core added),
       H8/H9 (URLs + branding in pyproject)" \
    "python/tests/test_route.py" \
    "python/tests/test_router.py" \
    "python/pyproject.toml" \
    "python/stratarouter/integrations/langchain_new.py" \
    "python/backup_setup.py"

# =============================================================================
# Done
# =============================================================================
echo ""
echo -e "${GREEN}${BOLD}════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}${BOLD}  ✅  All 10 commits applied on branch: $BRANCH${NC}"
echo -e "${GREEN}${BOLD}════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "  Committed as: ${BOLD}$CONTRIBUTOR_NAME${NC} <$CONTRIBUTOR_EMAIL>"
echo ""
echo -e "  ${BOLD}Next step — push and open a PR:${NC}"
echo ""
echo -e "    ${CYAN}git push origin $BRANCH${NC}"
echo ""
echo -e "  Then open a Pull Request on GitHub:"
echo -e "    ${CYAN}https://github.com/ai-deeptech/stratarouter/compare/$BRANCH${NC}"
echo ""
echo -e "  To verify the log before pushing:"
echo -e "    ${CYAN}git log --oneline -10${NC}"
echo ""
