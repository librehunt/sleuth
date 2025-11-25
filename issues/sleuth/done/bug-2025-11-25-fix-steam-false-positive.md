---
type: bug
status: done
created: 2025-11-25
priority: high
---

# Fix Steam and Medium False Positive Detection

## Problem
Both Steam and Medium return HTTP 200 with error pages instead of HTTP 404 when a profile doesn't exist. This causes false positives where non-existent usernames are reported as "Found".

## Evidence

### Steam
When searching for username `lsh0x`:
- **Expected**: Steam should return "Not Found"
- **Actual**: Steam returns "Found" with URL `https://steamcommunity.com/id/lsh0x`
- **Reality**: The page shows "Profil spécifié introuvable" (Profile not found)

![Steam Error Page](/Users/lsh/.gemini/antigravity/brain/9b60167b-b81e-410a-9451-67e6dc39ed5b/uploaded_image_1764079138634.png)

### Medium
When searching for username `lsh0x`:
- **Expected**: Medium should return "Not Found"
- **Actual**: Medium returns "Found" with URL `https://medium.com/@lsh0x`
- **Reality**: The page shows "PAGE NOT FOUND - 404 - Out of nothing, something."

![Medium Error Page](/Users/lsh/.gemini/antigravity/brain/9b60167b-b81e-410a-9451-67e6dc39ed5b/uploaded_image_1764079214010.png)


## Root Cause
The default `Site::parse_response()` implementation only checks HTTP status codes:
- 200-299 → Found
- 404 → Not Found

Both Steam and Medium return 200 even for non-existent profiles, with error messages in the response body.

## Solution
Implement custom `parse_response()` for both `SteamChecker` and `MediumChecker` that:
1. Checks HTTP status code (200 = continue, 404 = not found)
2. If status is 200, parse the response body
3. Look for error indicators in the body

**Steam error indicators:**
- French: "Profil spécifié introuvable"
- English: "The specified profile could not be found"

**Medium error indicators:**
- "PAGE NOT FOUND"
- "404"
- "Out of nothing, something."

## Implementation Tasks
- [ ] Update `SteamChecker::parse_response()` to check response body
- [ ] Update `MediumChecker::parse_response()` to check response body
- [ ] Add test cases for false positive detection (both sites)
- [ ] Verify fix with real profiles (both existing and non-existing)
- [ ] Document the custom parsing logic

## Acceptance Criteria
- Searching for non-existent Steam username returns "Not Found"
- Searching for non-existent Medium username returns "Not Found"
- Searching for existing usernames returns "Found"
- Tests cover both scenarios for both sites

## Branch
`fix/steam-false-positive`

## Related
This demonstrates the value of the recent refactoring to use `Arc<dyn Site>` and custom `parse_response()` methods (PR #6).
