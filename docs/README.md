# Website (GitHub Pages)

This folder contains the public website for the standard. It is **non-normative**:
it explains and links to the standard, but the specifications in
[`../specifications/`](../specifications/README.md) remain the source of truth.

Live site: <https://orivus.github.io/orivus-ai-product-framework/>

## Pages

| File | Page |
|------|------|
| `index.html` | Home |
| `problem.html` | The Problem |
| `why.html` | Why this standard exists (5-minute read) |
| `principles.html` | Core Principles (CP-1…CP-11 summary) |
| `article.html` | Essay: Why Product Knowledge Matters in AI-Native Engineering |
| `css/style.css` | Styling (sober, light/dark aware) |
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
