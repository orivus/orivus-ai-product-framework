# Orivus AI Product Framework — Introduction

| Field | Value |
|-------|-------|
| Version | 0.1 |
| Status | Experimental — Frozen |
| Normative Counterpart | [specifications/](../specifications/README.md) |

Entry point to the conceptual model.

For the standard's identity — why it exists, its mission, philosophy, and vision — read [MANIFESTO.md](../MANIFESTO.md) first.

This document bridges identity to knowledge: how the standard thinks about product engineering.

---

## 1. From Identity to Knowledge

The [Manifesto](../MANIFESTO.md) establishes three governing principles:

```
Humans govern the product.
AI governs engineering execution.
Product Knowledge governs the code.
```

The framework layer explains what those principles mean in practice — without defining conformance. Normative requirements live in [specifications/](../specifications/README.md).

---

## 2. Two Lifecycles

The standard distinguishes two independent but connected lifecycles.

**Product Discovery** eliminates uncertainty before software is built. Its output is Product Foundation — sufficient knowledge to begin implementation safely.

**Product Delivery** evolves the product continuously through governed Problem Resolutions. Each resolution moves the product toward a better state and leaves institutional knowledge behind.

These lifecycles have different objectives, different outputs, and different governance rules. They must never be confused.

See [Product Lifecycle](PRODUCT_LIFECYCLE.md) for the complete model.

---

## 3. Knowledge as the Permanent Asset

The standard is knowledge-native, not tool-native.

Product Knowledge — purpose, boundaries, architecture, contracts, decisions, validated learning — is the permanent engineering asset.

Artificial Intelligence is the current executor of governed work.

Source code is the derived artifact.

When the executor changes, the knowledge remains. This is why the standard survives beyond any single AI product, vendor, or development tool.

See [Product Knowledge Model](PRODUCT_KNOWLEDGE_MODEL.md) for the complete knowledge architecture.

---

## 4. Governed Execution

Engineering execution follows a repeatable cognitive cycle:

```
Understand → Plan → Execute → Verify → Learn
```

Every execution operates inside governed Product Knowledge — not unrestricted repository access.

Every completion produces evidence — not assumptions.

Every verified outcome becomes permanent Product Knowledge.

See [AI Execution Model](AI_EXECUTION_MODEL.md) and [Canonical Workflow](CANONICAL_WORKFLOW.md) for the complete execution model.

---

## 5. What This Layer Is — and Is Not

| Is | Is Not |
|----|--------|
| The conceptual model for knowledge-driven product development | The standard's identity document — that is the [Manifesto](../MANIFESTO.md) |
| Informative — explains how to think about products | Normative — conformance lives in [specifications/](../specifications/README.md) |
| Vendor, tool, and domain independent | A complete operational manual |
| The minimum conceptual model for v0.1 | A growing collection of new concepts |

Reference implementations validate the standard. They live in separate product repositories — see [README.md](../README.md#reference-implementations).

---

## 6. Reading Order

After the [Manifesto](../MANIFESTO.md):

| # | Document | Question |
|---|----------|----------|
| 1 | [Core Principles](CORE_PRINCIPLES.md) | What never changes? |
| 2 | [Product Lifecycle](PRODUCT_LIFECYCLE.md) | How does a product live? |
| 3 | [Product Knowledge Model](PRODUCT_KNOWLEDGE_MODEL.md) | What knowledge governs engineering? |
| 4 | [AI Execution Model](AI_EXECUTION_MODEL.md) | How does governed execution work? |
| 5 | [Governance Rules](GOVERNANCE_RULES.md) | What are the operational constraints? |
| 6 | [Canonical Workflow](CANONICAL_WORKFLOW.md) | What is the complete flow? |
| 7 | [Appendices](APPENDICES.md) | Optional adoption guidance |
