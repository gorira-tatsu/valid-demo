# 06. Comments

## Objective
- Allow users to write replies or opinions on a post.

## Requirements
- Users can submit comments from the post detail screen
- Users can view the list of comments associated with a post
- After submission, the new comment is reflected on the same detail screen

## Input Fields
- Comment body
- Author name

## Business Rules
- If the author name is omitted, set it to the anonymous display name
- Deleted posts cannot receive comments
- Comments are displayed in oldest-first order by creation time
- Successful comment submission must also keep the list-level comment count consistent
- When comment creation succeeds, `id` and `createdAt` are fixed, and reloads return the same order immediately afterward

## Screen Rules
- Show the comment form at the bottom of the post detail screen
- Reset the form after successful comment submission
- For each comment, show the author name, body, and creation timestamp
- If submission fails, preserve the input and allow retry on the same detail screen
- Do not allow duplicate submission of the same input while the request is in flight

## Proposed API Contract

### `POST /posts/:postId/comments`
- Summary: create a comment
- Request fields:
  - `body`
  - `authorName`
- Response fields:
  - `id`
  - `postId`
  - `body`
  - `authorName`
  - `createdAt`

## Validation
- If the body is empty, show "Please enter a comment"
- If the body exceeds 1,000 characters, show a limit-exceeded error

## Failure Cases
- Return `404` if the post does not exist or has already been deleted
- If saving fails, preserve the input and show the error
- Comment submission must not succeed against a post that was just deleted
- After retry recovery, failure messages must not remain
- After success, stale failure messages must not remain
