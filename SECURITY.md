# Security Policy

The Orivus AI Product Framework is a **specification**, not an executable
runtime. It ships documentation, normative specifications, and validation
evidence. It does not ship a service or library that processes untrusted input
at runtime.

Even so, a specification can carry risks worth reporting responsibly. This
policy explains what to report and how.

---

## Supported Versions

| Version | Status | Security fixes |
|---------|--------|----------------|
| 0.1 | Experimental — Frozen | Clarifications and errata only |

v0.1 is experimental. It is suitable for evaluation and reference
implementations, but it does not yet carry stability or support guarantees.

---

## What Counts as a Security Concern

Because this is a standard, "security" issues are mostly about guidance that
could lead adopters into unsafe engineering practices. Please report:

- normative guidance that could induce insecure implementations;
- governance gaps that could let unverified or malicious changes enter a
  conforming product undetected;
- validation or evidence rules that could be trivially forged to fake a PASS;
- ambiguous requirements that materially weaken product integrity guarantees;
- exposure of secrets, credentials, or private data inside repository content or
  examples.

Implementation-specific vulnerabilities (in a product that adopts the framework)
must be reported to that product's own maintainers, not here.

---

## Reporting a Vulnerability

Please report privately. **Do not open a public issue for security reports.**

1. Use GitHub's **private vulnerability reporting** ("Report a vulnerability")
   on this repository, if enabled; or
2. Contact the maintainers through the private channel published on the
   repository's GitHub organization profile.

Include:

- a clear description of the concern;
- the affected document(s) and section(s);
- the potential impact on adopters;
- a suggested remediation, if you have one.

---

## Response Expectations

As an experimental, community-driven standard, we aim to:

- acknowledge a report within a reasonable time;
- assess validity and impact;
- publish a clarification, erratum, or fix through the normal governed change
  process;
- credit reporters who wish to be acknowledged.

Timelines are best-effort during the v0.1 experimental phase.

---

## Disclosure

We prefer coordinated disclosure. Please give maintainers a reasonable
opportunity to address the concern before disclosing publicly.
