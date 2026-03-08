# 05. Post Edit And Delete

## Objective
- Allow the original poster to modify or delete the post.

## Requirements
- A poster can edit a post by entering the correct edit key
- A poster can delete a post by entering the correct edit key
- Successful edits update the modification timestamp
- Deleted posts disappear from both list and detail views

## Input Fields
- For editing:
  - Title
  - Body
  - Author name
  - Edit key
- For deletion:
  - Edit key

## Business Rules
- If the edit key does not match, neither update nor delete is allowed
- Deletion is logical deletion
- Deleted posts cannot be edited again
- After a successful update, the detail screen shows the updated content and updated timestamp
- After successful deletion, list, detail, and comment flows all prevent further interaction with that post
- After a successful update, remain on the same post detail screen and show only the successful result
- After successful deletion, do not remain on the deleted post detail or edit screen, and move to the list or an equivalent non-visible state

## Screen Rules
- Show a confirmation dialog before deleting
- Pre-fill the edit form with the current post content
- On edit failure, preserve the input and re-render the form
- After successful deletion, do not return to the deleted post's edit screen

## Proposed API Contract

### `PUT /posts/:postId`
- Summary: update a post
- Request fields:
  - `title`
  - `body`
  - `authorName`
  - `editKey`

### `DELETE /posts/:postId`
- Summary: delete a post
- Request fields:
  - `editKey`

## Validation
- Validation rules for updates are the same as for new post creation
- If the edit key does not match, show "The edit key is incorrect"

## Failure Cases
- Return `404` if the target post does not exist
- Return `404` if the post has already been deleted
- Do not allow deletion to succeed without passing through the confirmation dialog
- After an edit-key mismatch, allow retry while preserving the current input
