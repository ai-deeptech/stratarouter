# ═══════════════════════════════════════════════════════════
# PERMANENT FIX — paste this entire block on ole9
# ═══════════════════════════════════════════════════════════

set -e
REPO=/home/opc/backup/new/stratarouter

# ── 1. Install tools ─────────────────────────────────────────
pip install ruff --break-system-packages -q
rustup component add rustfmt clippy 2>/dev/null || true

# ── 2. Python: let ruff fix everything automatically ─────────
cd $REPO/python
ruff check stratarouter/ --fix --unsafe-fixes
echo "=== Ruff check after fix (should be 0 errors) ==="
ruff check stratarouter/
RUFF_EXIT=$?
echo "=== Ruff exit: $RUFF_EXIT ==="

# ── 3. Rust: let rustfmt fix formatting ──────────────────────
cd $REPO/core
cargo fmt
echo "=== cargo fmt done ==="

# ── 4. Rust: let clippy --fix fix what it can ────────────────
cargo clippy --fix --allow-dirty --allow-staged -- -D warnings 2>&1 | tail -30
echo "=== clippy --fix done ==="

# ── 5. Verify both are clean ─────────────────────────────────
echo ""
echo "=== Final ruff check ==="
cd $REPO/python && ruff check stratarouter/
RUFF_EXIT=$?

echo ""
echo "=== Final cargo fmt check ==="
cd $REPO/core && cargo fmt --check
FMT_EXIT=$?

echo ""
echo "=== Final clippy check ==="
cd $REPO/core && cargo clippy -- -D warnings 2>&1 | tail -40
CLIPPY_EXIT=$?

echo ""
echo "Ruff: $RUFF_EXIT | fmt: $FMT_EXIT | clippy: $CLIPPY_EXIT"

# ── 6. Commit and push only if all clean ─────────────────────
if [ "$RUFF_EXIT" = "0" ] && [ "$FMT_EXIT" = "0" ] && [ "$CLIPPY_EXIT" = "0" ]; then
  echo "=== ALL CLEAN — committing ==="
  cd $REPO
  git add -A
  git commit \
    --author="natarajanchandra02-afk <natarajanchandra02@users.noreply.github.com>" \
    -m "fix: auto-fix all ruff and rustfmt/clippy errors via tooling"
  git push origin fix/oss-review
  echo "=== PUSHED — CI should now pass ==="
else
  echo "=== STILL HAS ERRORS — showing remaining ==="
  echo "--- Ruff errors ---"
  cd $REPO/python && ruff check stratarouter/ 2>&1
  echo "--- Clippy errors ---"
  cd $REPO/core && cargo clippy -- -D warnings 2>&1 | grep "^error" | head -30
  echo "--- Fmt diff ---"
  cd $REPO/core && cargo fmt --check 2>&1
fi
