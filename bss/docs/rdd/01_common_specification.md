# 01. Common Specification

## Data Model

### Post
- `id`: post identifier
- `title`: post title
- `body`: post body
- `authorName`: post author name
- `editKey`: post edit key
- `createdAt`: creation timestamp
- `updatedAt`: update timestamp
- `deletedAt`: logical deletion timestamp, `null` when not deleted

### Comment
- `id`: comment identifier
- `postId`: associated post ID
- `body`: comment body
- `authorName`: comment author name
- `createdAt`: creation timestamp
- `deletedAt`: logical deletion timestamp, `null` when not deleted

## Shared Rules For Identifiers And Timestamps
- `id` must be unique and never reused within the same entity type
- `createdAt` is fixed exactly once when creation succeeds
- `updatedAt` is `null` until an update succeeds, and changes only on successful post updates
- `updatedAt` must be greater than or equal to `createdAt`
- The default post list order is descending `createdAt`, stabilized by descending `id` when timestamps tie
- The default comment order is ascending `createdAt`, stabilized by ascending `id` when timestamps tie
- Reloading the same data set must return the same ordering

## Shared Validation

### Post
- Title: required, 1 to 100 characters
- Body: required, 1 to 5,000 characters
- Author name: optional, defaults to the anonymous display name when omitted
- Edit key: required, 4 to 32 characters

### Comment
- Body: required, 1 to 1,000 characters
- Author name: optional, defaults to the anonymous display name when omitted

## Shared Screen Rules
- Datetimes are displayed in `YYYY-MM-DD HH:mm` format
- Line breaks are preserved when rendering body text
- Input errors are shown per field
- Communication failures display a retryable message at the top of the screen or directly below the form

## Shared API Policy
- APIs return JSON
- Successful cases return 2xx, invalid input returns 4xx, and unexpected failures return 5xx
- Deleted posts and comments are never returned from the API
- The effects of create, update, delete, and comment creation must remain consistent after reload
- Validation errors must support at least `message` plus per-field `fieldErrors`
- Even `404`, `403`, and `500` responses keep the basic JSON response structure

## Error Handling
- A non-existent post ID returns `404 Not Found`
- Invalid input returns `400 Bad Request`
- An incorrect edit key returns `403 Forbidden`
- Success and failure messages must not be displayed simultaneously for the same operation result
- Detail fetch, update, delete, and comment creation against deleted posts all resolve to `404 Not Found`
- Invalid `page` or `limit` values on the list API return `400 Bad Request`

## Security And Operational Assumptions
- HTML tags are treated as text and escaped on render
- Anti-spam controls may stay simple in the MVP, but at least minimal server-side validation is required
- Edit keys must not be stored in plaintext and must be hashed before persistence
- Comment deletion is out of scope for the MVP, and `deletedAt` is reserved for future extension
