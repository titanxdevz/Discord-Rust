# Rates and Defaults (edit these, or let the user override per project)

If a value is not set and matters, ask once. Otherwise use the placeholder and
label it "placeholder".

## Developer profile

- Experience: intermediate (configurable: junior / intermediate / senior)
- Model: independent freelancer, fixed-price contracts
- Typical work: web apps, dashboards, landing pages, Discord/Telegram bots, billing/hosting panel UI (e.g. Paymenter, Pterodactyl themes), API backends
- Home country: India. Reporting currency: INR.

## Core inputs

| Input | Default | Notes |
|---|---|---|
| Internal hourly cost | **Set by user** (placeholder INR 600/hr if unset, flagged) | Derive as target monthly income / billable hours per month |
| Billable hours per month | 90-110 | Not 160; selling, admin and gaps are unpaid |
| Productive coding hours per day | 4-6 (use 5) | Solo developer, with meetings and breaks |
| Non-coding overhead factor | 15% (range 10-20%) | Meetings, messaging, small unpaid revisions |
| Business buffer (on price) | 20% (floor 10% for Minimum) | Sales time, idle, tax, tools, support |
| Contingency (external dependencies only) | 0-10% | Not applied if PERT already covers the risk |
| AI-assist speedup cap | up to 25% on boilerplate/UI scaffolding only | Never on architecture, debugging, security |

## Contract defaults

- Revision rounds included: 2 (defined as changes within agreed scope)
- Bug-fix warranty: 15-30 days from handover (defects against agreed requirements, not new features)
- Quote validity: 7-15 days
- Payment structure by size:
  - Under INR 15,000: 50% advance, 50% before handover
  - Mid-size: 40% advance, 30% after core build, 20% after staging approval, 10% before production handover
  - Large or long: monthly or milestone-based with a kill fee
- Late payment: work pauses; delivery dates move accordingly
- IP transfer: on receipt of final payment

## Tax and fee reminders (verify live; do not hardcode rates)

- GST: add if registered; exports of services usually zero-rated with LUT.
- TDS: Indian business clients may deduct; section and rate vary. Confirm with a CA.
- US clients: W-8BEN for withholding exemption.
- Platform and payment fees: look up current Upwork, Fiverr, Freelancer, Wise, Payoneer, PayPal fees at estimate time.

## Reference hours (sanity checks only, realistic, solo intermediate, excluding overhead)

Adjust up for custom design, unclear specs, or unfamiliar stack.

| Feature | Typical hours |
|---|---|
| Static landing page, single, responsive | 8-20 |
| Multi-page marketing site (5-8 pages) | 25-60 |
| Auth (email + OAuth, sessions, reset) | 12-30 |
| CRUD module with validation | 6-16 |
| Admin dashboard (users, roles, tables, export) | 30-80 |
| Stripe / Razorpay integration with webhooks | 12-30 |
| Email/notification system | 6-16 |
| Real-time feature (websockets, live data) | 15-40 |
| Discord bot, basic command set | 20-50 |
| Discord bot, large (moderation, music, dashboard, DB) | 80-250 |
| Paymenter/billing theme redesign (storefront + portal) | 50-150 |
| Deployment, SSL, env, docs, handover | 6-16 |

## Categories to treat specially

- Discord/bot work: priced by command count, persistence, hosting needs, uptime and ongoing API changes.
- Panel/billing UI redesign: priced by number of unique screens, animation depth and framework constraints.
- Landing pages for clients: watch revision risk; cap rounds and require content before start.
