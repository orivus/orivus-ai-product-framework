# Implementation Plan Template

| Field | Value |
|-------|-------|
| Version | 0.2 |
| Status | Normative guidance — v0.2 |
| License | Apache License 2.0 |

This template defines the required structure for Implementation Plans under the
Orivus AI Product Framework v0.2.

The Implementation Plan is the execution contract. It MUST define milestones as
**verifiable increments of product value** toward the Product Outcome — not as
technical execution units.

Informative example: [examples/inventory-platform/IMPLEMENTATION_PLAN-001.md](../examples/inventory-platform/IMPLEMENTATION_PLAN-001.md).

Normative requirements: [Governance Specification §GS-14](../specifications/GOVERNANCE_SPECIFICATION.md#gs-14--verifiable-product-value-per-milestone) · [Product Specification §8.6](../specifications/PRODUCT_SPECIFICATION.md#86-milestones).

---

## Plan Header

```markdown
# Implementation Plan-NNN — <Intention title>

| Field | Value |
|-------|-------|
| Product | <product name> |
| Implementation Plan | NNN |
| Related Intention | [Intention NNN](INTENTION-NNN.md) |
| Related ADR | [ADR-NNN](ADR-NNN.md) |
| Related RFC | [RFC-NNN](RFC-NNN.md) |
| Status | Active |
```

---

## Living State

Track milestone execution. Update after each Milestone PASS only.

```markdown
## Living State

| Milestone | Product Value (summary) | Status |
|-----------|-------------------------|--------|
| M1 — <title> | <one-line value> | `PENDING` |
| M2 — <title> | <one-line value> | `PENDING` |

**Current milestone:** M1 — <title>

**Intention status:** Under Execution
```

---

## Product Outcome Traceability

Every milestone MUST be a partial realization of the same Product Outcome.

```markdown
## Product Outcome

<Measurable capability when the Intention completes>

## Milestone Progression

Product Outcome
    → M1 (+ verifiable value)
    → M2 (+ verifiable value)
    → M3 (+ verifiable value)
    → Product Outcome complete
```

---

## Milestone Structure (required)

Each milestone MUST use this structure. **Product Value comes first.**
Implementation Tasks come last.

```markdown
## Milestone M1 — <value-oriented title>

### Product Value

<What new value the product delivers when this milestone completes.
Describe value for the product consumer — not technical deliverables.>

### Consumer

<Who experiences this value — end user, operator, API consumer, application.>

### Observable Result

<What can be observed in the real world when the value exists.
Must be verifiable without reading source code.>

### Evidence

- <Automated test or certification entrypoint>
- <Human verification when required — smoke test, device test, operator check>

### Implementation Tasks

- <Technical work required to deliver the value — modules, layers, adapters>
- <Tasks are not milestones>
```

### Invalid milestone (non-conformant)

```markdown
## Milestone M1 — Repository Layer

Deliverables: repository module, unit tests
```

A milestone defined only by technical artifacts MUST NOT receive PASS under GS-14.

---

## Quality Gates

Reference repository Engineering Rules. Every milestone transaction MUST reach
VERIFIED before Milestone Audit.

---

## Closing the Intention

After all milestones PASS:

1. Outcome Certification exercises the complete Product Outcome.
2. Intention Audit confirms the Product Outcome exists (GS-13).
3. Human Review approves the completed capability.
4. Product Knowledge Synchronization records verified knowledge.
5. Intention marked **Closed**.
