# 08. BBS Acceptance Requirements

## Objective
- Define the cross-cutting requirements that determine whether the bulletin board is minimally viable as an MVP.
- Cover not only per-feature requirements, but also consistency across the full post lifecycle.

## Acceptance Conditions
- A user can find the latest non-deleted posts from the list and navigate to detail
- A user can create a new post and confirm it in detail after success
- A user can read comments and submit a comment from the same detail screen
- The original poster can update or delete the post using the edit key
- Deleted posts are excluded consistently from list, detail, and comment-submission flows
- Even first-time users can complete the flow because required fields, input errors, return-to-list paths, and edit-key retention warnings are visible

## Cross-Cutting Business Rules
- After a successful post creation, the new post becomes visible in the list or detail view
- After a successful update, the detail view reflects the updated content and timestamp
- After a successful deletion, the post can no longer be used from list, detail, or comment-submission APIs
- After a successful comment submission, both the comment count and comment content update immediately on the same detail screen
- The list-level comment count must not contradict the visible non-deleted comment count on the detail screen
- Reloading the list or detail view must preserve stable ordering for the same data set
- Error responses are handled consistently as JSON carrying `message` and, when needed, `fieldErrors`

## Acceptance Under Failure
- Invalid input does not save and remains correctable with the current input preserved
- Server errors do not show success and failure messages at the same time
- Operations against non-existent or deleted posts resolve consistently as `404 Not Found` or equivalent non-visible behavior
- An incorrect edit key allows neither update nor deletion and keeps the form retryable
- Post creation and comment submission do not allow duplicate success while a request is in flight
- After retry recovery, stale failure UI is cleared and the user returns to normal operation
- Even when detail loading for an updated post fails temporarily, retry recovery preserves updated-at visibility and comment-count consistency
- Invalid list `page` and `limit`, unknown post IDs, and update/delete/comment operations against deleted posts all fail consistently across the implementation

## `valid` Coverage In This Demo
- Cross-feature consistency from post creation through list visibility, detail visibility, update reflection, comment-count consistency, and rejection of comments after deletion:
  - `BoardFlowModel`
- Non-deleted post visibility on the list, detail navigation, and paging:
  - `PostListModel`
  - `ListRenderingModel`
- Comment empty state in detail, updated-at visibility, and deleted-post 404 behavior:
  - `PostDetailModel`
- Edit-key match/mismatch, delete confirmation, and invisibility after deletion:
  - `EditDeleteModel`
- Comment success reflection, input preservation on failure, and rejection on deleted posts:
  - `CommentModel`
- Mutual exclusion between success messages and retry banners, plus error recovery paths:
  - `PresentationContractModel`
  - `RetryUxModel`
- Double-submit prevention for post/comment creation and retry discipline after failure:
  - `SubmissionDisciplineModel`
- Recovery after temporary detail-load failure on an updated post, restoring `updatedAt` visibility and list/detail comment-count consistency:
  - `BoardFlowModel`
