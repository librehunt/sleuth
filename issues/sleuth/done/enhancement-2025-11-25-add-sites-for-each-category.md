---
type: enhancement
status: done
created: 2025-11-25
priority: medium
---

# Add One Site Implementation Per Category

## Description
Add one popular site implementation for each of the 6 empty site categories (social, forum, gaming, professional, nsfw, other). Currently only the `dev` category has an implementation (GitHub).

## Proposed Sites

| Category | Site | URL Pattern |
|----------|------|-------------|
| Social | Twitter/X | `https://x.com/{}` |
| Forum | Reddit | `https://www.reddit.com/user/{}` |
| Gaming | Steam | `https://steamcommunity.com/id/{}` |
| Professional | LinkedIn | `https://www.linkedin.com/in/{}` |
| NSFW | OnlyFans | `https://onlyfans.com/{}` |
| Other | Medium | `https://medium.com/@{}` |

## Implementation Tasks

- [ ] **Social**: Implement Twitter/X checker
  - [ ] Create `src/sites/social/twitter.rs`
  - [ ] Implement `Site` trait
  - [ ] Add tests
  - [ ] Update `src/sites/social/mod.rs`

- [ ] **Forum**: Implement Reddit checker
  - [ ] Create `src/sites/forum/reddit.rs`
  - [ ] Implement `Site` trait
  - [ ] Add tests
  - [ ] Update `src/sites/forum/mod.rs`

- [ ] **Gaming**: Implement Steam checker
  - [ ] Create `src/sites/gaming/steam.rs`
  - [ ] Implement `Site` trait
  - [ ] Add tests
  - [ ] Update `src/sites/gaming/mod.rs`

- [ ] **Professional**: Implement LinkedIn checker
  - [ ] Create `src/sites/professional/linkedin.rs`
  - [ ] Implement `Site` trait
  - [ ] Add tests
  - [ ] Update `src/sites/professional/mod.rs`

- [ ] **NSFW**: Implement OnlyFans checker
  - [ ] Create `src/sites/nsfw/onlyfans.rs`
  - [ ] Implement `Site` trait
  - [ ] Add tests
  - [ ] Update `src/sites/nsfw/mod.rs`

- [ ] **Other**: Implement Medium checker
  - [ ] Create `src/sites/other/medium.rs`
  - [ ] Implement `Site` trait
  - [ ] Add tests
  - [ ] Update `src/sites/other/mod.rs`

- [ ] **Quality Gates**
  - [ ] Run `cargo fmt`
  - [ ] Run `cargo clippy`
  - [ ] Run `cargo test`
  - [ ] Update documentation (README, CHANGELOG)

## Branch
`feat/add-sites-per-category`

## Acceptance Criteria
- All 6 sites are implemented with the `Site` trait
- Each site has unit tests
- All tests pass
- Code passes fmt and clippy
- Documentation is updated
