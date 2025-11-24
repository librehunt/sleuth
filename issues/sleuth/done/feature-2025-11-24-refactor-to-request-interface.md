# Refactor HTTP to Request Interface with End-to-End Search

**Type:** feature  
**Status:** done  
**Branch:** feat/request-interface  
**Linked roadmap section:** Core Infrastructure

---

## 🧠 Context
Currently, the codebase has an `http/` directory with placeholder implementations. We need to:
1. Refactor to a `request/` interface pattern that allows swapping implementations (HTTP, Tor, etc.)
2. Separate concerns: Sites build URLs and parse responses, Scanner executes requests
3. Implement end-to-end search functionality with GitHub as example
4. Make sites pure configuration (no HTTP client dependencies)

This architecture will make it easy to add Tor support later and improve testability.

## 🎯 Goal
Implement a complete request interface architecture that:
- Renames `http/` to `request/` with Request trait interface
- Separates site logic (URL building, response parsing) from request execution
- Implements HTTP request handler
- Adds Tor placeholder for future implementation
- Updates Site trait to remove async check_username, add build_url/parse_response
- Implements scanner with parameter order: username, sites, request (default HTTP)
- Implements end-to-end search with GitHub example
- Updates CLI and main.rs to wire everything together

## 📏 Success Metrics
- [ ] Request trait interface implemented
- [ ] HTTP implementation working
- [ ] Tor placeholder created
- [ ] Site trait refactored (no async, no HTTP client)
- [ ] Scanner can search across sites concurrently
- [ ] End-to-end GitHub search works
- [ ] All tests passing
- [ ] CLI can run: `sleuth github --site github`

## 🧩 Acceptance Criteria
- [ ] `request/` directory with Request trait
- [ ] `HttpRequest` implementation using reqwest
- [ ] `TorRequest` placeholder (returns error for now)
- [ ] Site trait refactored: build_url(), parse_response(), http_method()
- [ ] Scanner function: `scan_username(username, sites, request)` with default HTTP
- [ ] GitHub checker simplified (no HTTP client)
- [ ] Engine/CLI integration working
- [ ] End-to-end test: search GitHub username
- [ ] All existing tests updated/passing
- [ ] Documentation updated
- [ ] CHANGELOG entry added

## 🛠️ Implementation Outline
1. Create/switch to branch `feat/request-interface`
2. Rename `src/http/` to `src/request/`
3. Create Request trait in `src/request/trait_impl.rs`:
   - Define Request trait with head(), get(), request() methods
   - Define RequestResponse struct
4. Implement HttpRequest in `src/request/http.rs`:
   - Wrap reqwest Client
   - Implement Request trait
5. Create TorRequest placeholder in `src/request/tor.rs`
6. Update request/mod.rs with factory function
7. Refactor Site trait in `src/sites/site.rs`:
   - Remove async check_username()
   - Add build_url(), parse_response(), http_method()
8. Update GitHub checker to use new Site trait
9. Implement scanner function in `src/core/scanner.rs`:
   - Parameter order: username, sites, request (Option<Arc<dyn Request>>)
   - Default to HTTP if None
   - Use Arc for sharing across tasks
10. Implement engine in `src/core/engine.rs`
11. Update CLI output formatting
12. Wire main.rs to use engine
13. Update all module exports
14. Add comprehensive tests
15. Update documentation
16. Move this file to `in_progress/` then `done/`
17. Create PR referencing this issue

## 🔍 Alternatives Considered
- Keep HTTP in sites → Rejected: Hard to swap implementations, sites become complex
- Use Box instead of Arc → Rejected: Arc needed for sharing across async tasks
- Keep async in Site trait → Rejected: Separates concerns better without it

## ⚠️ Risks / Mitigations
- Breaking changes to Site trait → Mitigation: Update all site implementations
- Arc complexity → Mitigation: Use Arc::clone() pattern, well-documented
- Request trait design → Mitigation: Keep interface simple, extensible

## 🔗 Discussion Notes
This refactoring enables:
- Easy Tor integration later
- Better testability (mock Request trait)
- Cleaner separation of concerns
- Better parallelization (scanner handles all requests)

