# Website (GitHub Pages)

This folder contains the public website for the standard. It is **non-normative**:
it explains and links to the standard, but the specifications in
[`../specifications/`](../specifications/README.md) remain the source of truth.

Live site: <https://orivus.github.io/orivus-ai-product-framework/>

## Pages

| File | Page |
|------|------|
| `index.html` | Home — tagline, official flow diagram, credibility |
| `problem.html` | The Problem |
| `why.html` | Why this standard exists (5-minute read) |
| `quickstart.html` | Quick Start — first AI-native product in 5 minutes |
| `comparison.html` | Comparison with Scrum, DDD, OpenAPI, and related approaches |
| `principles.html` | Core Principles (CP-1…CP-11 summary) |
| `article.html` | Essay: Why Product Knowledge Matters in AI-Native Engineering |
| `assets/flow-diagram.svg` | Official product evolution flow diagram |
| `assets/og-image.png` | Open Graph / social preview (1200×630) |
| `assets/orivus-logo.png` | Favicon and brand mark |
| `css/style.css` | Styling (Orivus dark, system fonts) |
| `.nojekyll` | Disables Jekyll processing; serves files as-is |

## Enabling GitHub Pages (maintainers)

1. Repository → **Settings** → **Pages**.
2. **Build and deployment** → **Source**: *Deploy from a branch*.
3. **Branch**: `main`, **Folder**: `/docs`. Save.
4. Wait for the deployment, then verify the live URL above.

## Editing

Plain HTML and CSS. No build step, no JavaScript framework, no analytics. Keep it
minimal: the goal is that a new visitor understands the problem, the idea, and how
to start in under a minute. Avoid marketing language.

Tagline lockup:

- **Home hero** — full lockup (ORIVUS + Product Knowledge + governs implementation.)
- **Footer** — compact lockup on every page
- **Inner pages** — no lockup in hero (footer only)

Public category (website): **AI Product Engineering**. Normative principle:
**Product Knowledge governs implementation.** See Why §4 for the distinction.
