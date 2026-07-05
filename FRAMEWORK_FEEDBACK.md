# Framework Feedback

| Field | Value |
|-------|-------|
| Version | 0.1 |
| Status | **Frozen** — v0.1 Validated; observations only until v0.2 |
| Target | v0.2 evidence from independent reference implementations |

---

> **v0.1 is frozen.** Framework v0.1 is Validated (FV-001). Identity lives in [MANIFESTO.md](MANIFESTO.md). During active product builds, **do not modify the standard**. Record friction as observations. v0.2 evolves from reference implementation evidence — not new theory.

---

## Frozen Mode (v0.1)

1. **No framework changes** during active product delivery.
2. **Observations only** — one friction per OBS entry with evidence.
3. **Reference Validations** required only when agent-behavior changes are unavoidable.
4. **v0.2** born from evidence accumulated across real products, not speculative additions.
5. **Product energy** goes to reference products; the framework learns from them.

---

## Observation Template

```markdown
### OBS-NNN — Short title

| Field | Value |
|-------|-------|
| Product | <reference implementation> |
| Date | YYYY-MM-DD |
| Milestone / Phase | |

**Problem**

What friction appeared?

**Impact**

What did it cost — time, confusion, rework, scope creep?

**Evidence**

Milestone, file path, or specific situation.

**Recommendation**

What should v0.2 evaluate? (Not implemented in v0.1.)
```

---

## Observations

### OBS-001 — Human Review scope may be narrower than Intention prose

| Field | Value |
|-------|-------|
| Product | Reference implementation (voice platform) |
| Date | 2026-07-04 |
| Milestone / Phase | Platform audio adapter Intention — human smoke test, outcome certification |

**Problem**

The Intention prose described “audible speech.” The implementation used a synthetic-audio stand-in (deterministic PCM). Human Review could approve **physical hardware audio playback** (a clean 440 Hz sine) while **voice intelligibility** remained deferred to a future Intention. PKS Stage 3 required recording the scope boundary in evidence without rewriting the Intention.

**Impact**

Without explicit evidence language, agents or humans might claim full speech certification from a vertical slice that only proves the platform adapter path.

**Evidence**

Reference implementation milestone evidence and PKS Stage 3 note; Product Record scope table.

**Recommendation**

v0.2 SHOULD distinguish **Outcome Certification scope** from **deferred capabilities** in a standard Human Review / PKS template field (e.g. “certified” vs “explicitly deferred”).

---

### OBS-002 — Milestone batching violates execution model

| Field | Value |
|-------|-------|
| Product | Reference implementation (voice platform) |
| Date | 2026-07-05 |
| Milestone / Phase | Product-identity Intention — a run of consecutive milestones |

**Problem**

The agent implemented a run of consecutive milestones in a single pass, marked all of them PASS, and produced thin retroactive evidence. Framework v0.1 specified milestone-by-milestone execution but did not define an atomic **Milestone Transaction Protocol** with explicit anti-batching, LOCK phase, or precedence stack. The agent compensated by overloading AI Context with loop instructions — inverting Framework → AI Context order.

**Impact**

False PASS states; inadequate evidence; Human Review blocked on uncertified work; Intention docs bloated with framework teaching; rework required from the first batched milestone.

**Evidence**

A reference implementation plan marked a run of milestones PASS on the same day; the corresponding milestone evidence files were stubs; the implementation modules were batched together.

**Recommendation**

v0.1 operational refinement (not philosophy change): add normative [Milestone Transaction Protocol](specifications/MILESTONE_TRANSACTION_PROTOCOL.md), precedence stack, anti-batching AS-15, slim AI Context to milestone scope only. Pause affected Intention until framework and rules updated. Validate via [Framework Validation Protocol](specifications/FRAMEWORK_VALIDATION_PROTOCOL.md) before product resume.

---

### OBS-003 — Framework Validation validates agent behavior before product resume

| Field | Value |
|-------|-------|
| Product | Framework (meta) |
| Date | 2026-07-05 |
| Phase | FV-001 — Implement Logger |

**Problem**

After the v0.1 MTP fix, no proof existed that agents would follow one-milestone-per-transaction. Resuming the affected reference implementation immediately would repeat the same risk.

**Impact**

Without Framework Validation, framework changes are assumed working — the same failure mode as the earlier milestone batching.

**Evidence**

FV-001 executed M1→M4 incrementally in `validation/FV-001-sequential-milestone-loop/`; separate evidence per milestone; Living State updated between milestones; human-approved **PASS** for property Sequential Milestone Execution; Framework v0.1 **Validated**.

**Recommendation**

Formalize [Framework Validation Protocol](specifications/FRAMEWORK_VALIDATION_PROTOCOL.md) as mandatory gate before lifting PAUSED on product Intentions affected by framework changes. Rename to **Reference Validation** — certifies framework properties, not exercise artifacts.

---

### OBS-004 — Reference Validation certifies properties, not exercise artifacts

| Field | Value |
|-------|-------|
| Product | Framework (meta) |
| Date | 2026-07-05 |
| Phase | FV-001 human approval |

**Problem**

"Smoke test" framing implied validating the logger. PASS criteria must be framework-behavior-only or agents/humans will approve the wrong thing.

**Impact**

Misaligned approval criteria repeat the milestone-batching failure mode at framework level.

**Evidence**

Human approval of FV-001 with explicit property table (one milestone at a time, no batching, incremental evidence, etc.); `validation/FV-001-sequential-milestone-loop/`; `FRAMEWORK_VERSION.md` v0.1 Validated.

**Recommendation**

Adopt **Reference Validation** terminology; each FV declares a **Property**; evolution rule: agent-behavior framework changes require matching FV PASS before Framework Version Validated.
