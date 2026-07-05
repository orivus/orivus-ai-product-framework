# Orivus AI Product Framework — Appendices

| Field | Value |
|-------|-------|
| Version | 0.1 |
| Status | Experimental — Frozen |

The following appendices are **informative only**.

They add **no rules**.

They help teams adopt the standard consistently across different products, repositories, and AI engineering environments.

Everything in this document is **optional**.

---

## Appendix A — Recommended Repository Structure

The framework intentionally does not prescribe a repository layout.

A recommended structure may look like:

```
product/
├── docs/
│   ├── discovery/
│   ├── architecture/
│   ├── contracts/
│   ├── intentions/
│   ├── product-memory/
│   └── releases/
│
├── src/
├── tests/
├── tools/
└── README.md
```

Teams are free to organize repositories differently provided Product Knowledge remains governed and discoverable.

---

## Appendix B — Recommended Product Knowledge

The framework governs Product Knowledge rather than documentation.

Typical Product Knowledge may include:

### Product Discovery

- Product Vision
- Product Boundaries
- Product Capabilities
- Domain Model
- Product Contracts
- Runtime Architecture
- Quality Attributes

### Product Delivery

- Intentions
- Product Outcomes
- Planning Artifacts
- AI Context
- Product Shared Memory
- Release Records

---

## Appendix C — Recommended Planning Artifacts

The framework intentionally avoids prescribing mandatory documentation.

Depending on the complexity of the product, teams may use:

- ADR (Architecture Decision Record)
- RFC (Request for Comments)
- Product Contracts
- API Specifications
- State Machines
- Sequence Diagrams
- Event Models
- Threat Models
- Implementation Plans
- Milestones

The framework governs engineering knowledge rather than document formats.

---

## Appendix D — AI Context Recommendations

AI Context should contain only the knowledge required for the current Intention.

Typical AI Context includes:

- Current Intention
- Product Outcome
- Current Milestone
- Relevant Product Knowledge
- Product Shared Memory
- Applicable Architecture Decisions
- Product Contracts
- Coding Standards
- Repository Scope
- Acceptance Criteria

AI Context should remain minimal, deterministic, and focused.

---

## Appendix E — Product Shared Memory Recommendations

Product Shared Memory should preserve only verified engineering knowledge.

Recommended categories include:

- Architecture Decisions
- Engineering Patterns
- Validated Discoveries
- Technical Constraints
- Runtime Behaviors
- Engineering Conventions
- Accepted Technical Debt
- Known Risks
- Lessons Learned

Product Shared Memory should never become a changelog or conversation history.

Its objective is to improve future engineering work.

---

## Appendix F — Markdown First

The canonical representation of Product Knowledge should be Markdown.

Markdown provides:

- AI readability
- Human readability
- Version control compatibility
- Repository integration
- Semantic search
- Long-term maintainability

Binary formats such as DOCX, PDF, or presentations should be generated from Markdown whenever required.

Markdown remains the source of truth.

---

## Appendix G — AI Engineering Workflow

A typical engineering session follows this sequence:

```
Select Intention
        │
        ▼
Review Product Outcome
        │
        ▼
Generate AI Context
        │
        ▼
Execute Current Milestone
        │
        ▼
AI Validation Engine
        │
        ▼
Milestone Audit
        │
        ▼
Repeat Until Intention Complete
        │
        ▼
Intention Audit
        │
        ▼
Human Review
        │
        ▼
Merge
```

---

## Appendix H — AI Prompting Guidelines

Artificial Intelligence should receive goals, constraints, and governed context rather than implementation freedom.

Recommended prompts should:

- define the current Intention;
- define the expected Product Outcome;
- provide the current AI Context;
- identify the active Milestone;
- specify acceptance criteria;
- request evidence before completion.

Prompting should encourage deterministic engineering rather than unrestricted code generation.

---

## Appendix I — Applicability

Orivus AI Product Framework is technology independent.

It may be applied to:

- Mobile Applications
- Web Applications
- Backend Services
- SDKs
- APIs
- AI Agents
- Embedded Systems
- Edge Computing
- Infrastructure Platforms
- Voice Systems
- Multimodal Systems
- AI Operating Systems

The engineering domain may change.

The governance model remains the same.

---

## Appendix J — Framework Evolution

The framework itself should evolve through the same principles it defines.

Changes to the framework should:

- begin with an Intention;
- define a Product Outcome;
- be planned before implementation;
- preserve architectural consistency;
- produce verifiable evidence;
- undergo human review before adoption.

The Orivus AI Product Framework evolves through governed knowledge, just as the products built with it.

---

## Canonical Philosophy

The Orivus AI Product Framework is founded on one central idea:

> Product Knowledge governs implementation.
>
> Artificial Intelligence accelerates engineering.
>
> Governed knowledge preserves product integrity.

Together, they enable software that can evolve continuously without sacrificing architecture, maintainability, or long-term understanding.

This is the essence of AI-native product engineering.
