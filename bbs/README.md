# BBS valid demo

This directory is a demo that formalizes the requirements of a simple bulletin board application with [`valid`](https://github.com/gorira-tatsu/valid) and shows what kinds of guarantees can be checked.

[`valid`](https://github.com/gorira-tatsu/valid) is a Rust-first finite-state verification language and toolchain for expressing requirements as states, actions, transitions, and properties that can be checked mechanically.

## Recommended Reading Order

1. [`docs/rdd/README.md`](/Users/tatsuhiko/code/valid-demo/bbs/docs/rdd/README.md)
2. [`docs/rdd/07_valid_models.md`](/Users/tatsuhiko/code/valid-demo/bbs/docs/rdd/07_valid_models.md)
3. [`docs/rdd/08_bbs_acceptance_requirements.md`](/Users/tatsuhiko/code/valid-demo/bbs/docs/rdd/08_bbs_acceptance_requirements.md)
4. [`docs/valid_registry_workflow.md`](/Users/tatsuhiko/code/valid-demo/bbs/docs/valid_registry_workflow.md)
5. [`docs/demo_walkthrough.md`](/Users/tatsuhiko/code/valid-demo/bbs/docs/demo_walkthrough.md)

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
cargo run --bin bbs-valid-models -- models
```

Inspect the common specification model:

```sh
cargo run --bin bbs-valid-models -- inspect board-common-spec --json
```

Check a representative property:

```sh
cargo run --bin bbs-valid-models -- check board-common-spec --property=P_COMMON_HTML_IS_ALWAYS_ESCAPED --json
```

Review coverage:

```sh
cargo run --bin bbs-valid-models -- coverage board-common-spec --json
```

Run the full verification script:

```sh
./scripts/verify_valid_registry.sh
```

## Output Samples

### `inspect`

```json
{
  "status": "ok",
  "model_id": "CommonSpecModel",
  "state_fields": [
    "title_len",
    "body_len",
    "edit_key_len",
    "comment_body_len",
    "author_defaulted",
    "comment_author_defaulted",
    "html_escaped",
    "resource_visible",
    "outcome"
  ],
  "properties": [
    "P_COMMON_HTML_IS_ALWAYS_ESCAPED",
    "P_COMMON_BAD_REQUEST_HIDES_INVALID_RESOURCE"
  ]
}
```

Read it as:

- `state_fields`: what the requirement model actually keeps as state
- `properties`: what can be checked directly
- `model_id`: the underlying exported model

### `check`

```json
{
  "status": "PASS",
  "property_result": {
    "property_id": "P_COMMON_HTML_IS_ALWAYS_ESCAPED",
    "status": "PASS",
    "summary": "no violating state found in the reachable state space"
  },
  "review_summary": {
    "headline": "PASS P_COMMON_HTML_IS_ALWAYS_ESCAPED for board-common-spec"
  }
}
```

Read it as:

- `status`: overall result for this run
- `property_result.status`: whether the property held
- `summary`: short reason
- `review_summary.headline`: one-line reviewer-facing result

### `coverage`

```json
{
  "summary": {
    "transition_coverage_percent": 100,
    "decision_coverage_percent": 94,
    "guard_full_coverage_percent": 68
  },
  "gate": {
    "status": "warn",
    "reasons": [
      "guard_full_coverage below threshold"
    ]
  }
}
```

Read it as:

- `transition_coverage_percent`: were the declared transitions exercised
- `decision_coverage_percent`: were guard/update decisions exercised
- `guard_full_coverage_percent`: did each guard see both true and false
- `gate.status`: whether the current policy considers coverage sufficient

## Known Issue

As of March 8, 2026, `./scripts/verify_valid_registry.sh` does not fully pass.

- `board-post-list`
- property: `P_LIST_EMPTY_STATE_MATCHES_VISIBLE_COUNT`

The reachable counterexample shows a page-overflow case where `visible_posts == 0` while `empty_state_visible == false`. That means the boundary between "empty list state" and "page overflow state" is still underspecified.

The useful part is not only that the property fails. The useful part is what the failure teaches:

- the current requirement sentence is too coarse
- the model distinguishes between "no posts exist" and "the requested page overflowed"
- the property does not yet distinguish those cases

In other words, the failure is evidence of a requirements gap, not just a model bug.
