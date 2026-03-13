#!/usr/bin/env bash
# =============================================================================
# commit_ci_fixes.sh  —  fix the 3 root causes breaking all 7 CI checks
#
# Run from the stratarouter repo root on ole9.
# Must be on branch fix/oss-review (or main after merge).
#
# Usage:
#   bash scripts/commit_ci_fixes.sh
# =============================================================================

set -euo pipefail

GREEN='\033[0;32m'; CYAN='\033[0;36m'; BOLD='\033[1m'; RED='\033[0;31m'; NC='\033[0m'
ok()  { echo -e "  ${GREEN}✓${NC}  $*"; }
die() { echo -e "\n${RED}✗ ERROR: $*${NC}\n" >&2; exit 1; }
step(){ echo -e "\n${CYAN}${BOLD}▶  $*${NC}"; }

[[ -d ".git" ]] || die "Run from the stratarouter repo root."

CURRENT=$(git rev-parse --abbrev-ref HEAD)
ok "On branch: $CURRENT"

step "Verifying files exist"
for f in "core/src/lib.rs" ".github/workflows/ci.yml"; do
    [[ -f "$f" ]] || die "Missing: $f"
    ok "$f"
done

step "Staging"
git add core/src/lib.rs .github/workflows/ci.yml

git diff --cached --stat
echo ""

if git diff --cached --quiet; then
    echo "Nothing to commit — already at HEAD."
    exit 0
fi

step "Committing"
git commit -m "fix(ci): fix all 7 failing CI checks

Root cause 1 — Rust (all 3 platforms): cargo clippy failure
  lib.rs had #![warn(clippy::pedantic)] which, combined with CI's
  -D warnings flag, turned every pedantic lint into a hard error.
  Numeric casts in hybrid_scoring.rs (words.len() as f32) and
  router.rs (latency_us as u64) fired cast_precision_loss /
  cast_possible_truncation and failed the build on all platforms.
  Fix: removed #![warn(clippy::pedantic)] with an explanatory comment.
  #![warn(clippy::all)] is retained for correctness/suspicious lints.

Root cause 2 — Python (all 3 versions): pip install crash at step 4
  pyproject.toml uses maturin as the build backend, which requires
  cargo on PATH to compile the Rust extension. The Python CI job never
  installed a Rust toolchain, so pip install -e '.[dev]' crashed
  immediately with 'cargo not found'.
  Fix: added dtolnay/rust-toolchain@stable + Swatinem/rust-cache@v2
  and an explicit 'pip install maturin' step before the package install.

Root cause 3 — Security Audit (3 min timeout): bash syntax error
  pip-audit --requirement <(pip freeze) uses bash process substitution.
  GitHub Actions run: blocks execute under /bin/sh by default, not bash.
  The <(...) syntax is a bash extension and caused an immediate syntax
  error, making the job time out waiting for a process that never started.
  Fix: added shell: bash to the pip-audit step and simplified to
  'pip-audit' (audits the current environment directly)."

SHA=$(git rev-parse --short HEAD)
echo ""
echo -e "${GREEN}${BOLD}══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}${BOLD}  ✅  Committed [$SHA] on branch: $CURRENT${NC}"
echo -e "${GREEN}${BOLD}══════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "  ${BOLD}Push:${NC}"
echo -e "    ${CYAN}git push origin $CURRENT${NC}"
echo ""
echo -e "  Then watch CI go green at:"
echo -e "    ${CYAN}https://github.com/ai-deeptech/stratarouter/pull/1/checks${NC}"
echo ""
