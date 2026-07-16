# Orivus AI Product Framework — AI Execution Model

| Field | Value |
|-------|-------|
| Version | 0.1 |
| Status | Experimental — Frozen |

The Orivus AI Product Framework defines how Artificial Intelligence executes engineering work within a governed product lifecycle.

Artificial Intelligence is not treated as a code generator.

It is treated as an engineering execution engine operating within governed product knowledge.

The objective of the AI Execution Model is to maximize implementation autonomy while preserving product integrity.

This model applies to any agent — not a specific tool or vendor.

Normative counterpart: [AI Agent Specification](../specifications/AI_AGENT_SPECIFICATION.md).

> **Product Knowledge governs implementation.**

---

## 1. Cognitive Cycle

Every agent execution follows the same cognitive cycle:

```
Understand → Planning → Execute → Verify → Learn
```

| Phase | Purpose |
|-------|---------|
| **Understand** | Absorb governed Product Knowledge within AI Context |
| **Planning** | Define how to achieve the current Milestone |
| **Execute** | Implement within governed scope |
| **Verify** | Bring the project to VERIFIED through the AI Validation Engine |
| **Learn** | Product Knowledge Synchronization — incorporate verified knowledge into the product |

Verification alone is insufficient. The Learn phase is required.

---

## 2. Execution Principles

Every AI execution follows five principles.

- AI executes one Intention at a time.
- AI receives only governed context.
- AI continuously validates its own work.
- AI produces evidence before requesting approval.
- Humans approve completed capabilities, not intermediate implementation steps.

These principles allow AI to operate autonomously without sacrificing governance.

---

## 3. AI Execution Flow

Every Intention follows the same execution model.

```
Intention
        │
        ▼
Product Outcome
        │
        ▼
Planning
        │
        ▼
AI Context
        │
        ▼
AI Execution
        │
        ▼
AI Validation Engine
        │
        ▼
VERIFIED
        │
        ▼
Milestone Audit → PASS → Next Milestone (repeat)
        │
        ▼
Intention Audit
        │
        ▼
READY FOR HUMAN REVIEW
        │
        ▼
Human Approval → Merge
        │
        ▼
Product Knowledge Synchronization
```

See [Canonical Workflow](CANONICAL_WORKFLOW.md) for the complete flow including Self-Healing loops.

---

## 4. AI Context Generation

Before implementation begins, the framework generates an AI Context.

AI Context is a temporary execution package created from Product Knowledge.

It contains only the governed information required for the current Intention.

Its objective is to maximize implementation quality while minimizing unnecessary context.

Every Milestone may generate a different AI Context.

---

## 5. AI Execution

AI executes only the current Milestone.

Execution should remain focused on one engineering objective.

AI must not:

- expand the scope;
- redefine architecture;
- modify unrelated modules;
- introduce undocumented dependencies;
- change public contracts without returning to Planning.

Execution is constrained by the AI Context.

---

## 6. Milestone Execution Loop

> **Normative counterpart:** [Milestone Transaction Protocol](../specifications/MILESTONE_TRANSACTION_PROTOCOL.md) — LOCK → IMPLEMENT → VERIFY → AUDIT → PASS → UPDATE PLAN → NEXT. Exactly one milestone per transaction; batching is prohibited.

Every Milestone MUST be executed through the Milestone Execution Loop.

A Milestone is the smallest governable unit of **verifiable product value** toward the Product Outcome — not a unit of technical execution. See [GS-14](../specifications/GOVERNANCE_SPECIFICATION.md#gs-14--verifiable-product-value-per-milestone).

A Milestone is not complete when code is changed.

A Milestone is complete only when:

- implementation is complete;
- the Product Value defined in the Implementation Plan is demonstrable for the product consumer;
- validation reaches VERIFIED;
- evidence is produced;
- Milestone Audit returns PASS;
- the Implementation Plan state is updated.

Milestones are autonomous execution units governed by validation and audit, not by human approval.

The loop is:

```
Milestone
        │
        ▼
AI Build
        │
        ▼
AI Validation Engine
        │
        ▼
VERIFIED?
├── NO → Self-Healing → AI Validation Engine
└── YES → Milestone Audit
              │
              ▼
          PASS?
          ├── NO → Remediation → AI Validation Engine
          └── YES → Update Implementation Plan → Next Milestone
```

The agent MUST NOT ask for human approval between Milestones.

If a Milestone receives PASS, the agent MUST automatically continue to the next Milestone.

If a Milestone receives REJECT, the agent MUST remain on the same Milestone, remediate the findings, rerun validation, and rerun the Milestone Audit until PASS or until a governance blocker is detected.

---

## 7. AI Validation Engine

After implementation, AI immediately validates its own work.

The AI Validation Engine is responsible for bringing the project into a VERIFIED state.

It is not an audit.

It is an autonomous validation and correction engine.

Its objective is to eliminate mechanical failures before governance begins.

---

## 8. VERIFIED State

A project reaches VERIFIED when:

- the project builds successfully;
- required tests pass;
- lint and static analysis pass;
- blocking errors are resolved;
- architecture rules are satisfied;
- required evidence has been generated.

Milestone Audit must never execute before the project reaches VERIFIED.

---

## 9. Self-Healing Loop

Whenever verification fails, AI enters a correction loop.

```
Failure
    │
    ▼
Analyze Diagnostics
    │
    ▼
Identify Root Cause
    │
    ▼
Apply Fix
    │
    ▼
Rebuild
    │
    ▼
Revalidate
    │
    ▼
VERIFIED?
```

This loop continues until one of two conditions occurs:

- VERIFIED is achieved.
- Retry policy is exhausted.

Mechanical failures should never require human intervention.

Examples include:

- compilation errors;
- missing imports;
- failing tests;
- lint violations;
- formatting issues;
- dependency resolution;
- type errors.

If validation identifies architectural conflicts, contract violations, or Product Foundation inconsistencies, execution must stop and return to Planning.

---

## 10. Product Knowledge Synchronization

After Human Approval and merge, AI synchronizes newly verified knowledge.

Product Knowledge Synchronization is a process — not a document.

Only verified knowledge may be incorporated.

Typical synchronization includes:

- Product Shared Memory — architectural discoveries, patterns, conventions, risks;
- Product Record — state and delivery history;
- Documentation — verified product knowledge;
- Architecture Notes — structural decisions and constraints.

Temporary implementation details must never become permanent knowledge.

Documentation reflects verified product knowledge.

Documentation should never become the source of truth.

Product Knowledge remains the authoritative source.

---

## 11. Milestone Audit

Once the project is VERIFIED, AI executes the Milestone Audit.

Milestone Audit evaluates one engineering objective.

It verifies:

- milestone Product Value and Observable Result;
- whether the promised product value is demonstrable;
- acceptance criteria;
- architecture compliance;
- ADR compliance;
- RFC compliance;
- AI Context compliance;
- scope compliance;
- implementation quality;
- evidence completeness.

The audit produces only one verdict.

**PASS**

or

**REJECT**

If the verdict is REJECT, AI returns to remediation within the same Milestone and re-enters the Milestone Execution Loop at the AI Validation Engine.

The next Milestone must never begin until the current Milestone receives PASS.

---

## 12. Intention Audit

After every Milestone reaches PASS, AI executes the Intention Audit.

Unlike Milestone Audit, Intention Audit evaluates the complete capability.

It verifies:

- Product Outcome;
- Product Foundation alignment;
- architecture integrity;
- ADR compliance;
- RFC compliance;
- Implementation Plan completion;
- technical debt;
- evidence completeness;
- production readiness.

The Intention Audit produces one of two results:

**READY FOR HUMAN REVIEW**

or

**REJECT**

If REJECT is returned, AI creates remediation work and resumes execution.

---

## 13. Human Review

Human approval occurs only once.

The human reviews the completed Intention rather than individual Milestones.

The review focuses on:

- product value;
- architectural integrity;
- engineering quality;
- business impact;
- readiness for merge.

If approved:

```
READY FOR HUMAN REVIEW
        │
        ▼
Human Approval
        │
        ▼
Merge
        │
        ▼
Product Knowledge Synchronization
```

The human governs the product.

The AI governs engineering execution.

---

## 14. Execution Philosophy

The AI Execution Model intentionally separates execution from governance.

Artificial Intelligence should spend its time implementing, validating, correcting, and producing evidence.

Humans should spend their time making product and architectural decisions.

This separation allows AI to execute continuously for extended periods while preserving engineering quality through objective validation, governed knowledge, and evidence-based audits.

The result is an engineering workflow where autonomy increases without reducing governance, enabling AI-native product development at scale.

> **Product Knowledge governs implementation.**
