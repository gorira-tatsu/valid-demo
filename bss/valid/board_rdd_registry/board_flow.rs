/*
対応する要件定義:
- docs/rdd/08_BBS成立要件.md

この model が担うこと:
- 投稿作成から一覧、詳細、更新、コメント、削除までの横断整合
- 更新後の詳細取得失敗から retry 回復後の整合復元
- 再取得を跨いだ `updatedAt` とコメント件数整合
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidState)]
struct BoardFlowState {
    post_exists: bool,
    post_deleted: bool,
    post_updated: bool,
    list_visible_posts: u8,
    list_detail_navigation_available: bool,
    list_excerpt_matches_detail: bool,
    #[valid(range = "0..=10")]
    list_comment_count: u8,
    detail_screen_visible: bool,
    detail_edit_actions_visible: bool,
    #[valid(range = "0..=10")]
    detail_comment_count: u8,
    detail_updated_at_visible: bool,
    comment_submission_allowed: bool,
    detail_error_visible: bool,
    detail_retry_action_available: bool,
    detail_return_navigation_available: bool,
    #[valid(enum)]
    detail_retry_phase: RetryPhase,
    #[valid(enum)]
    outcome: ApiStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidAction)]
enum BoardFlowAction {
    #[valid(action_id = "CREATE_POST_FLOW", reads = [], writes = ["post_exists", "post_deleted", "post_updated", "list_visible_posts", "list_detail_navigation_available", "list_excerpt_matches_detail", "list_comment_count", "detail_screen_visible", "detail_edit_actions_visible", "detail_comment_count", "detail_updated_at_visible", "comment_submission_allowed", "detail_error_visible", "detail_retry_action_available", "detail_return_navigation_available", "detail_retry_phase", "outcome"])]
    CreatePostFlow,
    #[valid(action_id = "OPEN_LIST_ACTIVE_FLOW", reads = ["post_exists", "post_deleted", "detail_comment_count", "post_updated"], writes = ["list_visible_posts", "list_detail_navigation_available", "list_excerpt_matches_detail", "list_comment_count", "detail_error_visible", "detail_retry_action_available", "detail_return_navigation_available", "detail_retry_phase", "outcome"])]
    OpenListActiveFlow,
    #[valid(action_id = "OPEN_LIST_DELETED_FLOW", reads = ["post_exists", "post_deleted"], writes = ["list_visible_posts", "list_detail_navigation_available", "list_comment_count", "detail_error_visible", "detail_retry_action_available", "detail_return_navigation_available", "detail_retry_phase", "outcome"])]
    OpenListDeletedFlow,
    #[valid(action_id = "OPEN_DETAIL_ACTIVE_FLOW", reads = ["post_exists", "post_deleted", "list_comment_count", "post_updated"], writes = ["detail_screen_visible", "detail_edit_actions_visible", "detail_comment_count", "detail_updated_at_visible", "comment_submission_allowed", "detail_error_visible", "detail_retry_action_available", "detail_return_navigation_available", "detail_retry_phase", "outcome"])]
    OpenDetailActiveFlow,
    #[valid(action_id = "OPEN_DETAIL_DELETED_FLOW", reads = ["post_exists", "post_deleted"], writes = ["detail_screen_visible", "detail_edit_actions_visible", "detail_updated_at_visible", "comment_submission_allowed", "detail_error_visible", "detail_retry_action_available", "detail_return_navigation_available", "detail_retry_phase", "outcome"])]
    OpenDetailDeletedFlow,
    #[valid(action_id = "EDIT_POST_FLOW", reads = ["post_exists", "post_deleted", "detail_retry_phase"], writes = ["post_updated", "list_excerpt_matches_detail", "detail_updated_at_visible", "outcome"])]
    EditPostFlow,
    #[valid(action_id = "DELETE_POST_FLOW", reads = ["post_exists", "post_deleted", "detail_retry_phase"], writes = ["post_deleted", "list_visible_posts", "list_detail_navigation_available", "list_comment_count", "detail_screen_visible", "detail_edit_actions_visible", "detail_comment_count", "detail_updated_at_visible", "comment_submission_allowed", "detail_error_visible", "detail_retry_action_available", "detail_return_navigation_available", "detail_retry_phase", "outcome"])]
    DeletePostFlow,
    #[valid(action_id = "COMMENT_ON_ACTIVE_POST_FLOW", reads = ["post_exists", "post_deleted", "list_comment_count", "detail_comment_count", "comment_submission_allowed", "detail_screen_visible"], writes = ["list_comment_count", "detail_comment_count", "comment_submission_allowed", "outcome"])]
    CommentOnActivePostFlow,
    #[valid(action_id = "COMMENT_ON_DELETED_POST_FLOW", reads = ["post_exists", "post_deleted"], writes = ["comment_submission_allowed", "outcome"])]
    CommentOnDeletedPostFlow,
    #[valid(action_id = "FAIL_DETAIL_LOAD_AFTER_UPDATE_FLOW", reads = ["post_exists", "post_deleted", "post_updated", "list_comment_count"], writes = ["detail_screen_visible", "detail_edit_actions_visible", "detail_comment_count", "detail_updated_at_visible", "comment_submission_allowed", "detail_error_visible", "detail_retry_action_available", "detail_return_navigation_available", "detail_retry_phase", "outcome"])]
    FailDetailLoadAfterUpdateFlow,
    #[valid(action_id = "RETRY_DETAIL_LOAD_FLOW", reads = ["post_exists", "post_deleted", "detail_retry_phase", "detail_retry_action_available"], writes = ["detail_error_visible", "detail_retry_action_available", "detail_retry_phase", "outcome"])]
    RetryDetailLoadFlow,
    #[valid(action_id = "RECOVER_DETAIL_LOAD_AFTER_RETRY_FLOW", reads = ["post_exists", "post_deleted", "post_updated", "list_comment_count", "detail_retry_phase"], writes = ["detail_screen_visible", "detail_edit_actions_visible", "detail_comment_count", "detail_updated_at_visible", "comment_submission_allowed", "detail_error_visible", "detail_retry_action_available", "detail_return_navigation_available", "detail_retry_phase", "outcome"])]
    RecoverDetailLoadAfterRetryFlow,
}

valid_model! {
    model BoardFlowModel<BoardFlowState, BoardFlowAction>;
    init [BoardFlowState {
        post_exists: false,
        post_deleted: false,
        post_updated: false,
        list_visible_posts: 0,
        list_detail_navigation_available: false,
        list_excerpt_matches_detail: false,
        list_comment_count: 0,
        detail_screen_visible: false,
        detail_edit_actions_visible: false,
        detail_comment_count: 0,
        detail_updated_at_visible: false,
        comment_submission_allowed: false,
        detail_error_visible: false,
        detail_retry_action_available: false,
        detail_return_navigation_available: false,
        detail_retry_phase: RetryPhase::Idle,
        outcome: ApiStatus::Idle,
    }];
    transitions {
        transition CreatePostFlow [tags = ["allow_path", "create_path", "integration_path"]]
        when |state| state.post_exists == false
        => [BoardFlowState
            {
                post_exists : true, post_deleted : false, post_updated : false,
                list_visible_posts : 1, list_detail_navigation_available : true,
                list_excerpt_matches_detail : true, list_comment_count : 0,
                detail_screen_visible : true, detail_edit_actions_visible : true,
                detail_comment_count : 0, detail_updated_at_visible : false,
                comment_submission_allowed : true, detail_error_visible : false,
                detail_retry_action_available : false, detail_return_navigation_available
                : false, detail_retry_phase : RetryPhase::Idle, outcome : ApiStatus::Ok
            }
        ];
        transition OpenListActiveFlow [tags = ["allow_path", "integration_path", "list_path"]]
        when |state| state.post_exists && state.post_deleted == false
        => [BoardFlowState
            {
                list_visible_posts : 1, list_detail_navigation_available : true,
                list_excerpt_matches_detail : true, list_comment_count :
                state.detail_comment_count, detail_error_visible : false,
                detail_retry_action_available : false, detail_return_navigation_available
                : false, detail_retry_phase : RetryPhase::Idle, outcome : ApiStatus::Ok,
                .. state
            }
        ];
        transition OpenListDeletedFlow [tags = ["deny_path", "integration_path", "list_path"]]
        when |state| state.post_exists && state.post_deleted
        => [BoardFlowState
            {
                list_visible_posts : 0, list_detail_navigation_available : false,
                list_comment_count : 0, detail_error_visible : false,
                detail_retry_action_available : false, detail_return_navigation_available
                : false, detail_retry_phase : RetryPhase::Idle, outcome : ApiStatus::Ok,
                .. state
            }
        ];
        transition OpenDetailActiveFlow [tags = ["allow_path", "detail_path", "integration_path"]]
        when |state| state.post_exists && state.post_deleted == false
        => [BoardFlowState
            {
                detail_screen_visible : true, detail_edit_actions_visible : true,
                detail_comment_count : state.list_comment_count, detail_updated_at_visible
                : state.post_updated, comment_submission_allowed : true,
                detail_error_visible : false, detail_retry_action_available : false,
                detail_return_navigation_available : false, detail_retry_phase :
                RetryPhase::Idle, outcome : ApiStatus::Ok, .. state
            }
        ];
        transition OpenDetailDeletedFlow [tags = ["allow_path", "deny_path", "detail_path", "integration_path"]]
        when |state| state.post_exists && state.post_deleted
        => [BoardFlowState
            {
                detail_screen_visible : false, detail_edit_actions_visible : false,
                detail_updated_at_visible : false, comment_submission_allowed : false,
                detail_error_visible : false, detail_retry_action_available : false,
                detail_return_navigation_available : false, detail_retry_phase :
                RetryPhase::Idle, outcome : ApiStatus::NotFound, .. state
            }
        ];
        transition EditPostFlow [tags = ["allow_path", "edit_path", "integration_path"]]
        when |state| state.post_exists && state.post_deleted == false && state.detail_retry_phase
            != RetryPhase::ErrorShown && state.detail_retry_phase != RetryPhase::Retrying
        => [BoardFlowState
            {
                post_updated : true, list_excerpt_matches_detail : true,
                detail_updated_at_visible : true, outcome : ApiStatus::Ok, .. state
            }
        ];
        transition DeletePostFlow [tags = ["allow_path", "delete_path", "integration_path"]]
        when |state| state.post_exists && state.post_deleted == false && state.detail_retry_phase
            != RetryPhase::ErrorShown && state.detail_retry_phase != RetryPhase::Retrying
        => [BoardFlowState
            {
                post_deleted : true, list_visible_posts : 0,
                list_detail_navigation_available : false, list_comment_count : 0,
                detail_screen_visible : false, detail_edit_actions_visible : false,
                detail_comment_count : 0, detail_updated_at_visible : false,
                comment_submission_allowed : false, detail_error_visible : false,
                detail_retry_action_available : false, detail_return_navigation_available
                : false, detail_retry_phase : RetryPhase::Idle, outcome : ApiStatus::Ok,
                .. state
            }
        ];
        transition CommentOnActivePostFlow [tags = ["allow_path", "comment_path", "integration_path"]]
        when |state| state.post_exists && state.post_deleted == false &&
            state.detail_screen_visible && state.comment_submission_allowed &&
            state.list_comment_count <= 9 && state.detail_comment_count <= 9
        => [BoardFlowState
            {
                list_comment_count : state.list_comment_count + 1, detail_comment_count :
                state.detail_comment_count + 1, comment_submission_allowed : true, outcome
                : ApiStatus::Ok, .. state
            }
        ];
        transition CommentOnDeletedPostFlow [tags = ["allow_path", "comment_path", "deny_path", "integration_path"]]
        when |state| state.post_exists && state.post_deleted
        => [BoardFlowState
            {
                comment_submission_allowed : false, outcome : ApiStatus::NotFound, ..
                state
            }
        ];
        transition FailDetailLoadAfterUpdateFlow [tags = ["allow_path", "detail_path", "exception_path", "integration_path", "retry_path"]]
        when |state| state.post_exists && state.post_deleted == false && state.post_updated
        => [BoardFlowState
            {
                detail_screen_visible : false, detail_edit_actions_visible : false,
                detail_comment_count : state.list_comment_count, detail_updated_at_visible
                : false, comment_submission_allowed : false, detail_error_visible : true,
                detail_retry_action_available : true, detail_return_navigation_available :
                true, detail_retry_phase : RetryPhase::ErrorShown, outcome :
                ApiStatus::ServerError, .. state
            }
        ];
        transition RetryDetailLoadFlow [tags = ["detail_path", "integration_path", "recovery_path", "retry_path"]]
        when |state| state.post_exists && state.post_deleted == false && state.detail_retry_phase
            == RetryPhase::ErrorShown && state.detail_retry_action_available
        => [BoardFlowState
            {
                detail_error_visible : false, detail_retry_action_available : false,
                detail_retry_phase : RetryPhase::Retrying, outcome :
                ApiStatus::ServerError, .. state
            }
        ];
        transition RecoverDetailLoadAfterRetryFlow [tags = ["allow_path", "detail_path", "integration_path", "recovery_path", "retry_path"]]
        when |state| state.post_exists && state.post_deleted == false && state.detail_retry_phase
            == RetryPhase::Retrying
        => [BoardFlowState
            {
                detail_screen_visible : true, detail_edit_actions_visible : true,
                detail_comment_count : state.list_comment_count, detail_updated_at_visible
                : state.post_updated, comment_submission_allowed : true,
                detail_error_visible : false, detail_retry_action_available : false,
                detail_return_navigation_available : false, detail_retry_phase :
                RetryPhase::Recovered, outcome : ApiStatus::Ok, .. state
            }
        ];
    }
    properties {
        invariant P_FLOW_DELETED_POST_IS_HIDDEN_FROM_LIST |state|
            state.post_deleted == false || state.list_visible_posts == 0;
        invariant P_FLOW_DELETED_POST_IS_HIDDEN_FROM_DETAIL |state|
            state.post_deleted == false || state.detail_screen_visible == false;
        invariant P_FLOW_DELETED_POST_DISALLOWS_COMMENT |state|
            state.post_deleted == false || state.comment_submission_allowed == false;
        invariant P_FLOW_VISIBLE_DETAIL_REQUIRES_LIST_NAVIGATION |state|
            state.detail_screen_visible == false || state.list_detail_navigation_available;
        invariant P_FLOW_LIST_AND_DETAIL_COMMENT_COUNTS_MATCH |state|
            state.post_deleted || state.list_comment_count == state.detail_comment_count;
        invariant P_FLOW_UPDATED_POST_SHOWS_UPDATED_AT_ON_DETAIL |state|
            state.post_updated == false || state.detail_screen_visible == false ||
            state.detail_updated_at_visible;
        invariant P_FLOW_UPDATED_POST_KEEPS_LIST_AND_DETAIL_CONTENT_ALIGNED |state|
            state.post_updated == false || state.list_excerpt_matches_detail;
        invariant P_FLOW_DETAIL_LOAD_FAILURE_EXPOSES_RETRY_AND_RETURN_PATH |state|
            state.detail_retry_phase != RetryPhase::ErrorShown ||
            (state.detail_error_visible && state.detail_retry_action_available &&
            state.detail_return_navigation_available && state.outcome ==
            ApiStatus::ServerError && state.detail_screen_visible == false);
        invariant P_FLOW_DETAIL_RECOVERY_RESTORES_UPDATED_AND_COMMENT_CONSISTENCY |state|
            state.detail_retry_phase != RetryPhase::Recovered ||
            (state.detail_screen_visible && state.detail_comment_count ==
            state.list_comment_count && state.detail_updated_at_visible ==
            state.post_updated && state.detail_error_visible == false &&
            state.detail_retry_action_available == false && state.outcome ==
            ApiStatus::Ok);
    }
}
