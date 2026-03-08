# 00. Assumptions And Scope

## Objective
- Provide a lightweight bulletin board where anyone can post text easily.
- Define a minimum product that works for small communities and validation-oriented use cases.

## Intended Users
- Visitors who only read posts
- Users who create posts
- Users who comment on posts

## MVP Scope
- Posts can be listed chronologically
- A post detail page can be viewed
- New posts can be created
- The original poster can edit and delete a post
- Users can comment on posts

## Out Of Scope
- Sign-up, login, and logout
- Image, video, and file attachments
- Tags, categories, and full-text search
- Likes, reactions, and reporting
- Admin approval, banned-word management, and audit logs
- Notifications, email delivery, and external integrations

## Assumptions
- Poster identity is represented by an edit key chosen by the user at post creation time.
- The system never re-displays the edit key in plaintext. Users must store it themselves when creating the post.
- Posts and comments are text-only.
- The UI is assumed to be Japanese-language for this MVP.

## Success Conditions
- A first-time user can complete posting, reading, and commenting without guidance
- Invalid input returns clear error messages
- Deleted data never appears in list or detail views
