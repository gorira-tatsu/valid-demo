# 07. valid Modeling

## Goal
- Map the requirements in `docs/rdd` into finite-state models that can be verified with `valid`.
- Focus not on literal UI strings or raw JSON structure by themselves, but on business rules and transition constraints expressed as invariants and transitions.

## Location
- registry: `valid/board_rdd_registry.rs`
- exported models:
  - `board-common-spec`
  - `board-post-list`
  - `board-post-create`
  - `board-post-detail`
  - `board-edit-delete`
  - `board-comment`
  - `board-list-rendering`
  - `board-presentation-contract`
  - `board-api-contract`
  - `board-edit-key-storage`
  - `board-retry-ux`
  - `board-submission-discipline`
  - `board-message-contract`

## Modeling Strategy
- Abstract away from concrete strings into finite attributes such as `title_len`, `body_len`, and `edit_key_len`.
- Reduce screen specifications to UI facts such as `screen`, `phase`, `empty_state_visible`, and `form_preserved`.
- Normalize API behavior through `ApiStatus` into `Ok`, `BadRequest`, `Forbidden`, `NotFound`, and `ServerError`.
- Represent logical deletion with combinations such as `post_deleted` and `post_visible`, then pin list/detail invisibility as invariants.

## Mapping To The RDD

### `00_assumptions_and_scope.md`
- No authentication, identity represented by edit key:
  - `CommonSpecModel`
  - `EditDeleteModel`
- Do not show deleted data:
  - `PostListModel`
  - `PostDetailModel`
  - `EditDeleteModel`

### `01_common_specification.md`
- Required post/comment input and length constraints:
  - `CommonSpecModel`
- Anonymous-name defaulting:
  - `CommonSpecModel`
  - `PostCreateModel`
  - `CommentModel`
- `updatedAt` visibility and update-only-on-success behavior:
  - `PostDetailModel`
  - `BoardFlowModel`
- Separation of `400 / 403 / 404 / 5xx` behavior:
  - `CommonSpecModel`
  - `EditDeleteModel`
  - `CommentModel`
- Base contract for JSON error responses:
  - `ApiContractModel`
- HTML escaping assumptions:
  - `CommonSpecModel`
  - `PresentationContractModel`
- Datetime format, newline preservation, and retry-message placement:
  - `PresentationContractModel`
  - `RetryUxModel`
- JSON response structure for APIs:
  - `ApiContractModel`
- Hashed storage policy for `editKey`:
  - `EditKeyStorageModel`

### `02_post_list.md`
- 20-item cap, empty state, detail navigation, and new-post navigation:
  - `PostListModel`
- `page` and `limit` boundary conditions plus invalid-value rejection:
  - `PostListModel`
- Timestamp comparisons for newest/oldest sort, 120-character excerpts, pagination or load-more:
  - `ListRenderingModel`
- Stable ordering for the same data set:
  - `ListRenderingModel`
- Empty-state messaging:
  - `MessageContractModel`

### `03_post_creation.md`
- Success navigation to detail, input preservation on failure, and double-submit prevention while sending:
  - `PostCreateModel`
  - `SubmissionDisciplineModel`

### `04_post_detail.md`
- Detail visibility only for non-deleted posts, comment empty state, oldest-first comments, and updated-at display:
  - `PostDetailModel`

### `05_post_edit_and_delete.md`
- Update/delete only when the edit key matches, delete confirmation, logical deletion, and invisibility after deletion:
  - `EditDeleteModel`
- Incorrect edit-key message:
  - `MessageContractModel`

### `06_comments.md`
- Rejection of comments on deleted posts, reflection on success, and input preservation on failure:
  - `CommentModel`
- Duplicate-submit prevention while sending comments and retry discipline after failure:
  - `SubmissionDisciplineModel`

### `08_bbs_acceptance_requirements.md`
- Cross-feature consistency from post creation through list visibility, detail visibility, update reflection, comment-count consistency, invisibility after deletion, and comment rejection:
  - `BoardFlowModel`
- Consistency across list reloads and detail reloads:
  - `BoardFlowModel`
- Cross-feature recovery after a temporary detail-load failure on an updated post, restoring `updatedAt` visibility and comment-count consistency:
  - `BoardFlowModel`
- Mutual exclusion between success messages and retry banners, plus retry paths after failure:
  - `PresentationContractModel`
  - `RetryUxModel`
- Duplicate-submit prevention during post/comment creation and normal recovery afterward:
  - `SubmissionDisciplineModel`

## Verification Approach
- Avoid one huge model. Split by feature area to keep the state space manageable.
- Each model carries core invariants for its chapter and can be inspected or verified independently through `valid-registry`.
- When implementation introduces new requirements, first extend the corresponding chapter model with actions and invariants.

## valid-registry Operation
- Because verification is binary-based, run `cargo build` first whenever `valid/board_rdd_registry.rs` changes.
- Fix the contract drift baseline to [valid/contract-lock.json](/Users/tatsuhiko/code/valid-demo/bss/valid/contract-lock.json).
- See [docs/valid_registry_workflow.md](/Users/tatsuhiko/code/valid-demo/bss/docs/valid_registry_workflow.md) for the operating procedure.
- Minimum review order:
  - `valid_contract_check`
  - `valid_inspect`
  - `valid_lint`
  - `valid_check`
  - `valid_coverage`

## Additional Precision Added In This Demo
- `ListRenderingModel`
  - Verify newest-first and oldest-first behavior as invariants using `SortOrder` and timestamp comparisons for the first two items.
  - Compress excerpt behavior into `body_len` and `excerpt_len`, and require `excerpt_ellipsized` when the body exceeds 120 characters.
  - Preserve UI variation through `ContinuationUi::Pagination | LoadMore`.
- `RetryUxModel`
  - Fix list-load failures to `TopBanner` and form-submission failures to `BelowForm`.
  - Model retry as the recovery transition `ErrorShown -> Retrying -> Recovered`.
- `MessageContractModel`
  - Reduce major messages to the finite set `EmptyPostList / PostCreatedCompleted / InvalidEditKey` and bind them to screens and result codes.
- `PresentationContractModel`
  - Verify `YYYY-MM-DD HH:mm` by decomposing it into digit counts and separator counts.
  - Require body rendering to satisfy both `html_escaped` and `newline_preserved`.
  - Make retry-message placement explicit through `RetryMessagePlacement::TopBanner | BelowForm`.
  - Fix the successful post message to `SuccessMessageKind::PostCreatedCompleted`.
- `BoardFlowModel`
  - Model temporary detail-load failure on an updated post as `RetryPhase::ErrorShown`, requiring both retryability and a path back to the list.
  - After recovery in `RetryPhase::Recovered`, verify that `updatedAt` visibility and list/detail comment-count consistency are restored together.
- `ApiContractModel`
  - Fix the response field sets for `GET /posts` and `POST /posts` as invariants.
  - Treat preservation of the basic JSON structure even on error as part of the API contract.
- `EditKeyStorageModel`
  - Fix the policy that `editKey` must never be persisted in plaintext and must be hashed when stored.
