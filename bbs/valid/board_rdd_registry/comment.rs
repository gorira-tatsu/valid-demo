/*
Requirements covered:
- docs/rdd/01_common_specification.md
- docs/rdd/06_comments.md

This model covers:
- anonymous comment defaulting
- rejection of comments on deleted posts
- reflection on detail after success and input preservation on failure
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidState)]
struct CommentState {
    post_exists: bool,
    post_deleted: bool,
    #[valid(range = "0..=1001")]
    body_len: u16,
    #[valid(range = "0..=10")]
    comment_count: u8,
    author_defaulted: bool,
    form_reset: bool,
    reflected_on_detail: bool,
    form_preserved: bool,
    comments_sorted_oldest_first: bool,
    #[valid(enum)]
    outcome: ApiStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidAction)]
enum CommentAction {
    #[valid(action_id = "SETUP_VALID_COMMENT_TARGET", reads = [], writes = ["post_exists", "post_deleted", "body_len", "comment_count", "author_defaulted", "form_reset", "reflected_on_detail", "form_preserved", "comments_sorted_oldest_first", "outcome"])]
    SetupValidCommentTarget,
    #[valid(action_id = "SETUP_EMPTY_COMMENT_BODY", reads = [], writes = ["post_exists", "post_deleted", "body_len", "comment_count", "author_defaulted", "form_reset", "reflected_on_detail", "form_preserved", "comments_sorted_oldest_first", "outcome"])]
    SetupEmptyCommentBody,
    #[valid(action_id = "SETUP_TOO_LONG_COMMENT_BODY", reads = [], writes = ["post_exists", "post_deleted", "body_len", "comment_count", "author_defaulted", "form_reset", "reflected_on_detail", "form_preserved", "comments_sorted_oldest_first", "outcome"])]
    SetupTooLongCommentBody,
    #[valid(action_id = "SETUP_MISSING_COMMENT_TARGET", reads = [], writes = ["post_exists", "post_deleted", "body_len", "comment_count", "author_defaulted", "form_reset", "reflected_on_detail", "form_preserved", "comments_sorted_oldest_first", "outcome"])]
    SetupMissingCommentTarget,
    #[valid(action_id = "SETUP_DELETED_COMMENT_TARGET", reads = [], writes = ["post_exists", "post_deleted", "body_len", "comment_count", "author_defaulted", "form_reset", "reflected_on_detail", "form_preserved", "comments_sorted_oldest_first", "outcome"])]
    SetupDeletedCommentTarget,
    #[valid(action_id = "CREATE_VALID_COMMENT", reads = ["post_exists", "post_deleted", "body_len", "comment_count"], writes = ["comment_count", "form_reset", "reflected_on_detail", "form_preserved", "outcome"])]
    CreateValidComment,
    #[valid(action_id = "CREATE_ANONYMOUS_COMMENT", reads = ["post_exists", "post_deleted", "body_len", "comment_count"], writes = ["comment_count", "author_defaulted", "form_reset", "reflected_on_detail", "form_preserved", "outcome"])]
    CreateAnonymousComment,
    #[valid(action_id = "CREATE_INVALID_COMMENT", reads = ["body_len"], writes = ["form_reset", "reflected_on_detail", "form_preserved", "outcome"])]
    CreateInvalidComment,
    #[valid(action_id = "CREATE_COMMENT_ON_MISSING_POST", reads = ["post_exists", "post_deleted"], writes = ["form_preserved", "outcome"])]
    CreateCommentOnMissingPost,
    #[valid(action_id = "CREATE_COMMENT_SAVE_FAILURE", reads = ["post_exists", "post_deleted"], writes = ["form_preserved", "outcome"])]
    CreateCommentSaveFailure,
}

valid_model! {
    model CommentModel<CommentState, CommentAction>;
    init [CommentState {
        post_exists: true,
        post_deleted: false,
        body_len: 1,
        comment_count: 0,
        author_defaulted: false,
        form_reset: false,
        reflected_on_detail: false,
        form_preserved: false,
        comments_sorted_oldest_first: true,
        outcome: ApiStatus::Idle,
    }];
    transitions {
        transition SetupValidCommentTarget [role = setup] [tags = ["comment_path", "setup_path"]]
        when |state| true
        => [CommentState
            {
                post_exists : true, post_deleted : false, body_len : 1, comment_count : 0,
                author_defaulted : false, form_reset : false, reflected_on_detail : false,
                form_preserved : false, comments_sorted_oldest_first : true, outcome :
                ApiStatus::Idle
            }
        ];
        transition SetupEmptyCommentBody [role = setup] [tags = ["comment_path", "setup_path", "validation_path"]]
        when |state| true
        => [CommentState
            {
                post_exists : true, post_deleted : false, body_len : 0, comment_count : 0,
                author_defaulted : false, form_reset : false, reflected_on_detail : false,
                form_preserved : false, comments_sorted_oldest_first : true, outcome :
                ApiStatus::Idle
            }
        ];
        transition SetupTooLongCommentBody [role = setup] [tags = ["comment_path", "setup_path", "validation_path"]]
        when |state| true
        => [CommentState
            {
                post_exists : true, post_deleted : false, body_len : 1001, comment_count :
                0, author_defaulted : false, form_reset : false, reflected_on_detail :
                false, form_preserved : false, comments_sorted_oldest_first : true,
                outcome : ApiStatus::Idle
            }
        ];
        transition SetupMissingCommentTarget [role = setup] [tags = ["comment_path", "resource_path", "setup_path"]]
        when |state| true
        => [CommentState
            {
                post_exists : false, post_deleted : false, body_len : 1, comment_count :
                0, author_defaulted : false, form_reset : false, reflected_on_detail :
                false, form_preserved : false, comments_sorted_oldest_first : true,
                outcome : ApiStatus::Idle
            }
        ];
        transition SetupDeletedCommentTarget [role = setup] [tags = ["comment_path", "resource_path", "setup_path"]]
        when |state| true
        => [CommentState
            {
                post_exists : true, post_deleted : true, body_len : 1, comment_count : 0,
                author_defaulted : false, form_reset : false, reflected_on_detail : false,
                form_preserved : false, comments_sorted_oldest_first : true, outcome :
                ApiStatus::Idle
            }
        ];
        transition CreateValidComment [tags = ["allow_path", "comment_path"]]
        when |state| state.post_exists && !state.post_deleted && state.body_len >= 1 &&
            state.body_len <= 1000 && state.comment_count <= 9
        => [CommentState
            {
                comment_count : state.comment_count + 1, form_reset : true,
                reflected_on_detail : true, form_preserved : false, outcome :
                ApiStatus::Ok, .. state
            }
        ];
        transition CreateAnonymousComment [tags = ["allow_path", "comment_path", "defaulting_path"]]
        when |state| state.post_exists && !state.post_deleted && state.body_len >= 1 &&
            state.body_len <= 1000 && state.comment_count <= 9
        => [CommentState
            {
                comment_count : state.comment_count + 1, author_defaulted : true,
                form_reset : true, reflected_on_detail : true, form_preserved : false,
                outcome : ApiStatus::Ok, .. state
            }
        ];
        transition CreateInvalidComment [tags = ["comment_path", "deny_path", "validation_path"]]
        when |state| state.body_len == 0 || state.body_len == 1001
        => [CommentState
            {
                form_reset : false, reflected_on_detail : false, form_preserved : true,
                outcome : ApiStatus::BadRequest, .. state
            }
        ];
        transition CreateCommentOnMissingPost [tags = ["comment_path", "deny_path", "resource_path"]]
        when |state| state.post_exists == false || state.post_deleted
        => [CommentState
            {
                form_reset : false, reflected_on_detail : false, form_preserved : true,
                outcome : ApiStatus::NotFound, .. state
            }
        ];
        transition CreateCommentSaveFailure [tags = ["comment_path", "exception_path", "retry_path"]]
        when |state| state.post_exists && !state.post_deleted
        => [CommentState
            {
                form_reset : false, reflected_on_detail : false, form_preserved : true,
                outcome : ApiStatus::ServerError, .. state
            }
        ];
    }
    properties {
        invariant P_COMMENT_SUCCESS_RESETS_FORM |state|
            state.outcome != ApiStatus::Ok || state.form_reset;
        invariant P_COMMENT_SUCCESS_REFLECTS_ON_DETAIL |state|
            state.outcome != ApiStatus::Ok || state.reflected_on_detail;
        invariant P_COMMENT_FAILURE_PRESERVES_FORM |state|
            !(state.outcome == ApiStatus::BadRequest || state.outcome ==
            ApiStatus::ServerError || state.outcome == ApiStatus::NotFound) ||
            state.form_preserved;
        invariant P_COMMENT_UNAVAILABLE_POST_RETURNS_NOT_FOUND |state|
            !(state.post_exists == false || state.post_deleted) || state.outcome !=
            ApiStatus::Ok;
        invariant P_COMMENT_VISIBLE_ORDER_IS_OLDEST_FIRST |state|
            state.comments_sorted_oldest_first;
    }
}
