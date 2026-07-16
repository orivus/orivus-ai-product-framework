# Specifications

| Field | Value |
|-------|-------|
| Version | 0.2 |
| Status | Normative |

Normative specifications for the Orivus AI Product Framework (v0.1).

These specifications do not explain. They define conformance.

Validated through product implementations — not inside this repository. See [validation/](../validation/README.md).

---

## Normative Authority

On conflict:

1. `specifications/`
2. `framework/`

---

## Normative Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **MAY**, and **OPTIONAL** are interpreted per [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).

---

## Specifications

| Specification | Status | Defines |
|---------------|--------|---------|
| [Governance Specification](GOVERNANCE_SPECIFICATION.md) | Normative | GS-1 through GS-14 — what every implementation must satisfy |
| [Product Specification](PRODUCT_SPECIFICATION.md) | Normative | Product model, states, artifacts, lifecycle, Product Record |
| [AI Agent Specification](AI_AGENT_SPECIFICATION.md) | Normative | Agent behavior — AS-1 through AS-17 |
| [Milestone Transaction Protocol](MILESTONE_TRANSACTION_PROTOCOL.md) | Normative | Atomic milestone execution — LOCK through NEXT, anti-batching, precedence |
| [Framework Validation Protocol](FRAMEWORK_VALIDATION_PROTOCOL.md) | Normative | Reference Validations — framework property certification before product resume |
| [Conformance Program](CONFORMANCE_PROGRAM.md) | **Preparatory** | Future conformance model — **not operational in v0.1** |

---

## Conformance (v0.1)

**Conformance Program — Status: Preparatory**

Implementations perform **self-assessment against the three normative specifications**.

Operational certification does not exist in v0.1. See [Conformance Program](CONFORMANCE_PROGRAM.md).

---

## Deliberately Not in v0.1

- operational certification or official conformance authority;
- rigid schemas for ADR or RFC;

## Added in v0.2

- [templates/IMPLEMENTATION_PLAN_TEMPLATE.md](../templates/IMPLEMENTATION_PLAN_TEMPLATE.md)
  — milestone structure for value-driven plans (FV-002 PASS).

## Still not in scope

- tool-specific integrations;
- domain-specific rules;
- product-specific practices.

---

## Standard Layers

| Layer | Location |
|-------|----------|
| Identity | [MANIFESTO.md](../MANIFESTO.md) |
| Knowledge | [framework/](../framework/README.md) |
| **Rules** | **This directory** |
| Evidence | [validation/](../validation/README.md) |

Reference implementations: [README.md](../README.md#reference-implementations)
