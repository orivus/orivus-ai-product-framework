# Project Governance

This document describes how the Orivus AI Product Framework is governed as an
open source standard: who decides, how decisions are made, and how the standard
changes over time.

The framework governs its own evolution using the same principles it defines for
products: human-governed, knowledge-driven, and evidence-based.

---

## 1. Principles of Governance

1. **The standard never depends on any product.** Products depend on the
   standard. Governance protects this neutrality above convenience.
2. **Humans govern the standard.** Automation and AI may assist, but approval of
   changes is a human decision.
3. **Evidence over theory.** Behavioral changes require evidence from real
   implementations and a Reference Validation PASS.
4. **Convergence.** Every change should leave the standard clearer and simpler,
   never more contradictory.

---

## 2. Roles

| Role | Responsibility |
|------|----------------|
| **Maintainers** | Steward the standard, review contributions, approve or decline changes, cut releases, protect neutrality. |
| **Contributors** | Anyone who files observations, issues, clarity fixes, or Reference Validations. |
| **Adopters** | Teams implementing the standard in real products; the primary source of evidence. |

Maintainers act as the human governance authority for the standard. In the
experimental phase, the maintainer group is small and may be a single steward.

---

## 3. Types of Change

| Change type | Approval requirement |
|-------------|----------------------|
| Editorial / clarity / errata | Maintainer review. No behavioral impact. |
| New non-normative guidance (appendices, examples) | Maintainer review. Must stay product-neutral. |
| New or modified normative behavior | Evidence + Reference Validation **PASS** + human approval. Deferred to the next version. |

No behavioral change is accepted on theoretical merit alone.

---

## 4. Decision Process

```
Observation / Issue
        ↓
Discussion (issue / PR)
        ↓
Evidence (for behavioral change: Reference Validation)
        ↓
Maintainer review
        ↓
Human approval
        ↓
Release + CHANGELOG entry
```

Maintainers seek consensus. When consensus is not reached, maintainers decide in
favor of protecting the standard's neutrality, consistency, and long-term
maintainability.

Maintainers may decline a contribution — even a technically correct one — if it
compromises the independence or integrity of the standard.

---

## 5. Evolution Gate

The standard is frozen within a version. It advances only through the evolution
gate defined in [ROADMAP.md](ROADMAP.md) and
[FRAMEWORK_VERSION.md](validation/FRAMEWORK_VERSION.md):

```
Standard Change → Reference Validation PASS → Standard Version Validated → Release
```

See the [Framework Validation Protocol](specifications/FRAMEWORK_VALIDATION_PROTOCOL.md)
for how properties are certified.

---

## 6. Releases

- Releases are versioned per [ROADMAP.md](ROADMAP.md#versioning-and-compatibility).
- Every release updates [CHANGELOG.md](CHANGELOG.md).
- Behavioral changes reference the Reference Validation(s) that certify them.

---

## 7. Conformance Authority

An operational conformance authority does **not** exist in v0.1. The
[Conformance Program](specifications/CONFORMANCE_PROGRAM.md) is preparatory.
Until it becomes operational, implementations perform **self-assessment** against
the normative specifications.

---

## 8. Changing This Document

Changes to governance follow the same process described here: propose via issue,
discuss, and obtain maintainer approval. Governance changes must preserve human
authority and standard neutrality.
