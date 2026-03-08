# 02. Post List

## Objective
- Let users quickly check the latest posts.

## Requirements
- Show non-deleted posts in reverse chronological order
- Allow navigation from the list to each post detail page
- Allow navigation from the list screen to the new post screen

## Displayed Fields
- Title
- Author name
- Posted timestamp
- A leading excerpt of the body, up to roughly 120 characters
- Comment count

## Business Rules
- Deleted posts are not displayed
- If the body excerpt exceeds the limit, truncate the tail
- If there are zero posts, show an empty-state message
- The comment count shown in the list must match the number of visible non-deleted comments on the detail screen
- The default order is newest `createdAt` first, stabilized by descending `id` when timestamps tie
- Updating a post does not change the default list ordering

## Screen Rules
- Show at most 20 posts per screen
- If there are more than 20 posts, provide pagination or a "load more" style continuation UI
- In the empty state, show a message such as "No posts yet"
- Page transitions and reloads must not duplicate the same post within the same page

## Proposed API Contract

### `GET /posts`
- Summary: fetch the post list
- Query:
  - `page`: page number, default `1`
  - `limit`: result size, default `20`
- Response fields:
  - `items`: array of posts
  - `page`
  - `limit`
  - `totalCount`
- Boundary cases:
  - `page < 1` returns `400`
  - `limit < 1` returns `400`
  - `limit > 20` may return `400` or be clamped to `20`, but the implementation must choose one policy consistently
  - A `page` value beyond the total number of pages returns an empty array

## Failure Cases
- If loading fails, show a reload path
- Invalid page numbers return `400`
- If the list is reloaded immediately after deletion, the deleted post must not reappear
- After a retry recovers from a load failure, stale failure messages must not remain
