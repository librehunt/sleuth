---
type: bug
status: todo
created: 2025-11-25
priority: high
---

# Fix OnlyFans and Reddit False Positive Detection

## Problem
OnlyFans and Reddit return HTTP 200 with an HTML error page when a profile does not exist. The default `Site::parse_response()` only checks status codes, causing false positives.

## Evidence
- `curl -s https://onlyfans.com/nalinipriyauppari` returns a 200 page with "PAGE NOT FOUND".
- `curl -s https://www.reddit.com/user/nalinipriyauppari` returns a 200 page with "Sorry, there isn’t anything here".

## Solution
Implement custom `parse_response` for both sites that inspects the response body for known error phrases and override `http_method` to `GET` so the body is retrieved.

## Tasks
- Add body parsing logic and error phrase detection.
- Override `http_method` to return `"GET"`.
- Add unit tests for false positive detection.
- Update documentation and CHANGELOG.

## Acceptance Criteria
- Searching for a non‑existent OnlyFans or Reddit username reports "Not Found".
- All tests pass.

## Branch
`fix/onlyfans-reddit-false-positive`
