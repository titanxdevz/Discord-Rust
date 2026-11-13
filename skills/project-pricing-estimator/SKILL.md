---
name: project-pricing-estimator
description: >
  Estimate and price freelance web, app, bot and dashboard projects: effort
  hours, timeline, live competitor research, fixed-price quotes (minimum /
  recommended / premium), client-facing quotations and negotiation replies.
  Use whenever the user shares a client message, brief, feature list, repo or
  budget and asks "how much should I charge", "quote this", "estimate this",
  "is this budget enough", "client says too expensive", or wants a proposal,
  quotation or price for development work, even if they never say "estimate".
---

# Project Pricing Estimator V3

Help an independent developer quote **profitable, competitive, defensible
fixed-price** projects. Default currency INR; any ISO currency supported.
Developer profile and rates live in `references/rates-and-defaults.md` and are
overridable by the user. Never assume an agency, and never discount work
because the developer is in India.

## Reference files (read only when needed)

| File | Read when |
|---|---|
| `references/rates-and-defaults.md` | Always, at the start (hourly cost, hours/day, buffer, tax, warranty defaults, reference hours) |
| `references/market-research.md` | Doing market research or normalising comparables |
| `references/report-template.md` | Producing a Full Report |
| `references/quotation-template.md` | Writing the client-facing quotation |
| `references/negotiation.md` | Client pushback, or user asks for replies |

## Modes

- **Quick Quote**: short or small request, budget under roughly INR 25,000, or user says "quick". Output: assumptions, hours, price range, one recommended price, short quote. 3-5 searches.
- **Full Report**: larger, ambiguous or high-value projects, or user asks for a report. Use `report-template.md`. 8-12 searches, 5-8 comparables.
- If unclear, start with Quick Quote and offer the Full Report.

## Workflow

1. **Intake.** Accept any format (chat paste, email, PDF, screenshot, link, repo, feature list). Extract name, category, client country, audience, objective, deliverables, references, existing stack, deadline, stated budget. Redact personal details (names, phone numbers) from anything you echo back.
2. **Classify requirements** as Confirmed, Implied (needed to deliver a confirmed feature), Optional (never in the base price) or Unknown. Never turn an assumption into a confirmed feature.
3. **Clarify only if it matters.** Ask at most 3 questions, only for gaps that would move price or timeline by about 25% or more. Otherwise proceed with written assumptions. If the brief is too thin, give a price **range** and say what narrows it.
4. **Estimate effort** (below).
5. **Research the market** (below).
6. **Calculate price** (below), reconcile with market, run the self-check.
7. **Deliver** the internal analysis and the client quotation as **two separate blocks**. The client block never contains internal hourly cost, buffer, minimum price or negotiation limits.

## Effort estimation

Break the work into only the relevant modules: discovery/architecture, UI/UX,
frontend, backend, database, integrations, admin, QA, deployment/handover,
client communication. Also consider when relevant: SEO, CMS, data migration,
i18n, e-commerce, real-time features, mobile/PWA, DevOps, accessibility,
compliance (GDPR, India DPDP Act).

Per task record: name, complexity, O / R / P hours (optimistic, realistic,
pessimistic), dependencies, assumptions, key risk.

- **Expected hours per task = (O + 4R + P) / 6** (PERT). This is the only place
  uncertainty is priced, so do not add a separate risk percentage on top of it.
- **Overhead**: add the non-coding factor from the defaults (meetings,
  messages, unpaid small revisions), typically 10-20%.
- **AI-assisted coding**: credit speedup only for boilerplate, CRUD and UI
  scaffolding. Do not credit it for architecture, debugging, security review,
  integration quirks or client coordination. Cap the credit at what the
  defaults file allows, and say what you credited.
- **Existing codebase**: estimate an audit first, then apply a tech-debt factor
  of 1.2-1.5 to touched modules based on code quality.
- Use `references/rates-and-defaults.md` reference hours as sanity checks, not as
  fixed answers.

Do not over-engineer simple projects. A landing page does not need a CI/CD
pipeline line item.

## Market research

Run fresh searches every time; never reuse an old market figure as current.
Generate varied queries from features, stack, category and deliverables, not
the project's generic name. Details, sources and fallbacks are in
`references/market-research.md`.

Rules that always apply:

- Record per comparable: platform, URL, service, price, currency, package,
  scope, delivery time, evidence level, access date.
- Evidence levels: E1 verified completed price; E2 client-posted budget;
  E3 advertised freelancer package; E4 agency published price or estimate;
  E5 general benchmark or discussion.
- Marketplaces often block or hide pages. Use what is visible, cap the evidence
  level at what you actually saw, and mark unreachable pages "unavailable".
- Never fabricate a listing, URL, price or exchange rate. If fewer than three
  usable comparables exist, say so and label market data provisional.
- A starting price is not a final price; an hourly rate is not a project budget.

## Price calculation

Everything in INR first, then convert. Use the **real current date** from the
conversation for access dates, validity and timelines.

```
Billable hours   = sum(expected task hours) x (1 + overhead%)
Labour cost      = Billable hours x internal hourly cost
Direct expenses  = one-time project costs (APIs, licences, assets, contractors)
Contingency      = flat 0-10% of labour, ONLY for external dependencies
                   (unknown APIs, client content, third-party access)
Project cost     = Labour + Direct expenses + Contingency

Recommended      = Project cost / (1 - buffer% - fee%)
Minimum          = Project cost / (1 - 10% - fee%)        (floor buffer 10%)
Premium          = Recommended + (cost of added extras / (1 - buffer% - fee%))
```

- **Hourly cost** is the developer's own pay target per billable hour (see
  defaults for derivation). It is not the client rate.
- **Buffer** is not "profit" in the agency sense. For a solo developer it covers
  unpaid sales time, idle gaps, taxes, tools, and post-launch support.
- **fee%** = payment/platform/transfer/conversion costs as a share of the price
  (look up current Upwork, Fiverr, Wise, Payoneer, PayPal rates live).
- **Taxes are shown separately.** GST is added on top of the price if the
  developer is registered (exports are normally zero-rated under LUT). TDS
  withheld by Indian business clients and foreign withholding forms (e.g.
  W-8BEN for US clients) affect cash received, not the quoted price. Say these
  need confirming with the developer's CA.
- **Premium** includes only real extras (documentation, longer warranty,
  performance work, monitoring), each priced. Never pad.
- Show INR and the client currency, the exchange rate, its source and date.
  Lock the rate for the quote's validity period; the client bears FX swings
  after acceptance.

### Reconcile cost price with the market

Normalise comparables for scope, convert to INR on one date, drop outliers,
then take the median and the interquartile range.

| Situation | Action |
|---|---|
| Cost price above market P75 | Check scope and differentiators. Offer a reduced-scope option rather than cutting below Minimum. |
| Cost price inside market band | Quote Recommended. |
| Cost price below market P25 | Move toward the band if the developer's quality or speed justifies it. Do not quote below cost-based price just to match the market. |
| Market maximum below Minimum | Say the project is not viable at that market; recommend scope cut, hourly pricing, or declining. |

### Self-check (must pass before output)

- Minimum >= Project cost, and Minimum <= Recommended <= Premium.
- **Effective hourly rate** = net price after fees / total hours including
  overhead. Report it. If it is below the developer's hourly cost, flag a loss
  explicitly.
- Every figure traces to a calculation or a cited source.
- Client quote contains no internal numbers.

## Timeline

```
Working days = Billable hours / productive hours per day   (default in defaults file)
Calendar days = working days x 7/5 + client-wait days + deployment/buffer days
```

Give three windows: **Earliest**, **Recommended**, **Risk-adjusted**. Never
promise a date that depends on an unconfirmed client dependency (content,
credentials, approvals); state the dependency in the quote.

## Budget fit

If the client gave a budget, classify: sufficient, workable with reduced scope,
below sustainable cost, or unrealistic. When short, offer scope reduction (which
features to defer, with hours saved) instead of lowering quality or pay.

## Other pricing models

- **Hourly or retainer**: use when scope is open-ended. Price from hourly cost / (1 - buffer% - fee%), set a monthly hour cap, and state what is excluded.
- **Maintenance**: always separate from build. Never promise unlimited support in a fixed price. State duration, response time and exclusions.

## Hard rules

1. Never fabricate market data, listings, prices, or exchange rates. Say "calculated price" for your own numbers and never present them as market data.
2. Never present an advertised price as a completed transaction.
3. Never skip testing, deployment, or client-communication hours.
4. Never recommend a loss-making project without stating the loss.
5. Never expose internal limits in the client block.
6. Do not use a flat country multiplier. Price from scope, value, local competition and the client's company size.
7. If browsing is unavailable: still estimate effort and cost price, mark market research unavailable and the price provisional, and list what research remains.
8. End every completed estimate with **one clear recommended fixed price**, the reason it is justified, and what must stay excluded to protect it.
