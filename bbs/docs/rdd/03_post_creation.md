# 03. Post Creation

## Objective
- Allow users to create new topics on the bulletin board.

## Requirements
- A user can submit a post with a title, body, author name, and edit key
- After a successful submission, the user moves to the post detail screen
- Invalid input is shown as an error without saving the post

## Input Fields
- Title
- Body
- Author name
- Edit key

## Business Rules
- If the author name is omitted, set it to the anonymous display name
- The edit key is chosen by the user when creating the post
- The edit key does not need confirmation input, but users should be prompted to keep it safely
- Record the creation timestamp when the post succeeds
- Duplicate submission of the same post while a request is in flight must not create duplicate posts
- After success, `id` and `createdAt` are fixed, and the same post can be retrieved immediately afterward

## Screen Rules
- Required fields are visibly marked
- The body field is a multiline textarea
- It is acceptable to show a message such as "Post created successfully"
- After success, navigate to post detail, and if a success message is shown it must appear only once per successful operation

## Proposed API Contract

### `POST /posts`
- Summary: create a new post
- Request fields:
  - `title`
  - `body`
  - `authorName`
  - `editKey`
- Response fields:
  - `id`
  - `title`
  - `body`
  - `authorName`
  - `createdAt`

## Validation
- If the title is empty, show "Please enter a title"
- If the body is empty, show "Please enter a body"
- If the edit key is empty, show "Please enter an edit key"
- If a field exceeds its maximum length, show the corresponding per-field limit message

## Failure Cases
- If saving fails, show the error while preserving the current input
- Disable the submit button while the request is in flight to prevent double submission
- Accept another submit only after recovery from a retryable failure
- Do not leave stale failure messages after success
