# Orivus AI Product Framework — Product Lifecycle

| Field | Value |
|-------|-------|
| Version | 0.1 |
| Status | Experimental — Frozen |

Orivus AI Product Framework distinguishes two independent but connected lifecycles.

The first lifecycle discovers the product.

The second lifecycle delivers the product.

These lifecycles have different objectives, different outputs, and different governance rules.

They must never be confused.

---

## 1. Product Discovery Lifecycle

Product Discovery exists to eliminate uncertainty before software is built.

Its objective is to establish a Product Foundation that provides sufficient knowledge for implementation.

This lifecycle is executed when creating a new product or when a major product redesign requires redefining its foundations.

```
Product Discovery
        │
        ▼
Business Discovery (Optional)
        │
        ▼
Engineering Discovery (Required)
        │
        ▼
Product Foundation
══════════════════════════════
      PRODUCT READY
══════════════════════════════
```

**Business Discovery** defines business knowledge when required.

**Engineering Discovery** defines product knowledge.

Once Product Foundation is achieved, Product Delivery may begin.

---

## 2. Product Delivery Lifecycle

Once a Product Foundation exists, the product is delivered through governed Intentions.

Every Intention represents one meaningful product change — the first version, a new capability, an optimization, a refactor, or a correction.

Each Intention follows the same lifecycle regardless of product size or technology.

```
Intention
        │
     defines
        ▼
Product Outcome
        │
implemented through
        ▼
Planning
        │
        ▼
Implementation
        │
        ▼
Validation
        │
        ▼
Certification
        │
        ▼
Intention Audit
        │
        ▼
Human Review
        │
        ▼
Approved
        │
        ▼
Product Knowledge Synchronization
        │
        ▼
Closed
        │
        ▼
Next Intention
```

Every completed Intention becomes part of the delivered product.

**Approved** records the human decision.

**Closed** records the administrative product state after Product Knowledge Synchronization completes.

**Product Outcome** defines what must be true when the Intention completes — it is the expected result, not an artifact.

**Planning** transforms Product Knowledge into governed planning knowledge and AI Context.

**Implementation** executes Milestones within governed scope.

**Validation** brings the project to VERIFIED through the AI Validation Engine.

**Product Knowledge Synchronization** incorporates verified knowledge through a four-stage audit: Evidence, Shared Knowledge, Governance (verify only), and Repository.

---

## 3. Continuous Product Delivery

The product is continuously delivered through independent Intentions.

```
Product Foundation
        │
        ▼
Intention 001
        │
        ▼
Completed
        │
        ▼
Intention 002
        │
        ▼
Completed
        │
        ▼
Intention 003
        │
        ▼
Completed
        │
        ▼
...
        │
        ▼
Product Delivery
```

Each completed Intention contributes:

- new product capabilities;
- verified engineering knowledge;
- architectural improvements;
- implementation experience;
- Product Shared Memory.

The Product Foundation remains the architectural reference throughout the lifecycle.

Intentions deliver product changes without redefining product identity.

If an Intention requires changing the Product Foundation itself, implementation must stop and return to Product Discovery.

---

## 4. Lifecycle Philosophy

The framework intentionally separates discovering a product from delivering a product.

Products should be understood once.

Products should be delivered continuously.

Every delivery cycle begins with an Intention.

Every Intention produces governed knowledge.

Every completed Intention strengthens the product for future delivery.

This separation allows AI to implement product capabilities autonomously while preserving long-term architectural integrity, product identity, and shared engineering knowledge.

> **Product Knowledge governs implementation.**
