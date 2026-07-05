# Changelog

All notable changes to the Orivus AI Product Framework are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This standard versions its **specification**, so entries describe changes to
concepts, rules, and normative behavior — not code.

Versioning policy: see [ROADMAP.md](ROADMAP.md#versioning-and-compatibility).

---

## [Unreleased]

Nothing yet. v0.1 is frozen. Changes accumulate as observations in
[FRAMEWORK_FEEDBACK.md](FRAMEWORK_FEEDBACK.md) and are considered for v0.2 only
with supporting evidence from independent reference implementations.

---

## [0.1.2] — 2026-07-07

Adoption experience. Editorial and documentation only. **No change to normative
behavior**; v0.1 remains frozen.

### Added

- `docs/` — a small static website for GitHub Pages: Home, Problem, Why this
  standard exists, Core Principles, and the essay
  "Why Product Knowledge Matters in AI-Native Engineering".
- `examples/inventory-platform/` — a minimal, product-neutral walkthrough that
  shows the standard applied end to end through a **single** Intention
  (Opportunity → Discovery → Foundation → Intention → Outcome → Audit → Closed).

### Changed

- Rewrote the top of `README.md` for a 60-second read: canonical flow diagram
  (Product Opportunity → Product Knowledge → Intention → Governed AI Engineering
  → Verified Product → Institutional Knowledge), "why code generation is not
  enough", "why you need this", and a clearer Start Here.
- Updated `examples/README.md` to list the Inventory Platform walkthrough
  alongside FV-001.

### Notes

- `specifications/` and `framework/` were not modified. Only README links changed.

---

## [0.1.0] — 2026-07-05

First experimental release of the standard.

### Added

- **Identity** — [MANIFESTO.md](MANIFESTO.md): mission, philosophy, and vision
  of a knowledge-driven, AI-native product engineering standard.
- **Knowledge layer** — [framework/](framework/README.md): Introduction, Core
  Principles (CP-1…CP-11), Product Lifecycle, Product Knowledge Model, AI
  Execution Model, Governance Rules (GR-1…GR-13), Canonical Workflow, and
  informative Appendices.
- **Rules layer** — [specifications/](specifications/README.md):
  - Governance Specification (GS-1…GS-13)
  - Product Specification
  - AI Agent Specification (AS-1…AS-15)
  - Milestone Transaction Protocol (LOCK → IMPLEMENT → VERIFY → AUDIT → PASS →
    UPDATE PLAN → NEXT; anti-batching; precedence stack)
  - Framework Validation Protocol
  - Conformance Program (preparatory — not operational in v0.1)
- **Evidence layer** — [validation/](validation/README.md): Reference Validation
  suite and [FRAMEWORK_VERSION.md](validation/FRAMEWORK_VERSION.md).
- **FV-001 — Sequential Milestone Execution**: first Reference Validation,
  human-approved **PASS**. Certifies one-milestone-per-transaction, anti-batching,
  incremental evidence, incremental Living State, autonomous progression, and
  Human Review only at Intention end.
- All normative specifications adopt RFC 2119 / RFC 8174 language.

### Notes

- GR-13 / GS-13 (Product Outcome Realization) and the Milestone Transaction
  Protocol were introduced as operational refinements after observations
  OBS-001…OBS-004, and validated by FV-001 before adoption.

---

## [0.1.1] — 2026-07-07

Open source release readiness. Documentation and repository hygiene only. No
change to normative behavior.

### Added

- `LICENSE` (Apache License 2.0).
- `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`, `GOVERNANCE.md`,
  `ROADMAP.md`, `CHANGELOG.md`.
- `RELEASE-NOTES-v0.1.md` with experimental disclaimer and known limitations.
- `examples/` index describing reference implementations in neutral terms.
- `validation/OSR-001/` — Open Source Release Readiness Audit as project
  evidence.

### Changed

- Made the specification product-neutral: removed named-product references from
  the standard and replaced them with "reference implementation" /
  "product implementation" terminology.
- Unified status labels and version headers across documents.
- Aligned rule counts to GS-13 / GR-13.
- Replaced tool-specific wording ("Cursor Rules") with neutral "engineering
  rules / agent configuration".
- Fixed broken cross-references.

[Unreleased]: https://example.com/orivus-ai-product-framework
[0.1.0]: https://example.com/orivus-ai-product-framework
[0.1.1]: https://example.com/orivus-ai-product-framework
