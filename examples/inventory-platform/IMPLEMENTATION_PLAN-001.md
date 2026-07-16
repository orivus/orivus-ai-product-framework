# Implementation Plan-001 — Real-Time Stock Visibility API

| Field | Value |
|-------|-------|
| Product | Inventory Platform (example) |
| Intention | 001 — Real-Time Stock Visibility API |
| Status | Example — illustrative only |
| Framework | v0.2 — value-driven milestones |
| Outcome | An operator can query current stock over HTTP |

> Teaching artifact. It shows how one Intention is planned into **value-driven
> milestones** and how **Living State** tracks execution. It is not real product
> code.

Template: [templates/IMPLEMENTATION_PLAN_TEMPLATE.md](../../templates/IMPLEMENTATION_PLAN_TEMPLATE.md).

---

## Product Outcome

An operator can request current stock for a warehouse and SKU and receive an
accurate, current quantity through the public HTTP API.

## Milestone Progression

```
Product Outcome (operator can query stock)
    → M1 (+ operator can query one known item)
    → M2 (+ operator can query any item in catalog)
    → M3 (+ operator receives not-found for unknown items)
    → Product Outcome complete
```

---

## Living State

| Milestone | Product Value (summary) | Status |
|-----------|-------------------------|--------|
| M1 — Operator can query stock for known items | Accurate quantity for valid warehouse + SKU | `PASS` |
| M2 — Operator can query any catalog item | Same contract for full catalog | `PASS` |
| M3 — Operator receives not-found for unknown items | Clear absence for invalid SKU | `PASS` |

**Current milestone:** — (complete)

**Plan status:** READY FOR HUMAN REVIEW

In a real product, Living State is updated after **each** milestone PASS — never
in advance. Here every milestone already shows PASS because the example depicts a
completed Intention.

---

## How Each Milestone Runs

Every milestone follows the
[Milestone Transaction Protocol](../../specifications/MILESTONE_TRANSACTION_PROTOCOL.md):

```
LOCK → IMPLEMENT → VERIFY → AUDIT → PASS → UPDATE PLAN → NEXT
```

Exactly one milestone is open at a time. No batching.

Milestone Audit asks: **does the promised product value exist?**

---

## Milestone M1 — Operator can query stock for known items

### Product Value

An operator can retrieve the current quantity for a valid warehouse and SKU that
exists in the stock store.

### Consumer

Warehouse operator using the inventory HTTP API.

### Observable Result

`GET /stock?warehouse=W1&sku=SKU-001` returns `200` with the current quantity.

### Evidence

- Integration test: known warehouse + SKU returns accurate quantity
- Operator smoke test: curl against running service

### Implementation Tasks

- Query contract (request/response shape)
- Stock lookup domain function
- HTTP endpoint wired to domain
- In-memory stock store fixture for example

---

## Milestone M2 — Operator can query any catalog item

### Product Value

The same query contract works for any SKU present in the catalog — not only the
fixture used in M1.

### Consumer

Warehouse operator using the inventory HTTP API.

### Observable Result

Multiple distinct SKUs return correct quantities from the shared stock store.

### Evidence

- Integration test: parameterized cases across catalog SKUs
- Operator smoke test: two different SKUs in one session

### Implementation Tasks

- Expand stock store fixture to full example catalog
- Regression tests for M1 cases

---

## Milestone M3 — Operator receives not-found for unknown items

### Product Value

An operator receives a clear, contract-defined response when a warehouse or SKU
does not exist — the API does not fail silently or return misleading quantities.

### Consumer

Warehouse operator using the inventory HTTP API.

### Observable Result

Unknown SKU returns `404` (or contract-defined not-found) with no quantity body.

### Evidence

- Integration test: unknown SKU and unknown warehouse cases
- Outcome certification: end-to-end operator query scenario (GR-13)

### Implementation Tasks

- Not-found mapping in domain and HTTP layer
- Outcome certification test entrypoint

---

## Closing the Intention

After M3 PASS:

1. **Intention Audit** confirms the operator can query stock end to end (GR-13).
2. **Human Review** approves the completed capability.
3. **Product Knowledge Synchronization** records verified knowledge.
4. The Intention is marked **Closed**.

The product now carries more institutional knowledge into its next Intention.
