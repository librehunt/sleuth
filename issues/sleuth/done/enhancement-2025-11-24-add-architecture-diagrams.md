# Add Architecture Diagrams to ARCHITECTURE.md

**Type:** enhancement  
**Status:** done  
**Branch:** enhancement/add-architecture-diagrams  
**Linked roadmap section:** Documentation

---

## 🧠 Context

The `ARCHITECTURE.md` file currently provides a good overview of the codebase structure and components, but it lacks visual diagrams that would help developers understand:
- The execution flow from CLI entry point to HTTP requests
- How components interact with each other
- The concurrent execution model using Tokio
- The data flow through the system

Adding Mermaid diagrams will make the architecture much more accessible and easier to understand.

## 🎯 Goal

Add comprehensive Mermaid diagrams to `ARCHITECTURE.md` that illustrate:
1. The complete execution flow (flowchart)
2. Component architecture and relationships (graph)
3. Concurrent execution model (sequence diagram)
4. Data flow description

Also update the directory structure to reflect that `http/` has been renamed to `request/`.

## 📏 Success Metrics

- [ ] Flowchart diagram showing execution flow from CLI to HTTP requests
- [ ] Component architecture diagram showing relationships between modules
- [ ] Sequence diagram showing concurrent execution with Tokio
- [ ] Updated directory structure (http/ → request/)
- [ ] Data flow section added
- [ ] All diagrams render correctly in GitHub/Markdown viewers

## 🧩 Acceptance Criteria

- [ ] Flowchart diagram added showing complete execution path
- [ ] Component architecture graph added showing module relationships
- [ ] Sequence diagram added showing concurrent request execution
- [ ] Directory structure updated (http/ → request/)
- [ ] Data flow section added with clear explanation
- [ ] Diagrams use proper Mermaid syntax
- [ ] Diagrams are well-documented with clear labels
- [ ] CHANGELOG entry added

## 🛠️ Implementation Outline

1. Create/switch to branch `enhancement/add-architecture-diagrams`
2. Read current ARCHITECTURE.md
3. Add flowchart diagram for execution flow
4. Add component architecture graph
5. Add sequence diagram for concurrent execution
6. Update directory structure section (http/ → request/)
7. Add data flow section
8. Verify diagrams render correctly
9. Update CHANGELOG.md
10. Move this file to `in_progress/` then `done/`
11. Create PR referencing this issue

## 🔍 Alternatives Considered

- Use PlantUML instead of Mermaid → Rejected: Mermaid has better GitHub support
- Create separate diagram files → Rejected: Better to keep in ARCHITECTURE.md
- Use ASCII art → Rejected: Mermaid is more maintainable and visual

## ⚠️ Risks / Mitigations

- Diagrams may not render in all viewers → Mitigation: Test in GitHub, use standard Mermaid syntax
- Diagrams may become outdated → Mitigation: Keep diagrams simple, update with code changes
- Complexity of diagrams → Mitigation: Break into multiple focused diagrams

## 🔗 Discussion Notes

Mermaid diagrams are widely supported in GitHub, GitLab, and most Markdown viewers. They will significantly improve the documentation's accessibility and help new contributors understand the codebase faster.

