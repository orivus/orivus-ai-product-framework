# Product Specification

| Field | Value |
|-------|-------|
| Version | 0.2 |
| Status | Normative |
| Scope | Technology-independent · Vendor-independent · Domain-independent |

Normative specification defining what an AI-native product is within the Orivus AI Product Framework.

Any product claiming framework conformance MUST satisfy this specification.

---

## 1. Scope

This specification defines the canonical model of an AI-native product.

It applies to any implementation of the framework — regardless of organization, domain, or technology.

Informative background: [MANIFESTO.md](../MANIFESTO.md) · [framework/INTRODUCTION.md](../framework/INTRODUCTION.md)

---

## 2. Normative Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **MAY**, and **OPTIONAL** are interpreted per [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).

---

## 3. Product Definition

### 3.1 What a Product Is

An AI-native product is a **governed body of knowledge** with:

- defined purpose;
- explicit capabilities and constraints;
- architectural identity;
- canonical contracts;
- controlled delivery history.

A product MUST NOT be defined solely by its repository, codebase, or document collection.

### 3.2 Knowledge Hierarchy

Products MUST organize engineering assets as:

```
Product
    │
    ▼
Knowledge (authoritative)
    │
    ▼
Artifacts (expressions of knowledge)
    │
    ▼
Implementation (conformity to knowledge)
```

Knowledge MUST be authoritative over artifacts.

Knowledge MUST be authoritative over implementation.

Artifacts MUST NOT become more authoritative than the knowledge they express.

---

## 4. Canonical Product Model

Every conformant product MUST contain these dimensions:

```
Product
│
├── Purpose          Why the product exists
├── Capabilities     What the product does
├── Constraints      What the product MUST NOT do
├── Architecture     How the product is structured
├── Contracts        How the product interfaces externally
├── Knowledge        Verified understanding of the product
└── Delivery           How the product is built and changed over time
```

A product missing any dimension is non-conformant.

### 4.1 Purpose

Purpose MUST define why the product exists.

Purpose MUST remain stable during Delivery.

Purpose MUST only change through Product Discovery.

### 4.2 Capabilities

Capabilities MUST define owned responsibilities without overlap with other products in the same ecosystem.

Each capability MUST have exactly one owner.

### 4.3 Constraints

Constraints MUST explicitly define out-of-scope responsibilities.

Constraint violations MUST trigger escalation — not silent implementation.

### 4.4 Architecture

Architecture MUST govern implementation.

Implementation MUST NOT govern architecture.

Architectural changes during Delivery MUST extend Foundation principles — not redefine them without returning to Discovery.

### 4.5 Contracts

Contracts MUST define canonical product interfaces.

Contracts MUST be transport-agnostic and technology-neutral.

Every cross-product interaction MUST use defined contracts.

### 4.6 Knowledge

Knowledge MUST be the primary product asset.

Knowledge MUST be human-readable and AI-readable.

Knowledge MUST be maintained in open, version-controlled formats. Markdown is RECOMMENDED.

### 4.7 Delivery

Delivery MUST occur exclusively through governed Intentions.

Each Intention MUST define a Product Outcome before implementation begins.

---

## 5. Product Discovery

### 5.1 Purpose

Product Discovery MUST eliminate uncertainty before implementation begins.

Discovery MUST produce Product Foundation — a readiness state, not a document checklist.

### 5.2 Phases

```
Business Discovery (OPTIONAL)
        │
        ▼
Engineering Discovery (REQUIRED)
        │
        ▼
Product Foundation Achieved
        │
        ▼
Product State: Foundation
```

### 5.3 Discovery Completion

Discovery is complete when:

- purpose is understood;
- capabilities are identified without overlap;
- constraints are explicit;
- architectural direction is defined;
- contracts are defined;
- quality attributes are established;
- implementation MAY begin with acceptable uncertainty.

### 5.4 Discovery Prohibitions

Production implementation MUST NOT begin during Discovery.

Discovery MUST NOT be repeated per Intention.

---

## 6. Product Foundation

### 6.1 Definition

Product Foundation is the minimum verified knowledge required before implementation.

Product Foundation is a **state** — not documentation.

### 6.2 Requirements

Product Foundation MUST contain verified knowledge across:

- identity (purpose, principles);
- scope (capabilities, constraints);
- structure (domain model, architecture);
- interface (contracts);
- quality (attributes, metrics);
- implementation direction (principles, structure).

### 6.3 Stability

A product MUST have exactly one active Product Foundation.

Product Foundation MUST remain the architectural reference throughout the product lifecycle.

### 6.4 Foundation Change

If an Intention requires redefining Purpose or core Constraints, implementation MUST stop.

Product State MUST return to **Discovery**.

---

## 7. Product Delivery

### 7.1 Model

After Foundation, products MUST be delivered through governed Intentions:

```
Product Foundation (stable)
        │
        ▼
Intention → Completed → Intention → Completed → ...
```

### 7.2 Delivery Outputs

Each completed Intention MUST contribute:

- new or enhanced capabilities;
- verified engineering knowledge;
- updated Product Shared Memory;
- updated Product Record.

### 7.3 Delivery Prohibitions

Delivery MUST NOT silently redefine Product Foundation.

Delivery MUST NOT occur without a defined Product Outcome.

---

## 8. Intentions

### 8.1 Definition

An Intention is the smallest governed unit of product delivery.

It defines a measurable Product Outcome and the knowledge required for AI to implement that outcome without redefining the product.

Each Intention MUST introduce exactly one meaningful product change.

### 8.2 Structure

Every Intention MUST contain:

| Element | Requirement |
|---------|-------------|
| Identity | Unique identifier and name |
| Product Outcome | Measurable capability — the expected result, not an artifact |
| Planning Knowledge | Decisions and constraints |
| Execution State | Milestone progress |
| AI Context | Governed execution boundary |
| Evidence | Verification artifacts |

### 8.3 Lifecycle

```
Created → Outcome Defined → Planned → Under Execution
    → All Milestones PASS → Intention Audit PASS
    → Human Approved → Merged → Completed
```

### 8.4 Concurrency

Only one active Intention per product MUST execute at a time.

Parallel Intentions MAY execute only when explicitly declared independent.

### 8.5 Artifacts

ADRs, RFCs, implementation plans, and AI Context files are artifacts.

They express Intention knowledge.

They MUST NOT be confused with the Intention itself.

### 8.6 Milestones

A **Milestone** is the smallest governable unit of **verifiable product value** toward the Product Outcome of the Intention.

A Milestone is NOT a unit of technical execution. Technical work exists as **Implementation Tasks** within a Milestone.

Each Milestone MUST be a **partial realization** of the Product Outcome:

```
Product Outcome
    → Milestone 1 (+ verifiable value)
    → Milestone 2 (+ verifiable value)
    → Milestone N (+ verifiable value)
    → Product Outcome complete
```

Every Milestone in an Implementation Plan MUST declare:

| Field | Requirement |
|-------|-------------|
| Product Value | New value delivered to the product consumer |
| Consumer | Who experiences the value |
| Observable Result | What can be observed when the value exists |
| Evidence | Automated and human verification when required |
| Implementation Tasks | Technical work to deliver the value |

A Milestone MUST NOT receive PASS unless its defined Product Value is demonstrable for the product consumer.

Normative: [GS-14](GOVERNANCE_SPECIFICATION.md#gs-14--verifiable-product-value-per-milestone).

Template: [templates/IMPLEMENTATION_PLAN_TEMPLATE.md](../templates/IMPLEMENTATION_PLAN_TEMPLATE.md).

---

## 9. Product Outcome

### 9.1 Definition

A Product Outcome defines what MUST be true when an Intention completes.

A Product Outcome MUST NOT describe work — it MUST describe achieved capability.

### 9.2 Requirements

Every Product Outcome MUST be:

| Property | Requirement |
|----------|-------------|
| Measurable | Completion objectively verifiable |
| Bounded | Finite, explicit scope |
| Aligned | Consistent with Product Foundation |
| Independent | Achievable within one Intention |
| Evidence-based | Proof of achievement producible |

### 9.3 Prohibition

Implementation MUST NOT begin without a defined Product Outcome.

---

## 10. Product Knowledge Layers

### 10.1 Layers

| Layer | Phase | Persistence |
|-------|-------|-------------|
| Foundation Knowledge | Discovery | Stable reference |
| Delivery Knowledge | Per Intention | Intention-scoped |
| Operational Knowledge | Post-Intention | Product Shared Memory |
| Execution Knowledge | Per Milestone | AI Context |

### 10.2 Product Shared Memory

Product Shared Memory MUST preserve only verified engineering knowledge.

Product Shared Memory MUST NOT contain:

- brainstorming;
- opinions;
- interaction history;
- temporary notes;
- unverified assumptions.

### 10.3 AI Context

AI Context MUST contain only knowledge required for the current work unit.

AI Context MUST be regenerated per Milestone.

Agents MUST NOT execute without AI Context. See [AI Agent Specification](AI_AGENT_SPECIFICATION.md).

### 10.4 Synchronization

Knowledge MUST remain synchronized with implementation.

Desynchronized knowledge is non-conformant.

---

## 11. Product Contracts

### 11.1 Requirements

Contracts MUST define:

- canonical inputs;
- canonical outputs;
- identity guarantees (if applicable);
- quality guarantees (if applicable).

### 11.2 Properties

Contracts MUST be:

- transport-agnostic;
- engine-independent;
- technology-neutral.

### 11.3 Invariants

| ID | Rule |
|----|------|
| PC-1 | Cross-product interactions MUST use defined contracts |
| PC-2 | Contracts MUST be defined before implementation |
| PC-3 | Breaking changes MUST follow governance |
| PC-4 | Implementation MUST conform to contracts |

---

## 12. Validation

### 12.1 Levels

```
Knowledge Validation → Implementation Validation (VERIFIED)
    → Milestone Validation (product value demonstrable — GS-14)
    → Outcome Validation → Human Validation
```

Each level MUST pass before the next begins.

### 12.2 VERIFIED State

A project is VERIFIED when:

- build succeeds;
- required tests pass;
- lint and static analysis pass;
- architecture rules are satisfied;
- evidence is generated.

### 12.3 Audits

Milestone Audit verdict: `PASS` or `REJECT`.

Intention Audit verdict: `READY FOR HUMAN REVIEW` or `REJECT`.

Human approval MUST occur once per Intention — not per Milestone.

---

## 13. Product Record

### 13.1 Definition

Every product MUST maintain a Product Record — the authoritative state and history.

### 13.2 Contents

Product Record MUST include:

- product identity;
- current Product State;
- Foundation reference;
- delivery history;
- Shared Memory reference;
- validation status;
- release history (if applicable).

### 13.3 Updates

Product Record MUST be updated at every significant lifecycle event.

---

## 14. Product State

Product State is the authoritative answer to: **Where is the product now?**

Every agent and human operator MUST read Product State before any engineering action.

Product State is knowledge — not a document. It MUST be recorded in the Product Record.

### 14.1 Requirement

Every product MUST exist in exactly one state at any time.

### 14.2 States

| State | Meaning |
|-------|---------|
| **Discovery** | Product knowledge is being established — no production implementation |
| **Foundation** | Product Foundation achieved — delivery may begin |
| **Delivery** | Active Intention executing |
| **Release Candidate** | Intention Audit passed — awaiting human approval |
| **Released** | Formally released |
| **Deprecated** | End of life |

### 14.3 State Machine

```
Discovery → Foundation → Delivery → Release Candidate
    → Foundation / Released → Deprecated
```

Foundation redefinition MUST return state to **Discovery**.

### 14.4 Agent Behavior

Agents MUST read Product State before any action.

Permitted behavior MUST conform to Product State. See [AI Agent Specification](AI_AGENT_SPECIFICATION.md).

---

## 15. Completion Requirements

Completion is defined by **knowledge readiness** — not by document existence.

### 15.1 Discovery Complete

Discovery is complete when:

- Product Purpose is understood;
- Product Boundaries are defined;
- Product Capabilities are identified;
- Architecture direction is sufficient;
- Product Foundation is achieved.

When Discovery is complete, Product State MUST transition to **Foundation**.

### 15.2 Delivery Ready

Delivery MAY begin only when:

- Product Outcome exists;
- AI Context exists;
- Planning is sufficient for safe execution;
- Acceptance Criteria exist.

Product State MUST be **Foundation** or **Delivery** as appropriate.

### 15.3 Milestone Complete

A Milestone is complete when:

- Project is VERIFIED;
- the Product Value defined in the Implementation Plan is demonstrable for the product consumer;
- Milestone Audit receives PASS;
- Product Knowledge Synchronization is performed when verified knowledge was produced.

### 15.4 Intention Complete

An Intention is complete when:

- all Milestones are complete;
- Intention Audit receives PASS;
- Product State is **Release Candidate** pending human approval.

---

## 16. Conformance

A product is **Product Specification conformant** when it satisfies all MUST requirements in this document.

Self-declaration of conformance is permitted.

**Conformance Program — Status: Preparatory.** Framework implementations currently perform self-assessment against the normative specifications.

---

## 17. Informative References

- [MANIFESTO.md](../MANIFESTO.md) — standard identity
- [Framework Introduction](../framework/INTRODUCTION.md) — conceptual model
- [Governance Specification](GOVERNANCE_SPECIFICATION.md)
- [Governance Rules](../framework/GOVERNANCE_RULES.md) (informative)
- [Product Knowledge Model](../framework/PRODUCT_KNOWLEDGE_MODEL.md) (informative — normative: §10 and §14–§15 of this document)
