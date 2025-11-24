# Scaffold Repository Structure

**Type:** enhancement  
**Status:** done  
**Branch:** refactor/scaffold-repo-structure  
**Linked roadmap section:** Initial Setup

---

## 🧠 Context
The repository currently has a minimal structure with only `src/lib.rs` containing a placeholder function. We need to establish a proper architecture for sleuth (Rust implementation of sherlock) with modular organization, proper separation of concerns, and self-contained tests.

## 🎯 Goal
Create a well-organized repository structure that supports:
- Modular architecture for site checkers
- CLI interface
- HTTP client abstraction
- Configuration management
- Self-contained tests (tests in same files as code)
- Integration tests
- Examples and benchmarks

## 📏 Success Metrics
- [ ] All module directories created with proper mod.rs files
- [ ] Core traits and interfaces defined (Site trait, etc.)
- [ ] Basic module structure in place
- [ ] Tests can be run with `cargo test`

## 🧩 Acceptance Criteria
- [ ] Directory structure matches architecture design
- [ ] All modules have mod.rs files declaring submodules
- [ ] Core traits (Site, etc.) are defined
- [ ] Basic error types are defined
- [ ] Tests are self-contained in source files
- [ ] Integration test structure is in place
- [ ] Examples directory structure exists
- [ ] Benchmarks directory structure exists
- [ ] Documentation directories exist
- [ ] CHANGELOG entry added

## 🛠️ Implementation Outline
1. Create/switch to branch `refactor/scaffold-repo-structure`
2. Create directory structure:
   - `src/cli/` - CLI module
   - `src/core/` - Core engine functionality
   - `src/sites/` - Site-specific implementations
   - `src/http/` - HTTP client abstraction
   - `src/config/` - Configuration management
   - `src/data/` - Data structures
   - `src/utils/` - Utility functions
3. Create `mod.rs` files for each module
4. Define core traits (Site trait in `src/sites/site.rs`)
5. Define error types in `src/utils/error.rs`
6. Create basic data structures
7. Set up integration tests structure in `tests/`
8. Create examples and benches directories
9. Create docs directory with architecture documentation
10. Update Cargo.toml with necessary dependencies
11. Move this file to `in_progress/` then `done/`
12. Create PR referencing this issue

## 🔍 Alternatives Considered
- Flat structure → Rejected: Would become unmaintainable as project grows
- Separate crates → Considered: Too complex for initial structure, can refactor later if needed

## ⚠️ Risks / Mitigations
- Over-engineering → Mitigation: Start with essential modules, add more as needed
- Missing dependencies → Mitigation: Add dependencies incrementally as we implement features

## 🔗 Discussion Notes
Architecture discussed and agreed upon. Tests should be self-contained in source files using `#[cfg(test)] mod tests {}` pattern.

