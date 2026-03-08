# 04. Post Detail

## Objective
- Allow users to read the full post body and its comments.

## Requirements
- Show the full post body
- Show non-deleted comments associated with the post
- From the detail screen, users can move to or invoke comment creation, post editing, and post deletion

## Displayed Fields
- Title
- Body
- Author name
- Created timestamp
- Updated timestamp
- Comment list

## Business Rules
- If the post does not exist, show a 404 screen
- Do not show `updatedAt` when the post has never been updated
- Show comments in oldest-first order
- Deleted posts are treated as 404-equivalent even when accessed by direct URL
- The comment count and comment content shown on the detail screen must reflect the latest successful comment submission
- Comments with the same timestamp are stabilized by ascending `id`

## Screen Rules
- Preserve body line breaks
- Show an empty-state message when there are zero comments
- Place edit and delete controls near the post content area
- Make created and updated timestamps distinguishable for updated posts

## Proposed API Contract

### `GET /posts/:postId`
- Summary: fetch post detail
- Response fields:
  - `post`: post object
  - `comments`: array of comments

## Failure Cases
- Deleted posts are treated as 404-equivalent
- If loading fails, show an error message and a path back to the list
- If retry is possible after a load failure, success messages must not remain on screen
- After retry recovery, updated-at visibility and comment-count consistency must return to the latest successful state
