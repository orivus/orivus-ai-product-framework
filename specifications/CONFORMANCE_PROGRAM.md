# Conformance Program

| Field | Value |
|-------|-------|
| Version | 0.1 |
| Status | Preparatory — Not operational in v0.1 |
| Scope | Products · Agents · Repositories · Workflows · Organizations |

> **Preparatory specification.** This document defines a conformance model for future use. It is **not operational in v0.1**. No certification authority exists. It will be revised based on evidence from product implementations.

This specification defines how implementations are evaluated for conformance with the Orivus AI Product Framework.

Conformance is not self-evident.

Conformance MUST be demonstrated through evidence.

This program defines the levels, evidence, verdicts, and records required to make conformance objective and verifiable.

---

## 1. Purpose

The Conformance Program exists to answer:

> How do we know that an organization, product, or agent complies with the standard?

This specification defines:

- conformance levels;
- evidence requirements;
- assessment verdicts;
- certification scope;
- non-conformance rules;
- certification record requirements.

Without this program, normative specifications are unverifiable.

With this program, the framework becomes an auditable open standard.

---

## 2. Normative Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **MAY**, and **OPTIONAL** are interpreted per [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).

---

## 3. Normative Specifications

Conformance is evaluated against these specifications:

| Specification | Identifier |
|---------------|------------|
| [Governance Specification](GOVERNANCE_SPECIFICATION.md) | GS-1 through GS-14 |
| [Product Specification](PRODUCT_SPECIFICATION.md) | Product requirements |
| [AI Agent Specification](AI_AGENT_SPECIFICATION.md) | AS-1 through AS-17 |
| This Conformance Program | Conformance requirements |

---

## 4. Conformance Levels

An implementation MUST declare the conformance level being assessed.

Higher levels include all requirements of lower levels within their scope.

### Level 1 — Governance Conformant

An implementation is **Governance Conformant** when it satisfies all MUST and MUST NOT requirements in [Governance Specification](GOVERNANCE_SPECIFICATION.md).

**Scope:** Engineering process, governance boundaries, and operational constraints.

**Applies to:** Products, agents, repositories, workflows, and organizations adopting governed AI-assisted engineering.

**Does NOT require:** Full Product or Agent specification conformance.

---

### Level 2 — Product Conformant

An implementation is **Product Conformant** when it satisfies:

- Level 1 — Governance Conformant; and
- all MUST and MUST NOT requirements in [Product Specification](PRODUCT_SPECIFICATION.md).

**Scope:** Product definition, Product Foundation, Product State, Product Knowledge, contracts, delivery, and Product Record.

**Applies to:** AI-native products claiming framework product conformance.

---

### Level 3 — Agent Conformant

An implementation is **Agent Conformant** when it satisfies:

- Level 1 — Governance Conformant; and
- all MUST and MUST NOT requirements in [AI Agent Specification](AI_AGENT_SPECIFICATION.md).

**Scope:** Agent identity, cognitive model, knowledge ingestion, execution, governance, memory, human interaction, and invocation.

**Applies to:** AI agents, coding agents, autonomous engineering systems, or custom agent runtimes.

**Does NOT require:** Product Conformant status — an agent MAY be assessed independently.

---

### Level 4 — Full Framework Conformant

An implementation is **Full Framework Conformant** when it satisfies:

- Level 2 — Product Conformant; and
- Level 3 — Agent Conformant; and
- demonstrated governed operation of conformant agents on a conformant product.

**Scope:** Complete framework adoption — product, agent, and governance operating together.

**Applies to:** Reference implementations, production AI-native products with governed agent execution, and organizations claiming full standard adoption.

---

### Conformance Level Summary

| Level | Name | Required Specifications |
|-------|------|------------------------|
| **1** | Governance Conformant | Governance Specification |
| **2** | Product Conformant | Governance + Product Specification |
| **3** | Agent Conformant | Governance + AI Agent Specification |
| **4** | Full Framework Conformant | All three specifications + demonstrated integration |

---

## 5. Evidence Requirements

An implementation MUST provide evidence for each applicable requirement being assessed.

Claims without evidence are non-conformant.

### 5.1 General Evidence Rules

Evidence MUST be:

- **objective** — verifiable by a third party;
- **reproducible** — assessment can be repeated;
- **scoped** — tied to the assessed implementation and conformance level;
- **current** — reflects the state at time of assessment.

Evidence MUST NOT consist solely of undocumented assertions or interaction history.

### 5.2 Acceptable Evidence Types

Evidence MAY include, as applicable:

| Evidence Type | Demonstrates |
|---------------|--------------|
| Product Foundation | Product knowledge readiness (GS-1, Product Specification §6) |
| Product State record | Current lifecycle state (Product Specification §14) |
| Product Record | Authoritative product history (Product Specification §13) |
| Intentions | Governed change units (GS-2, GS-3) |
| Product Outcomes | Measurable capability definitions (GS-3) |
| Planning artifacts | Governed planning knowledge (GS-4) |
| AI Context | Scoped execution boundary (GS-5) |
| Validation reports | VERIFIED state achievement (GS-6) |
| Milestone audits | Milestone PASS verdicts (GS-7, GS-8) |
| Intention audits | Intention completion verdicts (GS-8, GS-9) |
| Product Shared Memory | Verified knowledge preservation (GS-10) |
| Knowledge artifacts | Markdown-first governed knowledge (GS-11) |
| Architecture records | Architecture authority compliance (GS-12) |
| Human approval records | Intention-level human governance (GS-9) |
| Agent invocation records | Governed agent invocation (AI Agent Specification §8) |
| Prohibited behavior absence | No AS-1 through AS-17 violations |

### 5.3 Evidence by Conformance Level

| Level | Minimum Evidence |
|-------|-----------------|
| **1 — Governance** | Documented governance process; evidence for applicable GS requirements |
| **2 — Product** | Product Foundation; Product State; Product Record; contracts; at least one completed or active Intention with Outcome |
| **3 — Agent** | Agent specification compliance demonstration; invocation model evidence; cognitive cycle evidence |
| **4 — Full Framework** | All Level 2 and Level 3 evidence; at least one complete Intention cycle with audits and human approval |

---

## 6. Assessment Process

### 6.1 Assessment Steps

Every conformance assessment MUST follow:

```
1. Declare scope and conformance level
        │
        ▼
2. Identify applicable requirements
        │
        ▼
3. Collect evidence per requirement
        │
        ▼
4. Evaluate each requirement
        │
        ▼
5. Assign verdict per requirement
        │
        ▼
6. Determine overall conformance level
        │
        ▼
7. Produce Certification Record
```

### 6.2 Assessor

An assessment MAY be performed by:

- the implementing organization (self-assessment);
- an independent auditor;
- a future certified conformance authority (TBD).

Self-assessment MUST follow identical evidence and verdict rules.

### 6.3 Requirement Evaluation

Each applicable requirement MUST be evaluated individually.

Requirements marked NOT APPLICABLE MUST include documented justification.

Undocumented NOT APPLICABLE markings are non-conformant.

---

## 7. Verdicts

Every requirement evaluation MUST produce exactly one verdict.

### 7.1 Verdict Definitions

| Verdict | Meaning |
|---------|---------|
| **PASS** | Requirement satisfied with sufficient evidence |
| **PARTIAL** | Requirement partially satisfied — SHOULDS met but MUSTS incomplete, or evidence incomplete |
| **FAIL** | MUST or MUST NOT requirement violated, or evidence absent |
| **NOT APPLICABLE** | Requirement does not apply to assessed scope — justification REQUIRED |

### 7.2 Verdict Rules

- Any **FAIL** on a MUST or MUST NOT requirement MUST result in failure of the assessed conformance level.
- **PARTIAL** MAY be accepted only for SHOULD requirements or when explicitly defined by assessment scope.
- **NOT APPLICABLE** MUST be justified in the Certification Record.
- Absence of evidence for a MUST requirement MUST be verdict **FAIL**.

### 7.3 Overall Level Verdict

| Condition | Overall Verdict |
|-----------|----------------|
| All applicable MUST requirements PASS | Level conformance **PASS** |
| Any MUST requirement FAIL | Level conformance **FAIL** |
| Only SHOULD gaps with documented rationale | Level conformance **PARTIAL** (if assessor permits) |

Full Framework Conformant (Level 4) MUST NOT receive PASS if either Product or Agent assessment FAILs.

---

## 8. Certification Scope

Certification MAY apply to:

| Scope | Description | Typical Level |
|-------|-------------|---------------|
| **Product** | A defined AI-native product | Level 2 or 4 |
| **Agent** | An AI agent or agent runtime | Level 3 or 4 |
| **Repository** | A product repository and its knowledge structure | Level 1 or 2 |
| **Workflow** | An engineering workflow adopting framework governance | Level 1 |
| **Organization** | An organization's adoption of the framework | Level 1 through 4 |

Certification scope MUST be declared before assessment begins.

Assessment MUST NOT claim broader scope than evidence supports.

---

## 9. Non-Conformance

### 9.1 Automatic Fail Conditions

Any of the following MUST result in FAIL:

- violation of any MUST requirement;
- violation of any MUST NOT requirement;
- production implementation without Product Foundation (GS-1);
- agent execution without AI Context (GS-5, AS-2);
- Milestone Audit before VERIFIED state (GS-6);
- human approval required per Milestone (GS-9, AS-8);
- unverified knowledge in Product Shared Memory (GS-10, AS-10);
- silent architectural redefinition (GS-12, AS-6);
- incomplete cognitive cycle without Learn phase (AS-13).

### 9.2 Remediation

A FAIL verdict MUST identify:

- violated requirement identifier (GS-n, AS-n, or Product Specification section);
- evidence gap;
- recommended remediation.

Re-assessment MAY occur after remediation.

Previous FAIL records SHOULD be preserved in Certification Record history.

---

## 10. Certification Record

Every assessment MUST produce a Certification Record.

### 10.1 Record Requirements

A Certification Record MUST include:

| Field | Required |
|-------|----------|
| Assessment identifier | Unique ID |
| Assessment date | ISO 8601 date |
| Framework version | e.g., 0.1 |
| Assessed entity | Product, agent, org, or repo name |
| Certification scope | As declared in §8 |
| Conformance level assessed | Level 1, 2, 3, or 4 |
| Assessor | Self, independent, or authority |
| Requirement evaluations | Per-requirement verdicts with evidence references |
| Overall verdict | PASS, PARTIAL, or FAIL |
| NOT APPLICABLE justifications | Where used |
| Remediation notes | If FAIL or PARTIAL |
| Validity period | OPTIONAL — TBD by certification authority |

### 10.2 Record Persistence

Certification Records MUST be human-readable.

Certification Records SHOULD be machine-readable.

Markdown is RECOMMENDED.

Certification Records MUST be version-controlled when assessing repositories.

### 10.3 Record Template

```markdown
# Certification Record

| Field | Value |
|-------|-------|
| Assessment ID | [unique-id] |
| Date | [YYYY-MM-DD] |
| Framework Version | 0.1 |
| Entity | [name] |
| Scope | [product | agent | repository | workflow | organization] |
| Level Assessed | [1 | 2 | 3 | 4] |
| Assessor | [name or organization] |
| Overall Verdict | [PASS | PARTIAL | FAIL] |

## Requirement Evaluations

| Requirement | Verdict | Evidence Reference |
|-------------|---------|-------------------|
| GS-1 | PASS | [link or path] |
| GS-2 | PASS | [link or path] |
| ... | ... | ... |

## NOT APPLICABLE Justifications

| Requirement | Justification |
|-------------|---------------|
| [if any] | [reason] |

## Remediation

[If FAIL or PARTIAL — required actions]

## Assessor Statement

[Declaration of assessment methodology and independence]
```

---

## 11. Self-Declaration vs Third-Party Certification

### 11.1 Self-Declaration

Organizations MAY self-declare conformance by:

1. performing assessment per this program;
2. producing a complete Certification Record;
3. publishing the record with declared conformance level.

Self-declared conformance MUST NOT imply third-party validation.

### 11.2 Third-Party Certification

Third-party certification MAY be established by a future conformance authority.

Third-party certification MUST follow this program without reduction of requirements.

Authority identification: TBD.

### 11.3 Reference Implementation Declaration

Reference implementations SHOULD publish Certification Records when the Conformance Program becomes operational.

---

## 12. Conformance Program Conformance

An assessment process is valid when it satisfies all MUST requirements in this document.

Invalid assessment processes MUST NOT produce authoritative Certification Records.

---

## 13. Informative References

- [Governance Specification](GOVERNANCE_SPECIFICATION.md)
- [Product Specification](PRODUCT_SPECIFICATION.md)
- [AI Agent Specification](AI_AGENT_SPECIFICATION.md)
- [framework/Canonical Workflow](../framework/CANONICAL_WORKFLOW.md)

---

## 14. Program Summary

```
Declare scope → Collect evidence → Evaluate requirements → Assign verdicts → Produce Certification Record
```

| Level | What it proves |
|-------|----------------|
| **1** | Governed engineering process |
| **2** | Well-defined AI-native product |
| **3** | Framework-conformant agent behavior |
| **4** | Complete standard adoption |

Conformance transforms the Orivus AI Product Framework from well-designed documentation into a verifiable open standard.
