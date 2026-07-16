# Governance Specification

| Field | Value |
|-------|-------|
| Version | 0.2 |
| Status | Normative |
| Scope | Product implementations · Agent implementations · Full framework implementations |

This specification defines the mandatory governance requirements for any implementation of the Orivus AI Product Framework.

Governance requirements apply to products, agents, and the engineering process that connects them.

Whenever a governance requirement conflicts with implementation convenience, the governance requirement SHALL prevail.

Informative background: [framework/Governance Rules](../framework/GOVERNANCE_RULES.md)

---

## 1. Scope

This specification defines operational constraints that preserve product integrity during AI-assisted engineering.

It applies to:

- any framework-conformant product;
- any framework-conformant agent;
- any organization adopting the framework.

Core Principles ([framework/Core Principles](../framework/CORE_PRINCIPLES.md)) describe philosophy.

This specification defines enforceable requirements.

---

## 2. Normative Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **MAY**, and **OPTIONAL** are interpreted per [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).

---

## 3. Governance Requirements

### GS-1 — Product Discovery First

Any framework-conformant implementation MUST establish a Product Foundation before production implementation begins.

An implementation MUST NOT generate production code before sufficient Product Knowledge exists to define the product's purpose, boundaries, architecture, and expected outcomes.

Product Discovery MUST be completed before Product Delivery begins.

If an active Intention requires redefining Product Foundation, implementation MUST stop and Product Discovery MUST resume.

---

### GS-2 — One Intention at a Time

Any framework-conformant implementation MUST execute only one active Intention at a time.

Parallel implementation MAY occur only when Intentions are explicitly declared independent.

Undeclared parallel Intentions are non-conformant.

---

### GS-3 — Every Intention Defines a Product Outcome

Every Intention MUST define its expected Product Outcome before implementation begins.

An implementation MUST NOT proceed without a measurable Product Outcome.

Implementation without a defined outcome is non-conformant.

---

### GS-4 — Planning Before Build

Every Intention MUST be planned before implementation.

Planning MUST eliminate sufficient uncertainty for safe agent execution.

This specification does NOT prescribe specific planning artifacts.

Planning MUST produce governed Product Knowledge sufficient for execution.

Only governed knowledge is REQUIRED — not specific document formats.

---

### GS-5 — AI Context Is Mandatory

Every implementation MUST execute using an AI Context generated from governed Product Knowledge.

An agent MUST NOT implement directly from repository state or interaction history alone.

AI Context MUST be assembled before each Milestone execution.

Unscoped execution is non-conformant.

---

### GS-6 — Verify Before Audit

A Milestone Audit MUST NOT execute unless the project has reached VERIFIED state through the AI Validation Engine.

Verification MUST precede governance.

Audit MUST NOT precede verification.

---

### GS-7 — Every Milestone Produces Evidence

Every Milestone MUST generate sufficient evidence to support objective verification.

Evidence MUST be reproducible.

A Milestone without evidence MUST NOT receive PASS.

Evidence absence MUST result in audit failure.

---

### GS-8 — Audit Before Progress

A Milestone MUST NOT progress to the next Milestone until the current Milestone has reached VERIFIED and received a PASS verdict from Milestone Audit.

If the current Milestone receives REJECT, the implementation MUST remain within the same Milestone until remediation succeeds or a governance blocker is detected.

Human approval MUST NOT be required between Milestones.

Human approval occurs only after the complete Intention is ready for review.

An Intention MUST NOT proceed to Human Review until Intention Audit receives PASS.

Milestones are autonomous execution units governed by validation and audit, not by human approval.

Governance MUST progress through verified states.

Skipping audit states is non-conformant.

---

### GS-9 — Humans Approve Intentions, Not Milestones

Agents govern implementation execution.

Humans govern the product.

Human approval MUST occur only after an Intention has successfully completed Intention Audit.

Human approval MUST mark an Intention as **Approved**.

An Intention MUST NOT be marked **Closed** until Product Knowledge Synchronization completes.

Human approval MUST NOT be required for intermediate Milestones.

Requesting human approval per Milestone is non-conformant.

---

### GS-10 — Preserve Product Knowledge

Every completed Intention MUST preserve reusable engineering knowledge through Product Knowledge Synchronization.

An Intention MUST NOT reach **Closed** until Product Knowledge Synchronization completes as a four-stage audit.

**Approved** records human acceptance of the Intention outcome.

**Closed** records that the product's knowledge representations are synchronized.

Approval without completed PKS MUST NOT mark an Intention as Closed.

Product Knowledge Synchronization is an audit of consistency — not a documentation exercise.

| Stage | Scope | Action |
|-------|-------|--------|
| 1 — Evidence | `audits/`, INTENTION-evidence, milestone evidence, certification, validation results, test counts | Synchronize |
| 2 — Shared Knowledge | Product Shared Memory, Product Record, quality baseline | Synchronize |
| 3 — Governance | Intention → ADR → RFC → Implementation Plan → AI Context | Verify alignment only |
| 4 — Repository | README, audit indexes, docs navigation, links, status tables | Synchronize |

Stage 3 MUST NOT rewrite governance artifacts to fix contradictions. Contradictions MUST route to Planning or a new Intention.

Repository Knowledge — how product knowledge is represented in the repository — MUST synchronize in Stage 4 and MUST NOT be conflated with Product Shared Memory.

Verified knowledge MUST be synchronized into Product Shared Memory and Product Record when it improves future engineering work.

Product Knowledge is a permanent product asset.

Discarding verified knowledge without synchronization is non-conformant.

Unverified knowledge MUST NOT enter Product Shared Memory.

---

### GS-11 — Markdown First

Governed Product Knowledge MUST be maintained in open, version-controlled, AI-readable formats.

Markdown is the RECOMMENDED canonical authoring format.

Binary formats — including DOCX, PDF, and presentation files — MAY be used as distribution formats.

Binary formats MUST NOT become the primary source of truth for governed Product Knowledge.

---

### GS-12 — Architecture Has Final Authority

Architecture MUST govern implementation.

Implementation MUST NOT govern architecture.

If implementation requires changing architectural authority, execution MUST stop.

Execution MUST return to Planning or Product Discovery as appropriate.

An implementation MUST NOT silently redefine the product.

Architectural integrity MUST take priority over implementation convenience.

---

### GS-13 — Product Outcome Realization

An Intention MUST NOT be marked **READY FOR HUMAN REVIEW**, **Approved**, or **Closed** unless its defined Product Outcome is **observable as user-visible product value**.

Milestone PASS verdicts certify milestone scope only. They MUST NOT be interpreted as Intention completion when the Product Outcome is still absent.

Intention Audit MUST verify Product Outcome realization explicitly. Intention Audit MUST receive **REJECT** when the promised value is missing, regardless of Milestone PASS count or test coverage.

Implementation without eventual user-visible Outcome realization is non-conformant for Intention closure.

Per-milestone value requirements: [GS-14](#gs-14--verifiable-product-value-per-milestone).

---

### GS-14 — Verifiable Product Value per Milestone

A **Milestone** is the smallest governable unit of **verifiable product value** toward the Product Outcome of the Intention.

Every Milestone in an Implementation Plan MUST represent a new increment of verifiable product value — a partial realization of the Product Outcome, not an independent technical task.

Each Milestone MUST declare, before execution begins:

| Field | Requirement |
|-------|-------------|
| **Product Value** | What new value the product delivers when the milestone completes |
| **Consumer** | Who experiences that value |
| **Observable Result** | What can be observed when the value exists |
| **Evidence** | How the value is demonstrated — automated and human when required |
| **Implementation Tasks** | Technical work required to deliver the value |

**Implementation Tasks** MUST NOT be promoted to Milestones. Internal implementation artifacts MAY exist only as tasks required to deliver the Milestone's Product Value.

A Milestone MUST NOT receive PASS unless the **Product Value** defined for that Milestone is demonstrable for the product consumer through the evidence declared in the Implementation Plan.

Milestone Audit MUST evaluate whether the promised product value exists — not whether technical artifacts were implemented.

A Milestone defined only by technical deliverables (layers, modules, boundaries, packaging) without Product Value and Observable Result is non-conformant.

Milestone Audit MUST receive **REJECT** when:

- Product Value is absent or undefined;
- Observable Result is absent or undefined;
- the promised value is not demonstrable through declared evidence;
- only internal implementation artifacts were delivered.

Template: [templates/IMPLEMENTATION_PLAN_TEMPLATE.md](../templates/IMPLEMENTATION_PLAN_TEMPLATE.md).

---

## 4. Governance and Product State

Governance requirements MUST be enforced according to Product State.

| Product State | Governance Focus |
|---------------|-----------------|
| **Discovery** | GS-1 — Foundation establishment |
| **Foundation** | GS-3, GS-4 — Intention and Outcome definition |
| **Delivery** | GS-2, GS-5 through GS-8 — Execution governance |
| **Release Candidate** | GS-9 — Human approval gate |
| **Released** | GS-10, GS-12 — Knowledge and architecture preservation |

Product State requirements: [Product Specification §14](PRODUCT_SPECIFICATION.md#14-product-state)

---

## 5. Governance and Agents

Agents operating on framework-conformant products MUST satisfy all applicable governance requirements.

Agent-specific requirements: [AI Agent Specification](AI_AGENT_SPECIFICATION.md)

Governance requirements GS-5, GS-6, GS-8, and GS-9 directly constrain agent behavior.

---

## 6. Conflict Resolution

When governance requirements conflict with:

| Conflict | Resolution |
|----------|------------|
| Implementation convenience | Governance requirement prevails |
| Agent autonomy scope | Governance boundary prevails |
| Undocumented practice | Governance requirement prevails |
| Informative framework guidance | Normative specification prevails |

Informative documents in [framework/](../framework/README.md) MUST NOT override this specification.

---

## 7. Requirement Index

| ID | Requirement | Applies To |
|----|-------------|------------|
| GS-1 | Product Discovery First | Product · Process |
| GS-2 | One Intention at a Time | Product · Agent |
| GS-3 | Product Outcome Required | Product · Intention |
| GS-4 | Planning Before Build | Product · Agent |
| GS-5 | AI Context Mandatory | Agent · Process |
| GS-6 | Verify Before Audit | Agent · Process |
| GS-7 | Milestone Evidence | Agent · Process |
| GS-8 | Audit Before Progress | Agent · Process |
| GS-9 | Human Approves Intentions | Product · Human |
| GS-10 | Preserve Product Knowledge | Product · Agent |
| GS-11 | Markdown First | Product |
| GS-12 | Architecture Authority | Product · Agent |
| GS-13 | Product Outcome Realization | Product · Intention · Process |
| GS-14 | Verifiable Product Value per Milestone | Product · Agent · Process |

---

## 8. Conformance

An implementation is **Governance Specification conformant** when it satisfies all MUST and MUST NOT requirements in this document.

Governance conformance is REQUIRED for full framework conformance.

Governance conformance alone does NOT imply Product or Agent conformance.

Full framework conformance requires all three normative specifications. v0.1: self-assessment against normative specifications. [Conformance Program](CONFORMANCE_PROGRAM.md) is preparatory.

---

## 9. Informative References

- [Core Principles](../framework/CORE_PRINCIPLES.md) — philosophical foundation
- [Governance Rules](../framework/GOVERNANCE_RULES.md) — informative counterpart (GR-1 through GR-14)
- [Canonical Workflow](../framework/CANONICAL_WORKFLOW.md) — operational workflow
- [AI Execution Model](../framework/AI_EXECUTION_MODEL.md) — execution governance model

### GR to GS Mapping

| Informative (GR) | Normative (GS) |
|------------------|----------------|
| GR-1 | GS-1 |
| GR-2 | GS-2 |
| GR-3 | GS-3 |
| GR-4 | GS-4 |
| GR-5 | GS-5 |
| GR-6 | GS-6 |
| GR-7 | GS-7 |
| GR-8 | GS-8 |
| GR-9 | GS-9 |
| GR-10 | GS-10 |
| GR-11 | GS-11 |
| GR-12 | GS-12 |
| GR-13 | GS-13 |
| GR-14 | GS-14 |
