//! Charity domain module (issues #1726).
//!
//! Documents the charity subsystem boundary within `ScavengerContract`.
//! All state-changing charity operations are implemented on `ScavengerContract`
//! in `lib.rs`:
//! - `set_charity_contract` — admin-only; stores the target charity address.
//! - `get_charity_contract` — read-only accessor.
//! - `donate_to_charity`   — deducts tokens from the donor's earned balance
//!                           and emits a `donation_made` event.
//!
//! # Design
//!
//! The charity subsystem intentionally contains no on-chain token transfer.
//! `donate_to_charity` deducts from the donor's internal `total_tokens_earned`
//! counter and records the event; the actual settlement with the external
//! charity contract is handled off-chain or via a separate redemption flow.
//! This keeps the charity entrypoint free of cross-contract calls and therefore
//! free of reentrancy risk.
//!
//! # Storage
//!
//! One instance-storage entry keyed by the `CHARITY` symbol constant holds the
//! configured charity contract address (set by admin, absent until first call).
//!
//! # Reentrancy
//!
//! `donate_to_charity` acquires the reentrancy guard before modifying storage
//! and releases it before returning, preventing re-entrant donation attempts.
