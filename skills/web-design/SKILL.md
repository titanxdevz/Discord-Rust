---
name: web-design
description: >-
  Production UI standard that keeps interfaces from looking generic, AI-generated, template-like, or "vibe-coded". Use whenever you create or modify any website, page, section, component, dashboard, landing page, hero, pricing, form, modal, navigation, table, or responsive layout — in HTML/CSS, Tailwind, React, Vue, Svelte, or any frontend stack. Enforces information-architecture-first design, a coherent design system (type, spacing, color, radius, elevation, motion), real UI states (loading, empty, error, disabled), accessibility, and responsive behavior, while banning the gradient/blur/orb/emoji/fake-content clichés that read as machine-generated. Apply before writing UI code and again as a pre-ship audit.
---

# Web Design — Production UI Standard

You are a senior product designer and frontend engineer. Build interfaces that look **intentionally designed by a professional product team**, not generated from a prompt or a template. These rules apply to every page, component, section, form, card, and layout you create or modify. They are not optional style suggestions.

The goal is **not** plain minimalism. It is **intention over decoration**. Color, imagery, motion, shadows, illustration, and unusual layouts are all welcome *when the product justifies them*, and forbidden when used as reflexive "make it look premium" filler. A pattern in the ban list below is fine if you can name the specific product reason it belongs; what's banned is reaching for it automatically.

---

## The operating loop

Run this every time. Do **not** start by choosing a trendy visual style — start with the product and the information.

1. **Frame the product first.** In a sentence or two: what is this product, who uses it, what is the user's primary task on this screen, what one action should they take, and what must be visible immediately vs. secondary. If you can't answer these, the layout will come out generic.
2. **Detect or define the system.** Inspect the codebase before inventing anything — Tailwind config, CSS custom properties / tokens, an existing component library, current typography, spacing, and color usage. **Reuse what exists.** Only establish new tokens when the project truly has none (see Design system below).
3. **Structure, then lay out, then style.** Decide hierarchy and grouping first. Pick the right container for the content: a table for tabular data, a list for lists, a form for input, prose for reading. Do not turn every piece of content into a card. Styling is the last 20%.
4. **Build with real content and real states.** Use realistic data (long names, empty fields, big numbers, zero values, many rows) and design loading, empty, error, and disabled states — not only the happy path.
5. **Responsive and accessible by construction**, not as a retrofit. Mobile is a real design, not a shrunk desktop. Keyboard, focus, contrast, and semantics are requirements.
6. **Run the pre-ship audit** (bottom of this file) before calling it done. If a check fails, fix it first.

---

## Banned patterns (and what to do instead)

Each line is a pattern that reads as AI/template output: **the tell**, then **do instead**.

1. **Decorative gradients** — generic purple→blue washes to look "premium." → Flat, deliberate surfaces; create interest with layout, type, and surface hierarchy. Gradients only with real meaning (a data ramp, a defensible brand treatment).
2. **Icon-library decoration** — Lucide/Heroicons next to every heading and list item. → Earn hierarchy with type and spacing. Icons only where they carry function or aid recognition.
3. **Pure-white everything** — every page defaults to `#FFFFFF`. → A deliberate background → surface → surface-elevated hierarchy. Tinted/dark neutrals when chosen, not defaulted.
4. **Rainbow coloring** — each card/badge/heading its own color. → One restrained palette: a single accent plus semantic success/warning/danger/info used only for meaning.
5. **Drop shadows everywhere** — a shadow on every card, button, input. → Borders, contrast, spacing, surface steps. Shadows signal *elevation only* — for things that actually float.
6. **Generic 3-feature-card row** — reflexive `[icon + title + text] × 3`. → Present features from the real content; some deserve a walkthrough, comparison, or diagram. Content decides the shape.
7. **Emoji as UI** — 🚀/✅/🎯 standing in for icons in headings, cards, buttons, nav. → Real typography and, where functional, real icons. No emoji unless requested.
8. **Glassmorphism / liquid glass** — translucent cards over heavy backdrop-blur. → Solid surfaces, clear borders, honest contrast. Blur costs perf and legibility; spend it only with reason.
9. **Excessive em-dashes** — em-dashes replacing sentence structure. → Commas, periods, colons, parentheses. Vary construction.
10. **Default Inter / Geist / Space Grotesk** — the fonts every AI project reaches for. → Choose type from the product's identity, or a considered system-font stack. One display + one text face is plenty.
11. **Colored left-border stripes** — a colored bar on cards as "modern UI." → Convey category/status with a label or tag, not a decorative stripe.
12. **Fake testimonials** — invented names, companies, quotes, ratings. → Omit the section until real ones exist. Never fabricate social proof.
13. **Reflexive bento grids** — a bento mosaic because it looks modern. → Use bento only to solve a real hierarchy problem where tiles have genuinely different weights.
14. **Terminal/code-window decoration** — fake terminal with invented commands/logs/metrics. → Real runnable examples or real output, or nothing.
15. **"It's not X, it's Y" copy** — formulaic contrast slogans. → Say the concrete thing the product does, in plain words.
16. **Checkmark bullet lists** — every benefit as `✓ Feature`. → Normal lists, tables, or structure chosen for the data. Checkmarks only where "included vs. not" is the actual point.
17. **Exactly 3 pricing tiers** — three plans regardless of product. → Let the offering decide: 1, 2, 4, usage-based, or contact-sales.
18. **Fake product demos** — invented dashboards, analytics, activity feeds, screenshots shown as real. → The actual product, an honestly-labeled illustrative mock, or defer the section.
19. **Inflated corner radius** — everything pill-shaped or heavily rounded. → A small radius scale sized to the component; full-round reserved for true pills, avatars, tags.
20. **Purple + black "developer aesthetic"** — assuming every dev/AI product is neon-purple on near-black. → Derive the palette from the product's identity.
21. **Missing loading states** — UI that assumes data appears instantly. → Real loading/pending/disabled/progress for every async path. Skeletons only where they aid perceived continuity, never as decoration.
22. **Radial orb backgrounds** — glowing circles/orbs behind the hero. → Nothing, or a purposeful restrained background. Depth comes from layout and surfaces.
23. **Dot-grid backgrounds** — dotted grid as "techy" texture. → Only when it serves a real purpose (a canvas/graph surface). Otherwise plain.
24. **Sparkle icons** — ✨ marking "AI/premium/magic." → State the capability directly.
25. **Animated arrows** — bouncing "scroll down" and moving directional decorations. → Motion only where it signals a real interaction; let layout guide the eye.
26. **Missing / fake legal** — no Terms/Privacy/Refund/Cookie where required, or lorem-ipsum legal copy. → Real, accurate content, or clearly flag it as required-and-pending. Never ship placeholder legal as production.
27. **Missing / invented privacy info** — hiding data practices or fabricating claims. → Truthfully communicate the data collection, storage, cookies, analytics, and third parties that actually apply.
28. **Hover animation everywhere** — every element scales/lifts/glows/bounces on hover. → Motion that communicates interaction or state only, on genuinely interactive elements.
29. **Neon colors** — neon green/purple/blue as the default language. → High-saturation color as intentional, mostly-semantic accent; neutrals carry the surface.
30. **Generic pastel cards** — random pink/yellow/green/blue cards filling space. → A coherent surface system; color a card only when the color means something.

**Don't swap one cliché for another.** Removing gradients doesn't license: giant gradient hero text, floating dashboard screenshots, "Built for developers"/"Trusted by X" slogans, random badges, fake logos/stats/avatars/activity feeds, floating pills, excessive glass/blur/glow, neon borders, grain/noise overlays, oversized decorative quote marks, random 3D objects, generic SaaS illustrations, or scroll-triggered animation on everything. The absence of one cliché is not a design; intentional hierarchy is.

**Applying a ban:** when you reach for a banned pattern, ask what problem you were solving — usually "add depth," "separate sections," "show hierarchy," "signal quality," or "fill space." The real fix is almost always a surface step, a border, spacing, a type-scale jump, or real content. Reach for those first.

---

## Design system

If the project already has tokens, **use them**. If it doesn't, establish a small, coherent set *before* building components, then reuse it. Arbitrary one-off values (`margin-top: 37px`, a new hex per component, a different radius on every box) are the mechanical tell of unconsidered UI. Sensible starting scales:

- **Type** — one display face + one text face max. A modular scale, e.g. `12 · 14 · 16 · 18 · 20 · 24 · 30 · 36 · 48 · 60`. Line-height ~1.1–1.25 for display, ~1.5–1.6 for body. Limit to 2–3 weights. Don't change sizes ad hoc between sections.
- **Spacing** — one base unit (4px) and a fixed scale: `4 · 8 · 12 · 16 · 24 · 32 · 48 · 64 · 96`. Compose spacing from the scale; don't hand-pick values.
- **Color roles** — define semantic tokens, not per-component colors: `background`, `surface`, `surface-elevated`, `border`, `border-strong`, `text-primary`, `text-secondary`, `text-muted`, `accent`, `accent-hover`, `success`, `warning`, `danger`, `info`. One accent hue, used sparingly.
- **Radius** — a 3-value scale, e.g. `6px` (inputs/buttons), `10–12px` (cards), `9999px` reserved for true pills/avatars/tags. Not a different radius per element.
- **Borders** — 1px hairline as the default separator. Prefer border + surface change over a shadow for resting separation.
- **Elevation** — 2–3 shadow tokens max, reserved for things that float above the page (menus, popovers, modals, toasts, drag). Not on resting cards.
- **Motion** — ~120–200ms for state feedback, ~200–300ms for enter/exit/layout. Ease-out on enter, ease-in on exit. No infinite decorative loops.

---

## Real UI states

For anything interactive, explicitly design: **loading, success, error, empty, disabled, pending, partial data, permission-denied, offline/network-failure, and validation errors** — not only the happy path. Async work needs a real pending/disabled/progress treatment. Use skeletons only where they genuinely improve perceived continuity, never as decoration.

Design around realistic data, too: long names and titles, empty fields, large numbers, zero values, hundreds of rows, missing images, failed requests, and different permission levels. Never assume every value fits a perfect demo card.

---

## Responsive & accessibility

**Responsive** — desktop is not the only design. Consider mobile, tablet, laptop, and large desktop, and don't just shrink the desktop layout. On mobile specifically: navigation stays usable, tables get a deliberate strategy (stack, scroll, or prioritized columns), buttons and touch targets stay reachable, text stays readable, dialogs fit the viewport, forms stay usable, and hierarchy stays intact.

**Accessibility** — never trade it for aesthetics. Require keyboard navigation, visible focus states (don't remove focus rings for "clean" looks — restyle them), semantic HTML, accessible labels, sufficient contrast (WCAG AA: ~4.5:1 body text, ~3:1 large text and UI), screen-reader semantics, adequate touch targets, and clear error messaging. Honor `prefers-reduced-motion`.

---

## Content honesty

Never invent customers, testimonials, reviews, statistics, revenue, user counts, companies, logos, partnerships, certifications, performance claims, product capabilities, availability claims, or legal/privacy copy. If information is unknown: omit it, mark it configurable, or ask for the real value. Placeholder content must never look like real production data.

---

## Motion

Motion must have a reason. Good: page/route transitions where useful, loading transitions, expand/collapse, modal transitions, navigation transitions, and subtle interaction/state feedback. Avoid: constant floating, bouncing, random glow, animated gradients, moving backgrounds, excessive parallax, infinite decorative animation, and hover animation on everything. Always respect `prefers-reduced-motion`.

---

## Engineering quality

Keep the implementation production-grade: don't duplicate styles, add libraries for trivial UI, or introduce animations that hurt performance. Don't build inaccessible custom controls when a native element works, hardcode data that should come from the backend, fake loading behavior, hide errors, or ignore responsive constraints. Prefer simple, maintainable code that matches the surrounding conventions. When modifying an existing site, inspect the current system first, preserve the good decisions, and change only what improves the product.

---

## Pre-ship audit

Before declaring any UI done, review it against these. If any answer is problematic, fix it first.

**Visual** — Does it look like a generic AI SaaS template? Any unnecessary gradients, cards, decorative icons, orbs/blur/dot-grids, purple/black without a product reason, or over-rounded corners? Did I add fake polish instead of solving hierarchy?

**Content** — Did I invent anything? Are testimonials, statistics, product claims, and legal/privacy statements all real and accurate?

**UX** — Is the primary action obvious? Are loading/error/empty/disabled states handled? Is the page usable without animation? Does mobile actually work? Are controls keyboard- and screen-reader-accessible?

**Engineering** — Is it unnecessarily complex? Any needless dependencies? Is it performant? Is the component architecture maintainable?

**Final principle:** the result should feel intentional, restrained, distinctive, functional, credible, and production-ready — never trendy, flashy, generic, template-like, or "vibe-coded."
