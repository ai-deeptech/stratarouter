REPO=/home/opc/backup/new/stratarouter

# Fix: rename PyO3 init function from stratarouter_core → _core
# Python expects PyInit__core because module-name = "stratarouter._core"
sed -i 's/^fn stratarouter_core(_py: Python/fn _core(_py: Python/' \
    $REPO/core/src/lib.rs

# Verify
grep -n "^fn _core\|^fn stratarouter_core" $REPO/core/src/lib.rs

# cargo fmt to keep it clean
cd $REPO/core && cargo fmt

# Commit and push
cd $REPO
git add -A
git commit \
  --author="natarajanchandra02-afk <natarajanchandra02@users.noreply.github.com>" \
  -m "fix: rename PyO3 module init fn to _core to match module-name stratarouter._core"
git push origin fix/oss-review
echo "PUSHED"
