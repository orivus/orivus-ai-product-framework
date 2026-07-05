# Orivus AI Product Framework — Product Knowledge Model

| Field | Value |
|-------|-------|
| Version | 0.1 |
| Status | Experimental — Frozen |

The Orivus AI Product Framework treats Product Knowledge as the primary engineering asset.

Software evolves.

Technologies change.

Programming languages become obsolete.

Artificial Intelligence models improve.

Product Knowledge remains.

For this reason, the framework governs knowledge first and implementation second.

Every engineering activity either creates, transforms, consumes, or preserves Product Knowledge.

---

## 1. Knowledge Layers

What knowledge does AI need to work correctly?

The standard organizes knowledge in layers — from permanent governance to temporary execution context:

```
Framework Governance (specifications/)
        │
        ▼
Product Foundation
        │
        ▼
Product Shared Memory
        │
        ▼
Intention Artifacts
        │
        ▼
AI Context
```

| Layer | Nature | Purpose |
|-------|--------|---------|
| Framework Governance | Standard-wide | Normative rules — GS, PS, AS |
| Product Foundation | Stable | Minimum knowledge before implementation |
| Product Shared Memory | Durable | Verified knowledge accumulated over time |
| Intention Artifacts | Intention-scoped | Planning knowledge — ADR, RFC, Implementation Plan, etc. |
| AI Context | Temporary | Governed execution boundary for the agent |

This model differentiates governed AI engineering from unstructured prompting.

Normative definitions: [Product Specification §10](../specifications/PRODUCT_SPECIFICATION.md).

---

## 2. Product Knowledge

Product Knowledge is the complete set of verified information that defines a software product.

It represents the collective understanding of the product accumulated throughout its lifecycle.

Product Knowledge includes:

- product purpose;
- product capabilities;
- architecture;
- contracts;
- quality attributes;
- engineering decisions;
- implementation constraints;
- validated outcomes;
- reusable patterns;
- verified discoveries;
- product memory.

Implementation is always governed by Product Knowledge.

Never the opposite.

---

## 2.1 Engineering Knowledge vs Evidence Knowledge

Product Knowledge splits into two classes with different lifecycle rules.

### Engineering Knowledge (governs forever)

Defines what the product is and how it must be built.

| Artifact | Nature |
|----------|--------|
| Intention | Scope and Product Outcome |
| ADR | Architectural decisions |
| RFC | Technical contracts and behavior |
| Implementation Plan | Milestone execution state at approval time |
| AI Context | Governed execution boundary |

Engineering Knowledge MUST NOT be rewritten because hardening removed a panic, a clone was eliminated, or test counts changed.

If Engineering Knowledge must change, return to Planning — not to evidence sync.

### Evidence Knowledge (state of the product)

Demonstrates what the repository currently guarantees.

| Artifact | Nature |
|----------|--------|
| `audits/` | Milestone and Intention audit evidence |
| Product Shared Memory | Durable verified product state |
| Product Record | Human approval and release history |
| Certification counts | Reproducible verification surface |
| Quality baseline | Enterprise guarantees the repo enforces |

Evidence Knowledge MUST synchronize after every significant change — Intention closure, certification pass, enterprise hardening, or baseline shift.

If evidence reads `74 tests` while the repository runs `81`, evidence has stopped being source of truth.

### Repository Knowledge (how knowledge is represented)

Repository Knowledge is distinct from Product Knowledge.

| Class | Question | Examples |
|-------|----------|----------|
| **Product Knowledge** | What does the product know? | Capabilities, contracts, baseline, decisions |
| **Repository Knowledge** | How is that knowledge represented in the repo? | README indexes, status tables, navigation links, verification counts |

Repository Knowledge MUST synchronize in PKS Stage 4.

Repository Knowledge MUST NOT be mixed into Product Shared Memory or governance artifacts.

---

## 2.2 Product Knowledge Synchronization — Four-Stage Audit

Product Knowledge Synchronization verifies that all knowledge representations remain consistent.

It is an audit, not a documentation exercise.

| Stage | Scope | Action |
|-------|-------|--------|
| **1. Evidence** | `audits/`, certification evidence, validation results, test counts | Synchronize |
| **2. Shared Knowledge** | Product Shared Memory, Product Record, quality baseline | Synchronize |
| **3. Governance** | Intention → ADR → RFC → Plan → AI Context | Verify alignment only |
| **4. Repository** | README, indexes, navigation, links, status tables | Synchronize |

Stage 3 does not correct contradictions. It detects them and routes to Planning or a new Intention.

An Intention closes only when:

- code is certified;
- quality gates are green;
- Human Review is **Approved**;
- all four PKS stages pass;
- the Intention is marked **Closed** in Product Record and repository status tables.

**Approved** and **Closed** are distinct states.

---

## 3. Product Foundation

Product Foundation is the initial state of Product Knowledge.

It represents the minimum verified knowledge required before implementation begins.

A Product Foundation exists when:

- the product purpose is understood;
- product capabilities are identified;
- architectural direction is defined;
- product boundaries are clear;
- implementation can begin with acceptable uncertainty.

Product Foundation is not documentation.

It is a readiness state.

Normative definitions: [Product Specification §10 and §14–§15](../specifications/PRODUCT_SPECIFICATION.md).

---

## 4. Product Shared Memory

Product Shared Memory is the persistent memory of the product.

Its purpose is to preserve verified engineering knowledge that will improve future AI-assisted implementation.

Unlike documentation, Product Shared Memory continuously evolves with the product.

It captures only reusable knowledge.

Examples include:

- verified architectural patterns;
- implementation discoveries;
- validated engineering decisions;
- recurring solutions;
- technical constraints;
- known risks;
- accepted technical debt;
- engineering conventions;
- runtime observations;
- lessons learned.

Product Shared Memory must never contain:

- brainstorming;
- conversations;
- opinions;
- temporary implementation notes;
- duplicated documentation;
- unverified assumptions.

Only verified knowledge becomes Product Shared Memory.

---

## 5. AI Context

AI Context is the governed interface between Product Knowledge and AI execution.

It is not a permanent knowledge repository.

It is a temporary execution context generated for one specific Intention.

Its objective is to provide Artificial Intelligence with only the knowledge required to complete the current work.

Typical AI Context may include:

- current Intention;
- Product Outcome;
- relevant Product Foundation;
- applicable Product Shared Memory;
- implementation constraints;
- architectural decisions;
- affected contracts;
- current milestone;
- acceptance criteria;
- coding conventions;
- repository scope.

AI Context intentionally excludes unrelated product knowledge.

Reducing irrelevant context improves consistency, predictability, and implementation quality.

---

## 6. Product Knowledge Flow

Product Knowledge continuously grows throughout the product lifecycle.

```
Product Discovery
        │
        ▼
Product Foundation
        │
        ▼
Intention
        │
     defines
        ▼
Product Outcome
        │
        ▼
AI Context
        │
        ▼
AI Implementation
        │
        ▼
Validated Knowledge
        │
        ▼
Product Knowledge Synchronization
        │
        ▼
Next Intention
```

Every completed Intention enriches the product.

Future Intentions begin with more knowledge than previous ones.

The product continuously becomes easier for both humans and AI to understand.

---

## 7. Product Knowledge Synchronization

Product Knowledge must remain synchronized with the product.

Whenever an Intention is completed — or whenever certification or enterprise hardening changes product state — verified knowledge MUST pass the four-stage synchronization audit.

Product Knowledge Synchronization is a mandatory audit — not a documentation task.

An Intention is not closed until all four stages pass.

### Four stages

| Stage | Name | Synchronize or verify |
|-------|------|----------------------|
| 1 | Evidence Synchronization | `audits/`, INTENTION-evidence, milestone evidence, certification, validation results, test counts |
| 2 | Shared Knowledge Synchronization | Product Shared Memory, Product Record, quality baseline |
| 3 | Governance Synchronization | Intention → ADR → RFC → Implementation Plan → AI Context — **verify only** |
| 4 | Repository Synchronization | README, audit indexes, docs navigation, links, status tables, verification counts |

Stage 3 detects contradictions; it does not rewrite governance artifacts. Contradictions require Planning or a new Intention.

Engineering artifacts are out of scope for Stages 1, 2, and 4 unless Planning itself changed.

Knowledge synchronization should occur automatically whenever possible.

---

## 8. Knowledge Principles

Product Knowledge follows five fundamental principles.

### Knowledge First

Engineering decisions are based on Product Knowledge before implementation begins.

### Single Source of Truth

Verified knowledge should exist only once.

Duplicated knowledge creates inconsistency.

### Verified Knowledge Only

Only validated engineering knowledge becomes permanent product knowledge.

Ideas remain ideas.

Verified outcomes become knowledge.

### AI Readable

Product Knowledge should remain readable by both humans and Artificial Intelligence.

Open, structured, version-controlled text formats are the preferred representation.

Markdown is the recommended canonical format.

### Continuous Learning

Every completed Intention should leave the product smarter than before through Product Knowledge Synchronization.

A mature product accumulates Product Knowledge, not documentation.

This continuous accumulation of verified knowledge enables AI to become progressively more effective while preserving long-term product integrity.
