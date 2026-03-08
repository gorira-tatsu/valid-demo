/*
対応する要件定義:
- docs/rdd/00_前提とスコープ.md
- docs/rdd/01_共通仕様.md

この model が担うこと:
- 投稿/コメントの必須入力と文字数制約
- 匿名時の `名無しさん` 補完
- `400 / 403 / 404 / 5xx` の使い分け
- HTML エスケープ前提
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidState)]
struct CommonSpecState {
    #[valid(range = "0..=101")]
    title_len: u8,
    #[valid(range = "0..=5001")]
    body_len: u16,
    #[valid(range = "0..=33")]
    edit_key_len: u8,
    #[valid(range = "0..=1001")]
    comment_body_len: u16,
    author_defaulted: bool,
    comment_author_defaulted: bool,
    html_escaped: bool,
    resource_visible: bool,
    #[valid(enum)]
    outcome: ApiStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidAction)]
enum CommonSpecAction {
    #[valid(action_id = "SETUP_MISSING_POST_TITLE", reads = [], writes = ["title_len", "body_len", "edit_key_len", "comment_body_len", "author_defaulted", "comment_author_defaulted", "html_escaped", "resource_visible", "outcome"])]
    SetupMissingPostTitle,
    #[valid(action_id = "SETUP_MISSING_POST_BODY", reads = [], writes = ["title_len", "body_len", "edit_key_len", "comment_body_len", "author_defaulted", "comment_author_defaulted", "html_escaped", "resource_visible", "outcome"])]
    SetupMissingPostBody,
    #[valid(action_id = "SETUP_MISSING_POST_EDIT_KEY", reads = [], writes = ["title_len", "body_len", "edit_key_len", "comment_body_len", "author_defaulted", "comment_author_defaulted", "html_escaped", "resource_visible", "outcome"])]
    SetupMissingPostEditKey,
    #[valid(action_id = "SETUP_TOO_LONG_POST_INPUT", reads = [], writes = ["title_len", "body_len", "edit_key_len", "comment_body_len", "author_defaulted", "comment_author_defaulted", "html_escaped", "resource_visible", "outcome"])]
    SetupTooLongPostInput,
    #[valid(action_id = "SETUP_MISSING_COMMENT_BODY", reads = [], writes = ["title_len", "body_len", "edit_key_len", "comment_body_len", "author_defaulted", "comment_author_defaulted", "html_escaped", "resource_visible", "outcome"])]
    SetupMissingCommentBody,
    #[valid(action_id = "SETUP_TOO_LONG_COMMENT_BODY", reads = [], writes = ["title_len", "body_len", "edit_key_len", "comment_body_len", "author_defaulted", "comment_author_defaulted", "html_escaped", "resource_visible", "outcome"])]
    SetupTooLongCommentBody,
    #[valid(action_id = "SUBMIT_VALID_POST", reads = ["title_len", "body_len", "edit_key_len"], writes = ["outcome", "resource_visible"])]
    SubmitValidPost,
    #[valid(action_id = "SUBMIT_POST_MISSING_TITLE", reads = ["title_len"], writes = ["outcome", "resource_visible"])]
    SubmitPostMissingTitle,
    #[valid(action_id = "SUBMIT_POST_MISSING_BODY", reads = ["body_len"], writes = ["outcome", "resource_visible"])]
    SubmitPostMissingBody,
    #[valid(action_id = "SUBMIT_POST_MISSING_EDIT_KEY", reads = ["edit_key_len"], writes = ["outcome", "resource_visible"])]
    SubmitPostMissingEditKey,
    #[valid(action_id = "SUBMIT_POST_TOO_LONG", reads = ["title_len", "body_len"], writes = ["outcome", "resource_visible"])]
    SubmitPostTooLong,
    #[valid(action_id = "SUBMIT_ANONYMOUS_POST", reads = ["title_len", "body_len", "edit_key_len"], writes = ["author_defaulted", "outcome", "resource_visible"])]
    SubmitAnonymousPost,
    #[valid(action_id = "SUBMIT_VALID_COMMENT", reads = ["comment_body_len"], writes = ["outcome", "resource_visible"])]
    SubmitValidComment,
    #[valid(action_id = "SUBMIT_COMMENT_MISSING_BODY", reads = ["comment_body_len"], writes = ["outcome", "resource_visible"])]
    SubmitCommentMissingBody,
    #[valid(action_id = "SUBMIT_COMMENT_TOO_LONG", reads = ["comment_body_len"], writes = ["outcome", "resource_visible"])]
    SubmitCommentTooLong,
    #[valid(action_id = "SUBMIT_ANONYMOUS_COMMENT", reads = ["comment_body_len"], writes = ["comment_author_defaulted", "outcome", "resource_visible"])]
    SubmitAnonymousComment,
    #[valid(action_id = "REQUEST_MISSING_RESOURCE", reads = ["resource_visible"], writes = ["outcome", "resource_visible"])]
    RequestMissingResource,
    #[valid(action_id = "REQUEST_WRONG_EDIT_KEY", reads = ["resource_visible"], writes = ["outcome", "resource_visible"])]
    RequestWrongEditKey,
    #[valid(action_id = "SIMULATE_SERVER_ERROR", reads = ["resource_visible"], writes = ["outcome"])]
    SimulateServerError,
}

valid_model! {
    model CommonSpecModel<CommonSpecState, CommonSpecAction>;
    init [CommonSpecState {
        title_len: 1,
        body_len: 1,
        edit_key_len: 4,
        comment_body_len: 1,
        author_defaulted: false,
        comment_author_defaulted: false,
        html_escaped: true,
        resource_visible: true,
        outcome: ApiStatus::Idle,
    }];
    transitions {
        transition SetupMissingPostTitle [role = setup] [tags = ["boundary_path", "setup_path", "validation_path"]]
        when |state| true
        => [CommonSpecState
            {
                title_len : 0, body_len : 1, edit_key_len : 4, comment_body_len : 1,
                author_defaulted : false, comment_author_defaulted : false, html_escaped :
                true, resource_visible : true, outcome : ApiStatus::Idle
            }
        ];
        transition SetupMissingPostBody [role = setup] [tags = ["boundary_path", "setup_path", "validation_path"]]
        when |state| true
        => [CommonSpecState
            {
                title_len : 1, body_len : 0, edit_key_len : 4, comment_body_len : 1,
                author_defaulted : false, comment_author_defaulted : false, html_escaped :
                true, resource_visible : true, outcome : ApiStatus::Idle
            }
        ];
        transition SetupMissingPostEditKey [role = setup] [tags = ["boundary_path", "setup_path", "validation_path"]]
        when |state| true
        => [CommonSpecState
            {
                title_len : 1, body_len : 1, edit_key_len : 0, comment_body_len : 1,
                author_defaulted : false, comment_author_defaulted : false, html_escaped :
                true, resource_visible : true, outcome : ApiStatus::Idle
            }
        ];
        transition SetupTooLongPostInput [role = setup] [tags = ["boundary_path", "setup_path", "validation_path"]]
        when |state| true
        => [CommonSpecState
            {
                title_len : 101, body_len : 5001, edit_key_len : 33, comment_body_len : 1,
                author_defaulted : false, comment_author_defaulted : false, html_escaped :
                true, resource_visible : true, outcome : ApiStatus::Idle
            }
        ];
        transition SetupMissingCommentBody [role = setup] [tags = ["boundary_path", "comment_path", "setup_path", "validation_path"]]
        when |state| true
        => [CommonSpecState
            {
                title_len : 1, body_len : 1, edit_key_len : 4, comment_body_len : 0,
                author_defaulted : false, comment_author_defaulted : false, html_escaped :
                true, resource_visible : true, outcome : ApiStatus::Idle
            }
        ];
        transition SetupTooLongCommentBody [role = setup] [tags = ["boundary_path", "comment_path", "setup_path", "validation_path"]]
        when |state| true
        => [CommonSpecState
            {
                title_len : 1, body_len : 1, edit_key_len : 4, comment_body_len : 1001,
                author_defaulted : false, comment_author_defaulted : false, html_escaped :
                true, resource_visible : true, outcome : ApiStatus::Idle
            }
        ];
        transition SubmitValidPost [tags = ["allow_path", "create_path", "validation_path"]]
        when |state| state.title_len >= 1 && state.title_len <= 100 && state.body_len >= 1 &&
            state.body_len <= 5000 && state.edit_key_len >= 4 && state.edit_key_len <= 32
        => [CommonSpecState { outcome : ApiStatus::Ok, resource_visible : true, .. state }
        ];
        transition SubmitPostMissingTitle [tags = ["boundary_path", "deny_path", "validation_path"]]
        when |state| state.title_len == 0
        => [CommonSpecState
            { outcome : ApiStatus::BadRequest, resource_visible : false, .. state }
        ];
        transition SubmitPostMissingBody [tags = ["boundary_path", "deny_path", "validation_path"]]
        when |state| state.body_len == 0
        => [CommonSpecState
            { outcome : ApiStatus::BadRequest, resource_visible : false, .. state }
        ];
        transition SubmitPostMissingEditKey [tags = ["boundary_path", "deny_path", "validation_path"]]
        when |state| state.edit_key_len == 0
        => [CommonSpecState
            { outcome : ApiStatus::BadRequest, resource_visible : false, .. state }
        ];
        transition SubmitPostTooLong [tags = ["boundary_path", "deny_path", "validation_path"]]
        when |state| state.title_len == 101 || state.body_len == 5001 || state.edit_key_len == 33
        => [CommonSpecState
            { outcome : ApiStatus::BadRequest, resource_visible : false, .. state }
        ];
        transition SubmitAnonymousPost [tags = ["allow_path", "defaulting_path"]]
        when |state| state.title_len >= 1 && state.title_len <= 100 && state.body_len >= 1 &&
            state.body_len <= 5000 && state.edit_key_len >= 4 && state.edit_key_len <= 32
        => [CommonSpecState
            {
                author_defaulted : true, outcome : ApiStatus::Ok, resource_visible : true,
                .. state
            }
        ];
        transition SubmitValidComment [tags = ["allow_path", "comment_path", "validation_path"]]
        when |state| state.comment_body_len >= 1 && state.comment_body_len <= 1000
        => [CommonSpecState { outcome : ApiStatus::Ok, resource_visible : true, .. state }
        ];
        transition SubmitCommentMissingBody [tags = ["comment_path", "deny_path", "validation_path"]]
        when |state| state.comment_body_len == 0
        => [CommonSpecState
            { outcome : ApiStatus::BadRequest, resource_visible : false, .. state }
        ];
        transition SubmitCommentTooLong [tags = ["comment_path", "deny_path", "validation_path"]]
        when |state| state.comment_body_len == 1001
        => [CommonSpecState
            { outcome : ApiStatus::BadRequest, resource_visible : false, .. state }
        ];
        transition SubmitAnonymousComment [tags = ["allow_path", "comment_path", "defaulting_path"]]
        when |state| state.comment_body_len >= 1 && state.comment_body_len <= 1000
        => [CommonSpecState
            {
                comment_author_defaulted : true, outcome : ApiStatus::Ok, resource_visible
                : true, .. state
            }
        ];
        transition RequestMissingResource [tags = ["deny_path", "resource_path"]]
        when |state| state.resource_visible
        => [CommonSpecState
            { outcome : ApiStatus::NotFound, resource_visible : false, .. state }
        ];
        transition RequestWrongEditKey [tags = ["deny_path", "security_path"]]
        when |state| state.resource_visible
        => [CommonSpecState { outcome : ApiStatus::Forbidden, .. state }
        ];
        transition SimulateServerError [tags = ["exception_path", "retry_path"]]
        when |state| state.resource_visible
        => [CommonSpecState { outcome : ApiStatus::ServerError, .. state }
        ];
    }
    properties {
        invariant P_COMMON_HTML_IS_ALWAYS_ESCAPED |state|
            state.html_escaped;
        invariant P_COMMON_BAD_REQUEST_HIDES_INVALID_RESOURCE |state|
            state.outcome != ApiStatus::BadRequest || state.resource_visible == false;
        invariant P_COMMON_WRONG_EDIT_KEY_RETURNS_FORBIDDEN |state|
            state.outcome != ApiStatus::Forbidden || state.resource_visible;
        invariant P_COMMON_MISSING_RESOURCE_RETURNS_NOT_FOUND |state|
            state.outcome != ApiStatus::NotFound || state.resource_visible == false;
        invariant P_COMMON_ANONYMOUS_POST_DEFAULTS_AUTHOR |state|
            state.author_defaulted == false ||
            (state.title_len >= 1 && state.title_len <= 100 && state.body_len >= 1 &&
            state.body_len <= 5000 && state.edit_key_len >= 4 && state.edit_key_len <= 32);
        invariant P_COMMON_ANONYMOUS_COMMENT_DEFAULTS_AUTHOR |state|
            state.comment_author_defaulted == false ||
            (state.comment_body_len >= 1 && state.comment_body_len <= 1000);
    }
}
