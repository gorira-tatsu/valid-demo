# Demo Walkthrough

This page shows one complete path from a requirement sentence to a `valid` property, a failing counterexample, and a likely requirement refinement.

## Requirement Sentence

When the requested list page contains zero visible posts, the UI should show an empty state.

## Model

This behavior is modeled in `board-post-list`.

Relevant state facts include:

- `visible_posts`
- `empty_state_visible`
- `requested_page`
- `total_pages`
- `outcome`

The point is not to model every UI pixel. The model keeps only the state needed to express the behavior in a checkable form.

## Property

The current property is:

```text
P_LIST_EMPTY_STATE_MATCHES_VISIBLE_COUNT
```

In practical terms, it tries to keep the list empty state aligned with the number of visible posts.

## Failing Example

Running:

```sh
cargo run --bin bbs-valid-models -- check board-post-list --property=P_LIST_EMPTY_STATE_MATCHES_VISIBLE_COUNT --json
```

currently fails.

The important part of the counterexample is:

```json
{
  "status": "FAIL",
  "review_summary": {
    "headline": "FAIL P_LIST_EMPTY_STATE_MATCHES_VISIBLE_COUNT for board-post-list",
    "failing_action_id": "LIST_PAGE_OVERFLOW",
    "action_sequence": [
      "SETUP_PAGE_OVERFLOW_REQUEST",
      "LIST_PAGE_OVERFLOW"
    ]
  },
  "traceback": {
    "action_id": "LIST_PAGE_OVERFLOW",
    "involved_fields": [
      "visible_posts",
      "empty_state_visible",
      "requested_page",
      "total_pages"
    ]
  }
}
```

The failing state shows:

- `visible_posts == 0`
- `empty_state_visible == false`
- `requested_page == 2`
- `total_pages == 1`

So the model found a page-overflow case, not a truly empty bulletin board.

## What The Failure Means

This is useful because it exposes an ambiguity in the requirement.

The original wording mixes two different situations:

- no posts exist
- the requested page is beyond the available page range

Those are not the same UI state, but the current property treats them as if they were.

## Likely Requirement Refinement

A more precise requirement would be closer to:

- Show the empty state when the bulletin board has no visible posts on the first valid page because the data set itself is empty.
- Do not treat page overflow as the same state as an empty bulletin board.
- For page overflow, return an empty result set while keeping a distinct navigation or boundary-state interpretation.

## Why This Matters For The Demo

This is the core value of the repo.

The point is not only to prove that a property passes. The point is to make underspecified requirements visible early, while they are still cheap to change.
