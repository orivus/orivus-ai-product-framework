# Orivus AI Product Framework — Core Principles

| Field | Value |
|-------|-------|
| Version | 0.1 |
| Status | Experimental — Frozen |

The Orivus AI Product Framework is built upon a small set of immutable principles.

These principles define how AI-native products should be engineered regardless of technology, programming language, development methodology, or AI model.

Every practice, workflow, and governance rule defined by this standard derives from these principles.

**These principles should not change between minor versions.**

### Summary

| Principle | ID |
|-----------|-----|
| Product Knowledge over Code | CP-3 |
| Architecture over Implementation | CP-4 |
| AI Requires Governed Context | CP-6 |
| Humans Govern Products · AI Governs Execution | CP-2 |
| Verified Knowledge over Conversations | CP-7 · CP-8 |
| Markdown First | CP-9 |
| Product Convergence | CP-11 |

---

## CP-1 — Product Before Implementation

A product must be understood before it is implemented.

Implementation begins only after sufficient product knowledge exists.

Software should never define the product.

The product defines the software.

---

## CP-2 — AI First, Human Governed

Artificial Intelligence performs engineering work.

Humans govern engineering decisions.

AI may discover, analyze, plan, implement, validate, and audit.

Humans remain responsible for product direction, architectural authority, and approval of completed product capabilities.

---

## CP-3 — Knowledge Governs Code

Code is not the primary asset of a software product.

Product Knowledge is.

Architecture, contracts, constraints, patterns, and verified decisions govern implementation.

Code is the result of governed knowledge.

Never its replacement.

> **Product Knowledge governs implementation.**

---

## CP-4 — Architecture Governs Implementation

Implementation must always conform to the architecture.

Implementation must never redefine architecture without returning to planning.

Architectural integrity always has higher priority than implementation convenience.

---

## CP-5 — Build Through Intentions

Products are delivered through governed Intentions.

An Intention is the smallest governed unit of product delivery. It defines a measurable Product Outcome and the knowledge required for AI to implement that outcome without redefining the product.

Each Intention introduces one meaningful product change.

Intentions must define the expected Product Outcome before implementation begins.

Large changes should be divided into smaller Intentions rather than increasing implementation complexity.

---

## CP-6 — AI Requires Governed Context

Artificial Intelligence should never operate on unrestricted repository knowledge.

Every implementation must execute using an AI Context that contains only the governed knowledge required for the current Intention.

Reducing irrelevant context improves consistency, predictability, and engineering quality.

---

## CP-7 — Verify Before Progress

No implementation progresses without evidence.

Every Milestone must reach a VERIFIED state before audit.

Every Intention must satisfy its expected Product Outcome before completion.

**Product Outcome Realization (GR-13 / GS-13):** Milestones may certify infrastructure; an Intention closes only when the user-visible capability promised by the Intention exists. Tests and architecture alone are insufficient.

Verified knowledge governs engineering. Conversations do not.

Verification precedes trust.

Evidence precedes approval.

---

## CP-8 — Preserve Product Knowledge

Products accumulate knowledge throughout their lifecycle.

Verified architectural decisions, implementation discoveries, patterns, risks, constraints, and technical learnings should become part of the Product Shared Memory.

Future development should build upon verified knowledge rather than reconstructing past decisions.

---

## CP-9 — Markdown First

Product Knowledge should be maintained in open, machine-readable, version-controlled formats.

Markdown is the canonical representation of governed product knowledge.

Binary formats such as DOCX, PDF, or presentation files are distribution formats, not authoring formats.

Knowledge should remain accessible to both humans and AI throughout the product lifecycle.

---

## CP-10 — Continuous Evolution

A product is never finished.

Each completed Intention strengthens the Product Foundation, enriches the Product Shared Memory, and enables the next evolution of the product.

The objective of the framework is not to deliver isolated implementations.

The objective is to enable continuous, governed evolution through AI-assisted engineering.

---

## CP-11 — Product Convergence

Every Problem Resolution MUST leave the product in a state that is equal to or better than its previous state.

The implementation MUST remove obsolete code, deprecated implementations, superseded documentation, unused tests, stale configuration, and any artifact that no longer contributes to the Product Foundation or current Product Knowledge.

AI-generated software MUST converge toward simplicity over time, not accumulate historical implementations.

> **AI must demonstrate that it resolved a problem without degrading the product — not merely that it wrote code.**

Product Convergence is a philosophy of product evolution, not merely dead-code removal. Each Intention should reduce entropy: fewer superseded paths, clearer Product Knowledge, tighter alignment with Product Foundation.

Reference Validation FV-005 (planned) will certify Dead Code Convergence behavior under this principle.
