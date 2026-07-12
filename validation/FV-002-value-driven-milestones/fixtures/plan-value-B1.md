# Milestone B1 — Operator can query stock for known items

### Product Value

An operator can retrieve current quantity for a valid warehouse and SKU.

### Consumer

Warehouse operator using the inventory HTTP API.

### Observable Result

`GET /stock?warehouse=W1&sku=SKU-001` returns `200` with current quantity.

### Evidence

- GS-14 plan audit per [GS-14](../../../specifications/GOVERNANCE_SPECIFICATION.md#gs-14--verifiable-product-value-per-milestone)

### Implementation Tasks

- Query contract
- Stock lookup domain
- Repository fixture
- Integration tests
