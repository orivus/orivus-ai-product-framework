# Contributing to the Orivus AI Product Framework

Thank you for your interest in improving this standard.

This document explains how the standard evolves, how to contribute, and what
kind of contributions are accepted at each stage of its lifecycle.

The framework governs its own evolution using the same principles it defines:
knowledge-driven, evidence-based, and human-governed.

---

## 1. Project Status

| Field | Value |
|-------|-------|
| Version | 0.2 |
| Status | **Validated** |
| License | [Apache License 2.0](LICENSE) |

**v0.1 is frozen.** No new specifications, rules, or concepts are accepted into
v0.1. The standard evolves to v0.2 only through evidence gathered from real
product implementations — not from theoretical additions.

This does not mean contributions are closed. It means contributions are
**channeled through evidence**, not through speculative design.

---

## 2. The Golden Rule

> The standard never depends on any product.
> Products depend on the standard.

Every contribution MUST keep the specification **technology-independent,
vendor-independent, and product-independent**.

Contributions that embed a specific product, company, tool, IDE, AI model, or
vendor into the normative specification will be declined.

Concrete implementations belong in **reference implementations** maintained
independently from this repository.

---

## 3. What You Can Contribute

### Always welcome

- **Observations** — friction found while applying the framework to a real
  product. File these in [FRAMEWORK_FEEDBACK.md](FRAMEWORK_FEEDBACK.md) using
  the observation template.
- **Clarity fixes** — typos, broken links, ambiguous wording, inconsistent
  terminology, or contradictory status labels. These do not change behavior.
- **Reference Validations** — new evidence (`FV-NNN`) that certifies a framework
  property. See [Framework Validation Protocol](specifications/FRAMEWORK_VALIDATION_PROTOCOL.md).
- **Adoption guidance** — non-normative appendices, examples, and getting-started
  material that help others adopt the standard.

### Deferred to v0.2

- New specifications, rules, principles, or concepts.
- Changes to normative agent behavior.
- Changes to the lifecycle, knowledge model, or governance model.

These require empirical evidence from independent reference implementations and
a Reference Validation PASS before they can be adopted.

**v0.2** is validated — see [RELEASE-NOTES-v0.2.md](RELEASE-NOTES-v0.2.md)
and [FRAMEWORK_VERSION.md](validation/FRAMEWORK_VERSION.md). v0.1 remains frozen.

---

## 4. How the Standard Evolves

Behavioral changes follow the framework's own evolution rule:

```
Standard Change → Reference Validation → Standard Version Validated → Standard Release
```

A change that modifies agent behavior is **not validated** until its Reference
Validation reaches **PASS**. See [validation/](validation/README.md) and
[FRAMEWORK_VERSION.md](validation/FRAMEWORK_VERSION.md).

No behavioral change is accepted on theoretical merit alone.

---

## 5. Contribution Workflow

1. **Open an issue first.** Describe the problem or observation before writing a
   change. For friction found in real use, prefer an `FRAMEWORK_FEEDBACK.md`
   observation (OBS-NNN).
2. **Keep scope small.** One concern per pull request.
3. **Preserve independence.** No product, vendor, or tool names in normative or
   framework documents.
4. **Match the existing style.** Markdown first, RFC 2119 language in
   specifications, consistent status and version headers.
5. **Update consistency surfaces.** If you change a rule ID, count, or status,
   update every place it is referenced.
6. **Provide evidence for behavioral changes.** Link the Reference Validation
   that certifies the change.

---

## 6. Documentation Rules

Every document should answer four questions and avoid duplication:

- **Why** does this exist?
- **How** is it applied?
- **When** does it apply?
- **When does it NOT** apply?

Normative requirements live in [specifications/](specifications/README.md).
Conceptual explanations live in [framework/](framework/README.md).
Evidence lives in [validation/](validation/README.md).

On conflict, `specifications/` prevails over `framework/`.

---

## 7. Reporting Problems

- **Standard defects, ambiguities, contradictions** → open a GitHub issue.
- **Friction from real-world use** → add an observation to
  [FRAMEWORK_FEEDBACK.md](FRAMEWORK_FEEDBACK.md).
- **Security concerns** → follow [SECURITY.md](SECURITY.md).

---

## 8. Governance

The standard is human-governed. Maintainers approve changes based on:

- independence from products and vendors;
- consistency with Core Principles;
- evidence supporting behavioral changes;
- clarity and long-term maintainability.

Maintainers may decline contributions that compromise the neutrality or
integrity of the standard, even if technically correct.

See [GOVERNANCE.md](GOVERNANCE.md) for roles and the decision process.

---

## 9. Code of Conduct

All participation is governed by our
[Code of Conduct](CODE_OF_CONDUCT.md). By contributing, you agree to uphold it.

---

## 10. License of Contributions

By submitting a contribution, you agree that it is licensed under the
[Apache License 2.0](LICENSE), consistent with the rest of the project.
