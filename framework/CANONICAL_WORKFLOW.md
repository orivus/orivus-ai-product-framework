# Orivus AI Product Framework — Canonical Workflow

| Field | Value |
|-------|-------|
| Version | 0.1 |
| Status | Experimental — Frozen |

The Canonical Workflow defines the standard engineering workflow prescribed by the Orivus AI Product Framework.

It summarizes the entire standard in one flow.

It illustrates how Product Knowledge is transformed into production software through governed AI-assisted engineering.

> **Product Knowledge governs implementation.**

The workflow is intentionally divided into two independent phases.

The first phase discovers the product.

The second phase delivers the product.

---

## 1. Phase 1 — Product Discovery

The objective of Product Discovery is to eliminate uncertainty before implementation begins.

```
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

Product Discovery is completed only once for a product, unless the product itself must be fundamentally redefined.

Once Product Foundation is achieved, Product Delivery may begin.

---

## 2. Phase 2 — Product Delivery

Product Delivery is the continuous phase in which the product is built, enhanced, and maintained through governed Intentions.

An Intention is the smallest governed unit of product delivery. It defines a measurable Product Outcome and the knowledge required for AI to implement that outcome without redefining the product.

An Intention may deliver the first product version, add a capability, optimize performance, refactor architecture, or eliminate technical debt.

Every Product Delivery cycle begins with a governed Intention.

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
Generate AI Context
        │
        ▼
AI Execution
```

Product Outcome is not an artifact.

It is the expected result of the Intention.

Planning transforms Product Knowledge into an executable AI Context.

Planning encompasses ADRs, RFCs, Implementation Plans, and other governed planning knowledge.

AI never executes directly from repository state or conversation history.

---

## 3. AI Execution Loop

Every Milestone follows the same autonomous execution cycle.

```
Current Milestone
        │
        ▼
AI Build
        │
        ▼
AI Validation Engine
        │
        ▼
Project VERIFIED?
        │
    ┌───┴────┐
    │        │
   NO       YES
    │        │
    ▼        ▼
Self-Healing Milestone Audit
    │        │
    └────────┘
```

The AI Validation Engine continuously attempts to bring the project into a VERIFIED state.

Mechanical implementation failures should be resolved autonomously.

Only VERIFIED project states may proceed to governance.

---

## 4. Milestone Governance

Once VERIFIED is achieved, AI executes the Milestone Audit.

```
Project VERIFIED
        │
        ▼
Milestone Audit
        │
        ▼
PASS?
    ┌───┴────┐
    │        │
   NO       YES
    │        │
    ▼        ▼
Remediation Update Implementation Plan
    │                │
    └────────────────┘
             │
             ▼
     Next Milestone
```

Each Milestone must produce:

- verified implementation;
- reproducible evidence;
- updated execution status.

The Implementation Plan becomes the execution state of the Intention.

Example:

```
M1  PASS
M2  PASS
M3  IN_PROGRESS
M4  PENDING
M5  PENDING
```

---

## 5. Intention Governance

After every Milestone reaches PASS, AI evaluates the complete Intention.

```
All Milestones PASS
        │
        ▼
Intention Audit
        │
        ▼
PASS?
    ┌───┴────┐
    │        │
   NO       YES
    │        │
    ▼        ▼
Remediation READY FOR HUMAN REVIEW
```

The Intention Audit verifies:

- **Product Outcome realization (GR-13)** — the user-visible capability exists; infrastructure-only delivery MUST FAIL audit
- Product Outcome definition alignment
- Product Foundation alignment
- Architecture integrity
- Product Contracts
- AI Context compliance
- Technical debt
- Evidence completeness
- Production readiness

### Infrastructure vs Product Outcome

Milestones may deliver means: sessions, warm-up, buffer pools, runtime constraints, certification harnesses, deterministic stand-ins.

Those MAY reach PASS without closing the Intention.

An Intention closes only when the **Product Outcome** named in the Intention document is observable by the user (GR-13).

---

## 6. Human Governance

Human intervention occurs only after AI has completed the entire engineering execution cycle.

```
READY FOR HUMAN REVIEW
        │
        ▼
Human Review
        │
        ▼
Approve?
    ┌───┴────┐
    │        │
   NO       YES
    │        │
    ▼        ▼
New Intention Approved
```

**Approved** is a human decision.

**Closed** is an administrative product state.

An Intention is not Closed until Product Knowledge Synchronization completes after Approval.

Humans govern the product.

Artificial Intelligence governs engineering execution.

---

## 7. Product Knowledge Synchronization

After Human Approval, the product MUST pass a **synchronization audit** before the Intention may be **Closed**.

Product Knowledge Synchronization is a mandatory audit — not a documentation task.

An Intention reaches **Closed** only when Approval and all four synchronization stages pass.

```
Approved (human decision)
        │
        ▼
Product Knowledge Synchronization (audit)
        │
        ├─ 1. Evidence Synchronization
        ├─ 2. Shared Knowledge Synchronization
        ├─ 3. Governance Synchronization (verify only)
        └─ 4. Repository Synchronization
        │
        ▼
Closed (administrative state)
        │
        ▼
Next Intention
```

Approval and Closed may coincide when PKS completes immediately after review — but they are distinct states.

### Stage 1 — Evidence Synchronization

Verify and update audit evidence: `audits/`, `INTENTION-evidence`, milestone evidence, certification evidence, validation results, test counts.

### Stage 2 — Shared Knowledge Synchronization

Verify and update persistent product knowledge: Product Shared Memory, Product Record, quality baseline.

### Stage 3 — Governance Synchronization

Verify alignment — do not rewrite. Confirm Intention → ADR → RFC → Implementation Plan → AI Context remain consistent with implementation. Contradictions require a new Intention or governed Planning update — not PKS correction.

### Stage 4 — Repository Synchronization

Verify and update Repository Knowledge: README files, audit indexes, docs navigation, links, status tables, verification counts.

Engineering knowledge governs the product and does not change because of hardening or hygiene work.

Evidence knowledge and Repository Knowledge represent current product state and MUST stay synchronized with the repository.

Every completed Intention leaves the product with more verified knowledge than before.

Future Intentions benefit from previous engineering work without reconstructing historical context.

---

## 8. Canonical Workflow

```
PRODUCT DISCOVERY
──────────────────────────────────────────────
Business Discovery (Optional)
                │
                ▼
Engineering Discovery
                │
                ▼
Product Foundation
═══════════════════════════════════════════════
               PRODUCT READY
═══════════════════════════════════════════════
PRODUCT DELIVERY
──────────────────────────────────────────────
Intention
        │
        ▼
Product Outcome
        │
        ▼
Planning
        │
        ▼
AI Context
        │
        ▼
AI Execution
        │
        ▼
AI Validation Engine
        │
        ▼
VERIFIED
        │
        ▼
Milestone Audit
        │
        ▼
PASS
        │
        ▼
Next Milestone
        │
        ▼
Intention Audit
        │
        ▼
READY FOR HUMAN REVIEW
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

---

## 9. Workflow Philosophy

The Canonical Workflow intentionally separates engineering execution from engineering governance.

Artificial Intelligence is responsible for planning, implementing, validating, self-correcting, auditing, and producing evidence.

Humans are responsible for approving completed product capabilities and governing the product.

The objective is not to increase human supervision.

The objective is to increase AI autonomy while preserving product integrity through governed knowledge, objective validation, and evidence-based engineering.

> **Product Knowledge governs implementation.**

This workflow enables AI-native product development where engineering scales through autonomous execution while product quality scales through governance.
