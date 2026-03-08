# BSS valid demo

This directory is a demo that formalizes the requirements of a simple bulletin board application with `valid` and shows what kinds of guarantees can be checked.

## Purpose

This demo focuses on a stage earlier than implementation verification. The goal is to show what can be checked mechanically during requirements definition itself.

More concretely, this sample answers questions such as:

- How should a requirement set be decomposed into `state`, `action`, and `property`?
- Can screen behavior, API contracts, business rules, and UX constraints be handled on the same verification foundation?
- Can the model cover not only happy paths, but also validation failures, boundary conditions, invisibility after deletion, double-submit prevention, and message consistency?

## Recommended Reading Order

1. [`docs/rdd/README.md`](/Users/tatsuhiko/code/valid-demo/bss/docs/rdd/README.md)
2. [`docs/rdd/07_valid_models.md`](/Users/tatsuhiko/code/valid-demo/bss/docs/rdd/07_valid_models.md)
3. [`docs/rdd/08_bbs_acceptance_requirements.md`](/Users/tatsuhiko/code/valid-demo/bss/docs/rdd/08_bbs_acceptance_requirements.md)
4. [`docs/valid_registry_workflow.md`](/Users/tatsuhiko/code/valid-demo/bss/docs/valid_registry_workflow.md)

## Model Inventory

There are 14 exported models.

- `board-common-spec`
  Shared constraints such as 400/403/404/5xx behavior, anonymous-name defaulting, and HTML escaping
- `board-post-list`
  List retrieval, paging, empty states, and navigation links
- `board-post-create`
  Post creation, invalid input handling, success navigation, and submitting state
- `board-post-detail`
  Detail rendering, not-found handling, updated-at visibility, and comment counts
- `board-edit-delete`
  Edit key checks, delete confirmation, and invisibility after deletion
- `board-comment`
  Comment submission, form preservation on failure, and ordering
- `board-list-rendering`
  Newest/oldest ordering, 120-character excerpts, and continuation UI
- `board-presentation-contract`
  Datetime formatting, body rendering, and success/failure messaging
- `board-api-contract`
  JSON contracts and success/failure response fields
- `board-edit-key-storage`
  Edit key storage policy
- `board-retry-ux`
  Retry flow, error-message placement, and recovery cleanup
- `board-submission-discipline`
  Double-submit prevention and retryability
- `board-message-contract`
  Which important messages are bound to which states
- `board-flow`
  Cross-feature consistency across list, detail, edit, delete, and comment flows

## What `valid` Makes Visible

From the requirements alone, this demo lets you check at least the following:

- `models`
  See which requirement areas were separated into independent models
- `inspect`
  Inspect state fields, actions, properties, and read/write metadata
- `check`
  Verify a requirement as a property and determine whether it holds
- `coverage`
  See which transitions, guards, and branches remain uncovered
- `contract check`
  Detect contract drift in exported model structure
- `lint`
  Check solver readiness and metadata quality for explanation and test generation
- `explain`
  Trace which transition and state change caused a property to fail

## Example Commands

Prerequisites:

- Rust toolchain
- Network access to fetch the pinned `valid` dependency from GitHub

List models:

```sh
cargo run --bin bss-valid-models -- models
```

Inspect the common specification model:

```sh
cargo run --bin bss-valid-models -- inspect board-common-spec --json
```

Check a representative property:

```sh
cargo run --bin bss-valid-models -- check board-common-spec --property=P_COMMON_HTML_IS_ALWAYS_ESCAPED --json
```

Review coverage:

```sh
cargo run --bin bss-valid-models -- coverage board-common-spec --json
```

Run the full verification script:

```sh
./scripts/verify_valid_registry.sh
```

## Current Verification Status

As of March 8, 2026, `./scripts/verify_valid_registry.sh` does not pass completely.

- `board-post-list`
- property: `P_LIST_EMPTY_STATE_MATCHES_VISIBLE_COUNT`

The reachable counterexample shows a page-overflow case where `visible_posts == 0` while `empty_state_visible == false`. That means the boundary between "empty list state" and "page overflow state" is still underspecified.

For a public demo repository, this is best treated as a known specification issue rather than hidden. The README states it explicitly so readers do not mistake the demo for a fully closed verification set.

## Suggested Focus Areas

These parts are especially worth reviewing:

- `board-common-spec`
  How small shared rules become invariants
- `board-list-rendering`
  How UI rendering rules become verifiable constraints instead of informal notes
- `board-retry-ux`
  How UX constraints can be modeled explicitly
- `board-message-contract`
  How messages can be treated as contracts tied to state and result
- `board-flow`
  Cross-feature consistency beyond individual screen-level models

## Notes

This demo pins `valid` to a specific GitHub commit. That improves reproducibility, but the first build still requires network access.

Also, this repository still has no license file. For a public repository, adding one would be the next sensible cleanup step.
