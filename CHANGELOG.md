# Changelog

## [Unreleased]

### Changed - Issues #1722, #1723, #1724, #1725 (Smart Contracts)

#### Breaking: Error code renumbering (#1722)
Duplicate error variants have been removed from `errors::Error` and all variants
now use non-overlapping domain ranges (1000–1099 auth, 2000–2099 input, 3000–3099
state, 4000–4099 not-found, 5000–5099 arithmetic, 6000–6099 config).

**Removed duplicate variants** (clients matching these numeric codes must update):
- `PermissionDenied = 51` → consolidated into `PermissionDenied = 1010`
- `NoDiscrepancy = 53` → consolidated into `NoDiscrepancy = 3017`
- `ReconciliationThresholdExceeded = 54` → consolidated into `ReconciliationThresholdExceeded = 3018`

**Subsystem variant codes migrated** (were 55–70, now in domain ranges):
- `KeyRotationNoActiveKey`: 55 → 1012
- `KeyRotationVersionNotFound`: 56 → 1013
- `KeyRotationUnauthorized`: 57 → 1014
- `KeyRotationCannotPurgeActive`: 58 → 1015
- `KeyRotationZeroKeyHash`: 59 → 1016
- `KeyRotationAlreadyExists`: 60 → 1017
- `CommitmentNotFound`: 61 → 1018
- `CommitmentHashMismatch`: 62 → 1019
- `CommitmentAlreadyVerified`: 63 → 1020
- `CommitmentCancelled`: 64 → 1021
- `CommitmentExpired`: 65 → 1022
- `CommitmentNotPending`: 66 → 1023
- `InvalidInput`: 67 → 2014
- `NotFound`: 68 → 4005
- `InvalidState`: 69 → 3019
- `CommitmentAlreadyExists`: 70 → 3020

**Added missing variants** (previously referenced in match arms but not defined):
- `IncentiveNotFound = 4002`
- `ParticipantNotFound = 4003`
- `Overflow = 5000`
- `CharityNotSet = 6000`
- `TokenAddressNotSet = 6001`
- `WasteTypeMismatchMerge = 3021`

#### Fixed - Authorization (#1723)
- `require_addresses_different` now correctly receives `&Env` as a parameter
  and delegates to `validation::validate_addresses_different` with a typed
  `Error::SameAddress` instead of a bare-string panic. All 7 call sites updated.

#### Fixed - Checked arithmetic (#1724)
- `redeem_carbon_credits`: `stats.carbon_credits_earned -= amount` replaced
  with `.checked_sub(amount).ok_or(Error::Overflow)?`.
- `create_carbon_listing`: same subtraction and listing counter `+ 1` now
  use `.checked_sub` / `.checked_add`.
- `record_performance_metric`: all three `+= 1` counter increments replaced
  with `.checked_add(1).unwrap_or_else(|| panic_with_error!(..., Error::Overflow))`.

#### Fixed - Storage TTL / rent extension (#1725)
Added missing `storage_utils::bump_instance(&env)` calls to mutating entrypoints
that previously did not extend instance-storage TTL:
- `redeem_carbon_credits`, `create_carbon_listing`, `cancel_carbon_listing`,
  `purchase_carbon_listing`
- `resolve_dispute`
- `create_collection_route`, `complete_route`
- `create_waste_batch` (also fixes unchecked `batch_count + 1`), `add_waste_to_batch`,
  `mark_batch_ready`, `process_batch`
- `certify_waste`
- `add_waste_tag`, `remove_waste_tag`, `set_waste_image`, `add_waste_document`
- `calculate_quality_score`, `update_waste_location`
- `create_challenge`, `join_challenge`, `update_challenge_progress`, `complete_challenge`
- `set_processing_cost`, `set_waste_composition`, `set_recycling_goal`

### Removed - Issue #1077
- **Export Endpoints Cleanup**: Removed legacy export formats that had no active client usage.
  - Removed `GET /api/export/excel` - Legacy Excel format
  - Removed `GET /api/export/xml` - Legacy XML format
  - Removed `GET /api/export/pdf` - Legacy PDF format
  - Removed `GET /api/export/legacy` - Legacy endpoint

### Changed - Issue #1077
- **Export Endpoints**: Active export endpoints now support only CSV and JSON formats.
  - `GET /api/export/waste` - CSV and JSON
  - `GET /api/export/users` - CSV and JSON (admin only)
  - `GET /api/export/analytics` - CSV and JSON (admin only)

### Migration Guide
Clients should migrate to the active endpoints:
- Use `format=csv` for CSV data
- Use `format=json` for JSON data

### Date
2024-01-15: Legacy endpoints removed
