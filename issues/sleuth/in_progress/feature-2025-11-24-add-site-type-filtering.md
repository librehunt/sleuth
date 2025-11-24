# Add Site Type/Category Filtering

**Type:** feature  
**Status:** in_progress  
**Branch:** feat/site-type-filtering  
**Linked roadmap section:** Core Features

---

## 🧠 Context
Users need the ability to filter sites by category/type (dev, social, nsfw, professional, gaming, forum, other) when searching for usernames. This allows users to:
- Search only specific categories (e.g., only dev sites)
- Exclude certain categories (e.g., exclude nsfw sites)
- Combine multiple categories
- Have better control over search scope

Currently, the `Site` trait and CLI don't support category filtering.

## 🎯 Goal
Implement site type/category system that:
- Defines site categories (dev, social, nsfw, professional, gaming, forum, other)
- Extends `Site` trait with `site_type()` method
- Adds CLI flag `--type` / `-t` for filtering (supports multiple values)
- Filters sites by type in the engine/scanner
- Updates site info structures to include type

## 📏 Success Metrics
- [ ] All sites can be categorized with a type
- [ ] CLI accepts `--type` flag with multiple values
- [ ] Engine correctly filters sites by type
- [ ] Users can search only specific categories (baseline: 0 categories → target: 7 categories)
- [ ] All tests passing with > 90% coverage

## 🧩 Acceptance Criteria
- [ ] `SiteType` enum defined with all categories
- [ ] `Site` trait extended with `site_type()` method
- [ ] CLI args support `--type` / `-t` flag (multiple values)
- [ ] Site filtering logic implemented in scanner/engine
- [ ] `SiteInfo` structure includes site type
- [ ] Comprehensive tests for all components
- [ ] Documentation updated (SITES.md, ARCHITECTURE.md)
- [ ] CHANGELOG entry added

## 🛠️ Implementation Outline
1. Create/switch to branch `feat/site-type-filtering`
2. Add `SiteType` enum in `src/data/site_info.rs`:
   - Define all categories (dev, social, nsfw, professional, gaming, forum, other)
   - Add helper methods (from_str, as_str, all)
   - Add serde support for serialization
3. Extend `Site` trait in `src/sites/site.rs`:
   - Add `site_type()` method returning `SiteType`
4. Update CLI args in `src/cli/args.rs`:
   - Add `--type` / `-t` flag accepting multiple values
   - Add helper method to parse site types
5. Update `SiteInfo` in `src/data/site_info.rs`:
   - Add `site_type` field
6. Implement filtering in `src/core/scanner.rs`:
   - Add function to filter sites by type
7. Add comprehensive tests
8. Update documentation
9. Move this file to `in_progress/` then `done/`
10. Create PR referencing this issue

## 🔍 Alternatives Considered
- Single category per site → Rejected: Sites might fit multiple categories, but one primary type is simpler
- Tags system → Considered: More flexible but more complex, can add later if needed
- No filtering → Rejected: Users need this feature for practical use

## ⚠️ Risks / Mitigations
- Breaking changes to Site trait → Mitigation: Add default implementation or make it required for new sites
- Type ambiguity → Mitigation: Clear documentation on category definitions
- Performance impact → Mitigation: Filtering is O(n) which is acceptable

## 🔗 Discussion Notes
Site type filtering is essential for user experience. Categories should be well-defined and extensible. Future enhancement could add tags for multi-category support.

