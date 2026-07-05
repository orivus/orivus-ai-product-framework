# Release Notes — v0.1 (Experimental)

| Field | Value |
|-------|-------|
| Version | 0.1 |
| Status | **Experimental** — Frozen |
| Date | 2026-07-05 (standard) · 2026-07-07 (open source release) |
| License | [Apache License 2.0](LICENSE) |

---

## Experimental Disclaimer

**This is an experimental standard (v0.1).**

It is published to be studied, adopted, tested, and improved through real use. It
is suitable for evaluation and for building reference implementations. It does
**not** yet carry stability, backward-compatibility, or support guarantees.

The standard may change as evidence accumulates. Adopt it with that expectation.

---

## What This Release Is

The Orivus AI Product Framework is an open engineering standard for
**knowledge-driven, AI-native product development**. It defines how humans and AI
agents collaborate to build software products without losing product knowledge,
architectural integrity, engineering quality, or long-term maintainability.

It is a **specification**, not a tool or library. It is technology-independent,
vendor-independent, and product-independent.

---

## What's Included

| Layer | Content |
|-------|---------|
| **Identity** | [MANIFESTO.md](MANIFESTO.md) — why the standard exists |
| **Knowledge** | [framework/](framework/README.md) — Core Principles (CP-1…CP-11), lifecycles, knowledge model, AI execution model, canonical workflow |
| **Rules** | [specifications/](specifications/README.md) — Governance (GS-1…GS-13), Product, AI Agent (AS-1…AS-15), Milestone Transaction Protocol, Framework Validation Protocol |
| **Evidence** | [validation/](validation/README.md) — Reference Validation suite; FV-001 **PASS** |

Open source project files: `LICENSE`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`,
`SECURITY.md`, `GOVERNANCE.md`, `ROADMAP.md`, `CHANGELOG.md`,
`FRAMEWORK_FEEDBACK.md`.

---

## What Was Validated

v0.1 certifies **framework behavior**, not any product.

**FV-001 — Sequential Milestone Execution (PASS, human-approved):**

- one milestone per transaction;
- no batching;
- incremental, per-milestone evidence;
- incremental Living State;
- autonomous milestone progression;
- Human Review only at the end of an Intention.

See [validation/FRAMEWORK_VERSION.md](validation/FRAMEWORK_VERSION.md).

---

## Known Limitations

- **Experimental maturity.** Concepts and rules may change in v0.2 based on
  evidence.
- **Single Reference Validation.** Only FV-001 is certified. Self-Healing,
  Outcome Certification, Product Knowledge Synchronization, and Dead Code
  Convergence are **not yet** independently validated (FV-002…FV-005 planned).
- **Conformance is self-assessment.** No operational conformance authority or
  certification exists. The [Conformance Program](specifications/CONFORMANCE_PROGRAM.md)
  is preparatory.
- **External validity is still accumulating.** Broad, independent,
  cross-domain evidence is the gate for v0.2.
- **No migration guarantees yet.** During 0.x, breaking changes may occur; they
  will be documented in [CHANGELOG.md](CHANGELOG.md).

---

## Who Should Use This

- Engineering leaders evaluating governed AI-assisted development.
- Teams building AI-native products that need to preserve product knowledge.
- Researchers and practitioners studying human–AI engineering collaboration.
- Builders of reference implementations willing to contribute evidence.

You do **not** need any specific product, vendor, IDE, or AI model to adopt it.

---

## How to Start

1. Read [MANIFESTO.md](MANIFESTO.md) (identity).
2. Read [framework/INTRODUCTION.md](framework/INTRODUCTION.md) (conceptual model).
3. Study [specifications/](specifications/README.md) (normative rules).
4. Review [validation/FV-001](validation/FV-001-sequential-milestone-loop/README.md)
   as a concrete, self-contained example of governed execution.
5. Record friction in [FRAMEWORK_FEEDBACK.md](FRAMEWORK_FEEDBACK.md).

---

## What's Next

See [ROADMAP.md](ROADMAP.md). v0.2 opens only with evidence from independent
reference implementations plus Reference Validation PASS for each behavioral
change.
