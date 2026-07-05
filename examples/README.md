# Examples & Reference Implementations

| Field | Value |
|-------|-------|
| Status | Non-normative |
| Adds rules | No |

This section explains how examples and reference implementations relate to the
standard. Nothing here defines conformance.

---

## The Boundary

> The standard never depends on a product. Products depend on the standard.

- The **standard** answers universal questions: how AI-native products are built,
  how humans collaborate with AI agents, how Product Knowledge is preserved, how
  AI-induced technical debt is avoided, and how engineering execution is governed.
- A **reference implementation** is a real product that adopts the standard. It
  demonstrates the standard in practice and produces evidence that guides the
  standard's evolution.

Reference implementations are **maintained independently** from this
specification — ideally in separate repositories. They are not part of the
standard and MUST NOT be required to understand or adopt it.

---

## Examples Inside This Repository

Two neutral, self-contained examples ship with the standard. Neither is a real
product; both exist only to make the standard concrete.

| Example | Teaches | Kind |
|---------|---------|------|
| [Inventory Platform](inventory-platform/README.md) | What a product looks like using the standard: Opportunity → Discovery → Foundation → one Intention → Outcome → Audit → Closed. | Product walkthrough (documentation only) |
| [FV-001 — Sequential Milestone Execution](../validation/FV-001-sequential-milestone-loop/README.md) | How governed execution behaves: one milestone per transaction under the Milestone Transaction Protocol. | Framework property validation |

The Inventory Platform is deliberately limited to **a single Intention** so the
shape of the standard is visible without the noise of a full product. FV-001 is
domain-neutral and proves a framework property, not a product.

---

## External Reference Implementations

Reference implementations are currently being developed to validate the
experimental specification across independent domains, such as:

- voice / speech platforms
- embedded and edge devices
- mobile applications
- backend and SaaS platforms

The standard evolves only through evidence obtained from these real product
implementations — never through new theory.

When an external reference implementation is published, it will be linked here
with an absolute URL, clearly labeled as an independent product that adopts the
standard — not as part of it.

---

## How Evidence Flows Back

```
Reference implementation applies the standard
        ↓
Friction is recorded as an observation (OBS-NNN) in FRAMEWORK_FEEDBACK.md
        ↓
A behavioral change is justified by evidence
        ↓
A Reference Validation (FV-NNN) certifies the change
        ↓
The standard evolves (see ROADMAP.md)
```

See [FRAMEWORK_FEEDBACK.md](../FRAMEWORK_FEEDBACK.md) and
[ROADMAP.md](../ROADMAP.md).
