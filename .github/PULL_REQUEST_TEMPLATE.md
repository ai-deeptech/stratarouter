## Summary

## Type: [ ] Bug fix  [ ] Feature  [ ] Docs  [ ] Performance

## Checklist
- [ ] `cargo test --release` passes
- [ ] `pytest tests/ -v` passes
- [ ] `CHANGELOG.md` updated
- [ ] No secrets committed (`git diff HEAD | grep -i "api_key\|password\|secret"`)
- [ ] `cargo clippy -- -D warnings` clean
