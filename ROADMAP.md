# Roadmap

| Field | Value |
|-------|-------|
| Current version | 0.1 |
| Status | Experimental — Frozen |

This roadmap describes how the Orivus AI Product Framework evolves. It is
intentionally evidence-driven: the standard advances through learning from real
product implementations, not through speculative design.

> The standard evolves only through evidence obtained from real product
> implementations.

---

## Guiding Principle

The standard never depends on any product. Products depend on the standard.

Every version increment must preserve the standard's independence from any
specific product, vendor, tool, IDE, or AI model.

---

## v0.1 — Experimental (current)

**Status: Frozen.**

Delivered:

- Standard identity (Manifesto).
- Conceptual model (framework layer).
- Normative specifications (GS, PS, AS, MTP, Framework Validation Protocol).
- First Reference Validation (FV-001 — Sequential Milestone Execution, PASS).
- Open source release artifacts.

v0.1 is frozen. No new specifications, rules, or concepts are added to v0.1.
Friction found in real use is recorded as observations in
[FRAMEWORK_FEEDBACK.md](FRAMEWORK_FEEDBACK.md).

---

## v0.2 — Evidence Gate (next)

v0.2 opens **only** when all of the following hold:

1. Independent reference implementations produce sufficient empirical evidence.
2. Observations in [FRAMEWORK_FEEDBACK.md](FRAMEWORK_FEEDBACK.md) justify
   specific, concrete changes.
3. Each behavioral change ships with a Reference Validation that reaches **PASS**.

### Candidate Reference Validations for v0.2

These are planned properties awaiting evidence before formal inclusion:

| ID | Property | Required before |
|----|----------|-----------------|
| FV-002 | Self-Healing Loop | Any change to the self-healing model |
| FV-003 | Outcome Certification | Any change to the outcome certification model |
| FV-004 | Product Knowledge Synchronization | Any change to the PKS model |
| FV-005 | Dead Code Convergence | Any change to the dead-code / convergence policy |

No candidate is adopted on theoretical merit. Each requires evidence plus a
Reference Validation PASS.

---

## Beyond v0.2 (directional, not committed)

- Operational Conformance Program (currently **preparatory**, not operational).
- Additional reference implementations across independent domains to strengthen
  external validity.
- Adoption tooling and getting-started material (non-normative).

These are directions, not promises. They advance only with evidence.

---

## Versioning and Compatibility

The standard uses semantic-style versioning for a specification:

| Change | Increment | Meaning |
|--------|-----------|---------|
| Editorial / clarity / errata | patch (0.1.x) | No behavioral change. Always compatible. |
| Backward-compatible additions | minor (0.x) | New optional rules or properties. Conforming implementations remain conforming. |
| Breaking changes to normative behavior | major (x.0) | A conforming implementation may need changes to remain conforming. |

### What "breaking the standard" means

A change is **breaking** when a previously conforming product, agent, or process
would no longer conform without modification. Breaking changes:

- require a major version increment;
- require Reference Validation evidence;
- must be documented in [CHANGELOG.md](CHANGELOG.md) with a migration note.

During the experimental phase (0.x), the standard may still change as evidence
accumulates. Stability guarantees strengthen as the standard matures.

---

## How Changes Are Approved

See [GOVERNANCE.md](GOVERNANCE.md). In summary:

```
Observation → Evidence → Reference Validation PASS → Human Approval → Release
```
