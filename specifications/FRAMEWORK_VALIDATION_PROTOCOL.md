# Framework Validation Protocol

| Field | Value |
|-------|-------|
| Version | 0.1 |
| Status | Normative — Operational refinement |
| Scope | Framework behavior validation before product resume |

Normative specification defining when and how the Orivus AI Product Framework MUST be validated after operational changes.

A **Reference Validation** demonstrates a **framework property** — not product functionality.

---

## Normative Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **MAY**, and **OPTIONAL** are interpreted per [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).

---

## 1. Principle

**A framework change MUST be validated before governing product development.**

The framework is a product. The standard applies the same discipline to itself that it requires of products.

**Framework Validation does not certify the exercise artifact.** It certifies that the framework can govern agent execution correctly.

---

## 2. Reference Validation

A **Reference Validation** (FV-NNN) is a controlled exercise that demonstrates one **framework property** using minimal non-product scope.

| Concept | Definition |
|---------|------------|
| **Framework property** | A behavioral guarantee the framework claims (e.g. sequential milestone execution) |
| **Exercise artifact** | Incidental code used only as execution substrate — not the validation target |
| **PASS** | Human-approved proof that the framework property held under reproducible evidence |

Reference Validations form the framework's certification suite — analogous to product certification tests.

### 2.1 Planned properties (non-exhaustive)

| ID | Property | Status |
|----|----------|--------|
| FV-001 | Sequential Milestone Execution | **PASS** |
| FV-002 | Value-Driven Milestones | **PASS** |
| FV-003 | Self-Healing Loop | Planned |
| FV-004 | Outcome Certification | Planned |
| FV-005 | Product Knowledge Synchronization | Planned |
| FV-006 | Dead Code Convergence | Planned |

Each framework change that modifies agent behavior MUST incorporate at least one Reference Validation demonstrating the introduced or modified property. A framework change is not validated until its Reference Validation reaches **PASS**.

---

## 3. Validation Gate

When the framework receives operational changes (execution model, milestone protocol, agent rules), the following sequence is REQUIRED before resuming product development on a paused Intention:

```
Framework Changed
        ↓
Reference Validation (FV-NNN)
        ↓
Framework Validation PASS (human approval)
        ↓
Framework Version Validated
        ↓
Resume Product Development
```

The agent MUST NOT lift **PAUSED** on a product Intention until **Framework Version Validated** is recorded for the framework version in effect.

---

## 4. Framework Validation PASS Criteria

Framework Validation PASS MUST answer **only** questions about framework behavior — not exercise functionality.

| Validation | Required result |
|------------|-----------------|
| Agent executed one milestone at a time | Demonstrated |
| No batching | Demonstrated |
| Living State updated incrementally | Demonstrated |
| Each milestone produced its own evidence | Demonstrated |
| Next milestone began automatically | Demonstrated |
| Agent did not request human intervention between milestones | Demonstrated |
| Human Review occurred only at the end | Demonstrated |
| Milestone Transaction Protocol respected completely | Demonstrated |

If any criterion lacks reproducible evidence, Framework Validation FAILS.

The exercise artifact (e.g. a logger interface) MAY be trivial. Its correctness is irrelevant to PASS.

---

## 5. Reference Validation Structure

```
validation/
    FV-001-sequential-milestone-loop/
    FV-002-value-driven-milestones/
    FV-003-self-healing/
    FV-004-outcome-certification/
    ...
```

Each Reference Validation MUST declare:

| Field | Example |
|-------|---------|
| ID | FV-001 |
| Property | Sequential Milestone Execution |
| Framework version | 0.1 |
| Exercise artifact | Minimal logger (incidental) |

Reference Validation MUST:

- use non-product scope only;
- define explicit milestones exercising the property;
- produce incremental evidence per milestone;
- update Living State after each milestone PASS;
- reach **READY FOR HUMAN REVIEW** without stopping between milestones;
- receive human approval before recording **PASS**.

Reference Validation MUST NOT:

- modify reference product code;
- claim PASS based on exercise artifact quality;
- batch milestones;
- substitute documentation for demonstrated agent behavior.

---

## 6. Validation Record

On human-approved PASS, the agent MUST record:

| Artifact | Location |
|----------|----------|
| Reference Validation evidence | `validation/FV-NNN-*/evidence/VALIDATION-evidence.md` |
| Per-milestone evidence | `validation/FV-NNN-*/evidence/M*-evidence.md` |
| Framework version record | `validation/FRAMEWORK_VERSION.md` |
| Validation index | `validation/README.md` |

The record MUST cite the **framework property** validated — not the exercise artifact.

---

## 7. Framework Version Validated

**Framework Version Validated** is the authoritative state indicating a framework version has passed required Reference Validations and MAY govern product development.

Recorded in `validation/FRAMEWORK_VERSION.md`:

| Field | Content |
|-------|---------|
| Version | e.g. 0.1 |
| Validated date | ISO date |
| Reference Validations | FV-NNN list with PASS |
| Properties certified | Named framework properties |

---

## 8. Resume Authorization

A product Intention paused for *Framework execution model refinement* MAY resume only when:

1. **Framework Version Validated** is recorded;
2. Required Reference Validation(s) for the change have human-approved **PASS**;
3. engineering rules and agent configuration reflect validated framework behavior;
4. product AI Context is slim (milestone scope only).

Resume point is the first **PENDING** milestone unless humans direct otherwise.

---

## 9. Conformance

Violations include:

- approving Framework Validation because the exercise artifact works;
- resuming product delivery without Framework Version Validated;
- framework agent-behavior changes without a corresponding Reference Validation;
- claiming validation from batched milestone execution;
- using product Intention code as substitute for Reference Validation.
