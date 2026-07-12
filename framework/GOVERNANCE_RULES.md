# Orivus AI Product Framework — Governance Rules

| Field | Value |
|-------|-------|
| Version | 0.2 |
| Status | Informative |
| Normative Counterpart | [specifications/GOVERNANCE_SPECIFICATION.md](../specifications/GOVERNANCE_SPECIFICATION.md) |

Governance Rules define the operational boundaries of the Orivus AI Product Framework.

Unlike Core Principles, which describe philosophy, Governance Rules define mandatory constraints that preserve product integrity during AI-assisted engineering.

**For enforceable requirements, see [Governance Specification](../specifications/GOVERNANCE_SPECIFICATION.md) (GS-1 through GS-14).**

These rules are the contract of the standard.

Whenever a Governance Rule conflicts with implementation convenience, the Governance Rule prevails.

### Summary

| Rule | ID |
|------|-----|
| Foundation before Build | GR-1 |
| One Active Intention | GR-2 |
| Product Outcome Required | GR-3 |
| Planning Before Build | GR-4 |
| AI Context Mandatory | GR-5 |
| VERIFIED Before Audit | GR-6 |
| Every Milestone Produces Evidence | GR-7 |
| Audit Before Progress | GR-8 |
| Humans Approve Intentions, Not Milestones | GR-9 |
| Product Knowledge Synchronization Before Closure | GR-10 |
| Markdown Is Source of Truth | GR-11 |
| Architecture Has Authority | GR-12 |
| Product Outcome Realization | GR-13 |
| Verifiable Product Value per Milestone | GR-14 |

---

## GR-1 — Product Discovery First

No implementation may begin before Product Discovery has established a Product Foundation.

Products must be understood before they are implemented.

**Normative:** [GS-1](../specifications/GOVERNANCE_SPECIFICATION.md#gs-1--product-discovery-first)

---

## GR-2 — One Intention at a Time

Artificial Intelligence must execute only one active Intention at a time.

Parallel implementation is permitted only when Intentions are explicitly independent.

**Normative:** [GS-2](../specifications/GOVERNANCE_SPECIFICATION.md#gs-2--one-intention-at-a-time)

---

## GR-3 — Every Intention Defines a Product Outcome

Every Intention must define its expected Product Outcome before implementation begins.

Implementation without a measurable outcome is not permitted.

**Normative:** [GS-3](../specifications/GOVERNANCE_SPECIFICATION.md#gs-3--every-intention-defines-a-product-outcome)

---

## GR-4 — Planning Before Build

Every Intention must be planned before implementation.

Planning must eliminate sufficient uncertainty for AI to execute safely.

The framework does not prescribe specific planning documents.

Only governed product knowledge is required.

**Normative:** [GS-4](../specifications/GOVERNANCE_SPECIFICATION.md#gs-4--plan-before-build)

---

## GR-5 — AI Context Is Mandatory

Every implementation must execute using an AI Context generated from governed Product Knowledge.

AI must never implement directly from repository state or conversational history alone.

**Normative:** [GS-5](../specifications/GOVERNANCE_SPECIFICATION.md#gs-5--ai-context-is-mandatory)

---

## GR-6 — Verify Before Audit

A Milestone Audit must never execute unless the project has reached a VERIFIED state through the AI Validation Engine.

Verification always precedes governance.

**Normative:** [GS-6](../specifications/GOVERNANCE_SPECIFICATION.md#gs-6--verify-before-audit)

---

## GR-7 — Every Milestone Produces Evidence

Every Milestone must generate sufficient evidence to support objective verification.

Evidence must be reproducible.

A Milestone without evidence automatically fails audit.

**Normative:** [GS-7](../specifications/GOVERNANCE_SPECIFICATION.md#gs-7--every-milestone-produces-evidence)

---

## GR-8 — Audit Before Progress

No Milestone may begin until the current Milestone has received a PASS verdict.

Every Intention must receive a PASS Intention Audit before Human Review.

Governance always progresses through verified states.

**Normative:** [GS-8](../specifications/GOVERNANCE_SPECIFICATION.md#gs-8--audit-before-progress)

---

## GR-9 — Humans Approve Intentions, Not Milestones

Artificial Intelligence governs implementation.

Humans govern the product.

Human approval occurs only after an Intention has successfully completed the Intention Audit.

Human approval marks an Intention as **Approved** — a human decision.

An Intention becomes **Closed** only after Product Knowledge Synchronization completes.

Intermediate implementation steps do not require human approval.

**Normative:** [GS-9](../specifications/GOVERNANCE_SPECIFICATION.md#gs-9--humans-approve-intentions-not-milestones)

---

## GR-10 — Product Knowledge Synchronization Before Closure

An Intention MUST NOT reach **Closed** until Product Knowledge Synchronization completes as a four-stage audit.

**Approved** is a human decision. **Closed** is an administrative product state. They are distinct even when they occur on the same day.

| Stage | Scope |
|-------|-------|
| 1 — Evidence | `audits/`, INTENTION-evidence, milestone evidence, certification, validation results, test counts |
| 2 — Shared Knowledge | Product Shared Memory, Product Record, quality baseline |
| 3 — Governance | Intention → ADR → RFC → Plan → AI Context — verify alignment only; contradictions route to Planning |
| 4 — Repository | README, indexes, navigation, links, status tables, verification counts |

Engineering knowledge governs the product and is not rewritten for hygiene work.

Evidence knowledge and Repository Knowledge represent current product state and MUST remain consistent.

**Normative:** [GS-10](../specifications/GOVERNANCE_SPECIFICATION.md#gs-10--preserve-product-knowledge)

---

## GR-11 — Markdown First

Governed Product Knowledge must be maintained in open, version-controlled, AI-readable formats.

Markdown is the canonical authoring format.

Binary formats such as DOCX, PDF, or presentation files are distribution formats and must never become the primary source of truth.

**Normative:** [GS-11](../specifications/GOVERNANCE_SPECIFICATION.md#gs-11--markdown-first)

---

## GR-12 — Architecture Has Final Authority

Architecture governs implementation.

Implementation never governs architecture.

If implementation requires changing architectural authority, execution must stop and return to Planning or Product Discovery.

No implementation may silently redefine the product.

**Normative:** [GS-12](../specifications/GOVERNANCE_SPECIFICATION.md#gs-12--architecture-has-final-authority)

---

## GR-13 — Product Outcome Realization

A Product Intention cannot be considered implemented — and MUST NOT reach Intention Audit, Human Review, or **Closed** — unless its promised **user-visible Product Outcome exists**.

Passing tests, perfect architecture, and complete RFC coverage do **not** substitute for the value the Intention promises.

**Intention Audit MUST FAIL** when the Product Outcome is absent, even if every Milestone in the Implementation Plan has PASS.

Per-milestone value is governed by [GR-14](#gr-14--verifiable-product-value-per-milestone).

**Example:** An Intention named *Real-Time Stock Visibility* does not close until an operator can query current stock through the public API — not when only the domain model and repository layer exist.

**Normative:** [GS-13](../specifications/GOVERNANCE_SPECIFICATION.md#gs-13--product-outcome-realization)

---

## GR-14 — Verifiable Product Value per Milestone

A Milestone is the smallest governable unit of **verifiable product value** toward the Product Outcome — not a unit of technical execution.

Every Milestone MUST declare Product Value, Consumer, Observable Result, Evidence, and Implementation Tasks before execution begins.

A Milestone MUST NOT receive PASS unless the Product Value defined for that Milestone is demonstrable for the product consumer.

Internal implementation work (layers, modules, adapters, packaging) MAY exist only as **Implementation Tasks** within a value-driven Milestone.

Milestone Audit asks: **does the promised product value exist?** — not merely whether technical artifacts were implemented.

**Example:** A Milestone titled *Repository Layer* with no Product Value is non-conformant. A Milestone titled *Operator can query stock* with repository and API as Implementation Tasks is conformant.

**Normative:** [GS-14](../specifications/GOVERNANCE_SPECIFICATION.md#gs-14--verifiable-product-value-per-milestone)
