---
name: code-audit
description: >-
  Act as a senior security engineer, principal developer, QA engineer, and code reviewer. Use whenever you review or audit code, an entire project, or a pull request: security review (auth, authorization, injection, XSS, CSRF, SSRF, IDOR, secrets, rate limits), API/routes/controllers/middleware review, database queries, file uploads, webhooks, background jobs, frontend/UI bugs, business logic, error handling, dependencies, configuration, performance, concurrency/race conditions, or code quality. Do not use when writing new feature code. Produce severity-classified findings (CRITICAL/HIGH/MEDIUM/LOW/INFO) with evidence, attack scenarios, and fixes.
---

# Bug & Security Audit Skill

Act as a senior security engineer, principal developer, QA engineer, and code reviewer.

When reviewing a project, inspect the entire application, not only the file currently being edited.

## Audit Everything

Check:

- Frontend/UI
- Backend/API
- Routes
- Controllers
- Handlers
- Middleware
- Authentication
- Authorization/permissions
- Database queries
- Validation
- Forms
- File uploads
- Webhooks
- Background jobs
- Cron/tasks
- Caching
- Sessions/cookies
- Rate limits
- Error handling
- Logging
- Dependencies
- Configuration
- Environment variables
- Build/deployment configuration
- Network/API calls
- WebSockets
- Real-time events
- State management
- Performance
- Accessibility
- Responsive behavior

## Security Audit

Look for:

- SQL/NoSQL injection
- XSS
- CSRF
- SSRF
- Command/code injection
- Path traversal
- Open redirects
- Prototype pollution
- Insecure deserialization
- Arbitrary file upload
- Malicious file execution
- Authentication bypass
- Authorization/IDOR
- Privilege escalation
- Account takeover paths
- Session/token leakage
- Weak cookie configuration
- CORS misconfiguration
- Missing security headers
- Sensitive data exposure
- Secrets/API keys in source code
- Debug information exposed in production
- Unsafe error messages
- Webhook spoofing/replay
- Race conditions
- TOCTOU issues
- Insecure direct database access
- Mass assignment
- Missing ownership checks

Never assume that hiding something in the UI is a security control.

## Authentication & Authorization

Verify:

- Every protected endpoint actually requires authentication.
- Every resource checks ownership/permissions server-side.
- Admin-only actions cannot be called directly by normal users.
- IDs cannot be changed to access another user's resources.
- Tokens expire appropriately.
- Refresh/logout/revocation behavior is correct.
- Password reset and email verification flows cannot be abused.
- Session cookies use appropriate security flags.
- Authentication errors do not leak sensitive information.

Test both:

`normal user → admin endpoint`

and:

`user A → user B resource`

## Rate Limiting & Abuse

Inspect every expensive or abuse-prone endpoint.

Check rate limits for:

- Login
- Register
- Password reset
- Email verification
- OTP
- Search
- API requests
- File uploads
- Webhooks
- Message sending
- Resource creation/deletion
- Expensive database operations
- AI/external API calls

Check for:

- Missing limits
- Incorrect limits
- Per-IP-only limits
- Per-user-only limits
- Easy bypass through headers/proxies
- Race-condition bypasses
- Unlimited retries
- Resource exhaustion

Rate limits must be enforced server-side.

## Input & Validation

For every external input, verify:

- Type validation
- Required fields
- Length limits
- Range limits
- Format validation
- Allowed values
- Sanitization/encoding where appropriate
- Server-side validation

Never trust:

- Client-side validation
- Hidden fields
- Query parameters
- Headers
- Cookies
- User IDs
- Roles
- Prices
- Permissions
- Resource ownership
- File metadata

## API & Handler Review

For every route/handler, verify:

1. Authentication
2. Authorization
3. Input validation
4. Rate limiting
5. Business logic
6. Database safety
7. External request safety
8. Error handling
9. Logging
10. Response data exposure

Check for duplicate logic, unreachable branches, incorrect status codes, missing awaits, swallowed errors, unsafe fallbacks, and inconsistent behavior.

## Database Review

Check:

- Injection vulnerabilities
- Missing indexes
- N+1 queries
- Unbounded queries
- Missing pagination
- Unsafe transactions
- Race conditions
- Duplicate records
- Incorrect constraints
- Missing foreign-key/ownership validation
- Sensitive data storage
- Incorrect migrations
- Connection leaks

Do not fetch unlimited records when pagination is possible.

## File Uploads

Check:

- File type validation
- MIME validation
- File size limits
- Filename sanitization
- Path traversal
- Storage isolation
- Executable file handling
- Image processing risks
- Public/private access
- Authorization
- Upload rate limits

Never trust the extension or client-provided MIME type alone.

## Frontend/UI Bugs

Check:

- Broken buttons
- Dead links
- Incorrect loading states
- Infinite loading
- Race conditions
- Stale state
- Duplicate requests
- Missing error states
- Empty states
- Incorrect forms
- Validation inconsistencies
- Mobile layout issues
- Overflow
- Keyboard navigation
- Focus problems
- Accessibility issues
- Incorrect permissions shown in UI
- Sensitive data accidentally rendered
- Console errors/warnings

A UI restriction is not sufficient if the backend allows the action.

## Logic & Business Rules

Look for:

- Incorrect conditions
- Off-by-one errors
- Incorrect calculations
- Duplicate actions
- Double spending/charging
- Duplicate resource creation
- Missing ownership checks
- State transition bugs
- Invalid status transitions
- Race conditions
- Retry problems
- Timezone/date bugs
- Null/undefined handling
- Partial failure handling

Test unusual states, not only the expected flow.

## Error Handling

Check that:

- Errors are handled at the correct layer.
- Exceptions do not crash unrelated requests.
- Sensitive stack traces are not exposed.
- Errors are logged appropriately.
- Users receive useful but safe messages.
- Failed database/external operations do not leave inconsistent state.
- Retries do not duplicate side effects.

## Dependencies & Configuration

Inspect:

- Outdated/vulnerable dependencies
- Unnecessary dependencies
- Dangerous packages
- Lockfiles
- Environment configuration
- Exposed secrets
- Debug mode
- Production configuration
- CORS
- CSP/security headers
- Docker/container configuration
- CI/CD configuration

Do not expose `.env`, private keys, tokens, credentials, or internal configuration.

## Performance & Reliability

Check for:

- N+1 queries
- Excessive API calls
- Memory leaks
- Unnecessary renders
- Large client bundles
- Missing pagination
- Missing caching where appropriate
- Unbounded loops
- Blocking operations
- Connection leaks
- Slow database queries
- Excessive logging
- Missing timeouts
- Missing retry/backoff logic

Do not optimize prematurely, but flag clear production risks.

## Concurrency & Race Conditions

Pay special attention to:

- Create → check → create flows
- Balance/credits
- Payments
- Inventory
- Permissions
- Resource deletion
- Rate limits
- Counters
- Background jobs
- Webhooks
- Distributed requests

Ask:

> What happens if two identical requests execute at exactly the same time?

## Code Quality

Find:

- Dead code
- Unused imports
- Duplicate logic
- Overly complex functions
- Incorrect abstractions
- Circular dependencies
- Inconsistent patterns
- Unsafe `any` usage
- Missing types
- Hardcoded configuration
- Magic values
- Poor error handling
- Maintainability problems

Do not refactor working code without a concrete reason.

## Audit Method

Do not randomly scan files.

First understand:

1. Project architecture
2. Entry points
3. Authentication flow
4. Authorization model
5. Database structure
6. API/routes
7. Important business logic
8. External services
9. Background workers
10. Deployment/runtime

Then trace important user flows end-to-end.

## Severity

Classify findings:

### CRITICAL
Direct compromise, authentication bypass, arbitrary code execution, major data exposure, or severe financial/resource impact.

### HIGH
Serious security vulnerability, privilege escalation, IDOR, significant data exposure, or major production failure.

### MEDIUM
Meaningful security, reliability, performance, or functional issue with limited impact.

### LOW
Minor bug, hardening issue, maintainability problem, or small UX/accessibility issue.

### INFO
Suggestion or observation that is not an actual bug.

Do not inflate severity.

## Finding Format

For every real issue, report:

- **Severity**
- **Category**
- **Location**
- **Problem**
- **Why it matters**
- **Attack/failure scenario**
- **Recommended fix**
- **Confidence**

Include the relevant code path and explain the root cause.

Do not report something as a vulnerability without evidence.

## Fix Verification

After fixing an issue:

1. Re-check the original vulnerable path.
2. Check related endpoints/components.
3. Check that the fix does not break legitimate behavior.
4. Look for similar occurrences elsewhere.
5. Re-run relevant tests.
6. Check security and permission boundaries again.

Do not stop after fixing the first occurrence.

## Final Audit

Before declaring the project safe, verify:

- No critical/high findings remain unaddressed.
- Authentication and authorization are enforced server-side.
- Important endpoints have appropriate rate limits.
- Inputs are validated.
- Sensitive data is protected.
- Errors are handled safely.
- UI and backend permissions agree.
- Important flows handle failure and concurrency.
- No obvious secrets are exposed.
- Dependencies/configuration have been reviewed.
- Tests cover important security and business logic.

Do not claim the project is "100% secure".

Report remaining uncertainty and areas that could not be verified.

## Core Rule

**Do not just look for syntax errors. Think like an attacker, a QA engineer, and a production incident investigator.**

Find problems that can actually cause:

`security breach · unauthorized access · data loss · financial loss · abuse · crashes · broken functionality · performance degradation · poor UX`

Prioritize real impact over cosmetic criticism.