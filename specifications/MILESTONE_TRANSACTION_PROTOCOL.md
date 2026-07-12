# Milestone Transaction Protocol

| Field | Value |
|-------|-------|
| Version | 0.2 |
| Status | Normative |
| Scope | Technology-independent · Model-independent · Platform-independent |

Normative specification defining how a framework-conformant agent MUST execute a single Milestone as an atomic transaction.

This protocol implements execution constraints from [AI Agent Specification](AI_AGENT_SPECIFICATION.md) and [Governance Specification](GOVERNANCE_SPECIFICATION.md).

---

## Normative Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **MAY**, and **OPTIONAL** are interpreted per [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).

---

## 1. Precedence Stack

Each layer MAY assume that all layers above it are already defined.

The agent MUST NOT compensate for a missing upper layer by embedding its rules in a lower layer.

```
Framework (specifications/)
        ↓
Engineering Rules (repository standards, language rules)
        ↓
AI Context (active Intention milestone scope)
        ↓
Implementation (code, tests, evidence)
```

### 1.1 Layer Responsibilities

| Layer | Governs | MUST NOT contain |
|-------|---------|------------------|
| **Framework** | Agent execution model, milestone transactions, audits, VERIFIED, PASS, Human Review | Product-specific scope, language rules |
| **Engineering Rules** | Repository quality, architecture boundaries, validation commands | Milestone scope, product outcome |
| **AI Context** | Active milestone objective, acceptance criteria, constraints | Execution loop teaching, framework philosophy |
| **Implementation** | Code and tests for the locked milestone only | Scope beyond active milestone |

### 1.2 Failure Rule

If the agent requires the Intention or AI Context to explain how the execution loop works, the framework layer has failed.

The Intention MUST say what to build. The framework MUST say how to execute.

---

## 2. Milestone Transaction

A **Milestone Transaction** is the atomic unit of Delivery execution.

Exactly **one** active Milestone MAY be open at any time.

A transaction MUST complete (PASS or governance stop) before the next Milestone transaction begins.

### 2.1 Transaction Phases

Every Milestone transaction MUST follow this sequence in order:

```
LOCK → IMPLEMENT → VERIFY → AUDIT → PASS → UPDATE PLAN → NEXT
```

| Phase | Agent action |
|-------|--------------|
| **LOCK** | Identify the single PENDING milestone. Confirm no other milestone is in progress. Read AI Context for that milestone only. |
| **IMPLEMENT** | Implement deliverables for the locked milestone only. |
| **VERIFY** | Run the AI Validation Engine until the project reaches **VERIFIED**. |
| **AUDIT** | Execute Milestone Audit against acceptance criteria. Verdict: PASS or REJECT only. |
| **PASS** | Record milestone PASS only after audit PASS. |
| **UPDATE PLAN** | Update Implementation Plan Living State and milestone evidence. |
| **NEXT** | Automatically begin the next PENDING milestone transaction. |

The agent MUST NOT skip phases. The agent MUST NOT reorder phases.

### 2.2 Anti-Batching Prohibition

The agent MUST NOT:

- implement multiple milestones in one transaction;
- run a single validation pass covering multiple uncertified milestones;
- create evidence for multiple milestones before each receives individual audit PASS;
- mark multiple milestones PASS without individual audit PASS per milestone;
- advance Living State for milestones not yet audited.

Batching milestones is a framework violation, not an implementation shortcut.

---

## 3. State Definitions

### 3.1 VERIFIED

A project is **VERIFIED** for the locked milestone when:

- build succeeds;
- required tests pass;
- lint and static analysis pass;
- architecture rules are satisfied;
- milestone-scoped evidence is generated.

Milestone Audit MUST NOT execute before VERIFIED.

VERIFIED applies to the **current locked milestone scope**, not to future milestones.

### 3.2 Milestone PASS

**Milestone PASS** means:

- the locked milestone reached VERIFIED;
- Milestone Audit returned PASS;
- evidence documents acceptance criteria and validation output for that milestone;
- Implementation Plan Living State records PASS for that milestone only.

Milestone PASS MUST NOT be confused with:

- Intention completion;
- Human Review approval;
- Product Outcome delivery;
- Release readiness.

### 3.3 Living State

**Living State** is the authoritative operational snapshot in the Implementation Plan and AI Context.

After each milestone PASS, the agent MUST update Living State before starting NEXT.

Living State MUST reflect:

- Intention status;
- current milestone (next PENDING or PAUSED/CLOSED);
- per-milestone status (PENDING, IN PROGRESS, PASS, REJECT).

The agent MUST NOT claim future milestones as PASS in Living State.

### 3.4 Intention PAUSED

An Intention MAY be **PAUSED** by human governance or framework refinement.

When PAUSED:

- no new milestone transactions start;
- partial progress remains recorded;
- resume continues from the first PENDING milestone unless humans direct otherwise.

---

## 4. AI Validation Engine

The **AI Validation Engine** is the mechanical validation pipeline defined by Engineering Rules for the repository.

For each VERIFY phase, the agent MUST run the full required pipeline and capture output suitable for milestone evidence.

Mechanical failures MUST be resolved through Self-Healing before AUDIT.

---

## 5. Milestone Audit

Milestone Audit evaluates the locked milestone against:

- Product Value and Observable Result declared in the Implementation Plan;
- evidence that the promised product value is demonstrable for the product consumer;
- AI Context scope and constraints;
- ADR/RFC constraints for that value.

The primary audit question is: **does the promised product value exist?** — not merely whether technical artifacts were implemented.

Verdicts: **PASS** or **REJECT** only.

On REJECT: remediate the locked milestone and repeat IMPLEMENT → VERIFY → AUDIT. The agent MUST NOT proceed to NEXT.

A Milestone MUST receive REJECT when Product Value is undefined or not demonstrable through declared evidence.

---

## 6. Outcome Certification

**Outcome Certification** executes only after all milestones PASS.

It certifies the complete Intention outcome — typically via `tests/intention/*/certification.rs` or equivalent deterministic entrypoint.

Outcome Certification is Intention-level, not milestone-level.

The agent MUST NOT substitute per-milestone PASS for Outcome Certification.

---

## 7. Human Review

**Human Review** is required for Intention closure and product acceptance decisions the agent MUST NOT make.

Human Review occurs after:

- all milestones PASS;
- Outcome Certification PASS;
- Intention Audit returns **READY FOR HUMAN REVIEW**.

Humans approve Intentions. Humans do not approve individual milestones.

---

## 8. Evidence Requirements

Each milestone evidence file MUST include at minimum:

| Section | Content |
|---------|---------|
| Milestone identity | Identifier and title |
| Product Value | Value promised for this milestone |
| Observable Result | What was observed when value was demonstrated |
| Scope | Implementation Tasks completed in this transaction |
| Evidence | Automated and human verification executed |
| Validation | Commands run and outcome (VERIFIED) |
| Audit verdict | PASS or REJECT with rationale — value demonstrable or not |
| Living State update | Plan fields updated |

Retroactive evidence created after batch implementation MUST NOT support Milestone PASS.

---

## 9. Stop Conditions

The agent MUST stop the milestone loop and escalate when:

- Product Outcome is ambiguous;
- ADR/RFC/AI Context conflict;
- scope must expand;
- architecture or public contracts must change;
- security or governance approval is required;
- Intention status is PAUSED;
- retry policy is exhausted.

When stopped for PAUSED, the agent MUST NOT implement further milestones until resume is authorized.

---

## 10. Conformance

A framework-conformant agent MUST treat this protocol as binding during Delivery.

Violations include: milestone batching, skipping LOCK, marking PASS without audit, teaching the execution loop in AI Context, and inverting the precedence stack.

---

## 11. Framework Validation Before Product Resume

When a product Intention is **PAUSED** for framework execution model refinement, resume MUST follow [Framework Validation Protocol](FRAMEWORK_VALIDATION_PROTOCOL.md):

```
Framework Changed → Framework Validation PASS → Resume Product Intention
```

Resuming product delivery without Framework Version Validated is a protocol violation. See [Framework Validation Protocol](FRAMEWORK_VALIDATION_PROTOCOL.md).
