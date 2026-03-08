/*
対応する要件定義:
- docs/rdd/01_共通仕様.md
- docs/rdd/08_BBS成立要件.md

この model が担うこと:
- 一覧失敗時は TopBanner、フォーム失敗時は BelowForm に出すこと
- ErrorShown -> Retrying -> Recovered の retry 回復遷移
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidState)]
struct RetryUxState {
    #[valid(enum)]
    screen: ScreenContext,
    retry_message_visible: bool,
    retry_action_available: bool,
    #[valid(enum)]
    retry_message_placement: RetryMessagePlacement,
    #[valid(enum)]
    phase: RetryPhase,
    #[valid(enum)]
    outcome: ApiStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidAction)]
enum RetryUxAction {
    #[valid(action_id = "FAIL_LIST_LOAD", reads = ["screen", "phase"], writes = ["retry_message_visible", "retry_action_available", "retry_message_placement", "phase", "outcome"])]
    FailListLoad,
    #[valid(action_id = "FAIL_CREATE_SUBMIT", reads = ["screen", "phase"], writes = ["retry_message_visible", "retry_action_available", "retry_message_placement", "phase", "outcome"])]
    FailCreateSubmit,
    #[valid(action_id = "FAIL_EDIT_SUBMIT", reads = ["screen", "phase"], writes = ["retry_message_visible", "retry_action_available", "retry_message_placement", "phase", "outcome"])]
    FailEditSubmit,
    #[valid(action_id = "FAIL_COMMENT_SUBMIT", reads = ["screen", "phase"], writes = ["retry_message_visible", "retry_action_available", "retry_message_placement", "phase", "outcome"])]
    FailCommentSubmit,
    #[valid(action_id = "RETRY_AFTER_FAILURE", reads = ["retry_action_available", "phase"], writes = ["phase"])]
    RetryAfterFailure,
    #[valid(action_id = "RECOVER_AFTER_RETRY", reads = ["phase"], writes = ["retry_message_visible", "retry_action_available", "retry_message_placement", "phase", "outcome"])]
    RecoverAfterRetry,
}

valid_model! {
    model RetryUxModel<RetryUxState, RetryUxAction>;
    init [RetryUxState {
        screen: ScreenContext::ListScreen,
        retry_message_visible: false,
        retry_action_available: false,
        retry_message_placement: RetryMessagePlacement::None,
        phase: RetryPhase::Idle,
        outcome: ApiStatus::Idle,
    }];
    transitions {
        transition FailListLoad [tags = ["exception_path", "list_path", "retry_path", "ux_path"]]
        when |state| state.phase == RetryPhase::Idle
        => [RetryUxState
            {
                screen : ScreenContext::ListScreen, retry_message_visible : true,
                retry_action_available : true, retry_message_placement :
                RetryMessagePlacement::TopBanner, phase : RetryPhase::ErrorShown, outcome
                : ApiStatus::ServerError
            }
        ];
        transition FailCreateSubmit [tags = ["create_path", "exception_path", "retry_path", "ux_path"]]
        when |state| state.phase == RetryPhase::Idle
        => [RetryUxState
            {
                screen : ScreenContext::CreateForm, retry_message_visible : true,
                retry_action_available : true, retry_message_placement :
                RetryMessagePlacement::BelowForm, phase : RetryPhase::ErrorShown, outcome
                : ApiStatus::ServerError
            }
        ];
        transition FailEditSubmit [tags = ["edit_path", "exception_path", "retry_path", "ux_path"]]
        when |state| state.phase == RetryPhase::Idle
        => [RetryUxState
            {
                screen : ScreenContext::EditForm, retry_message_visible : true,
                retry_action_available : true, retry_message_placement :
                RetryMessagePlacement::BelowForm, phase : RetryPhase::ErrorShown, outcome
                : ApiStatus::ServerError
            }
        ];
        transition FailCommentSubmit [tags = ["comment_path", "exception_path", "retry_path", "ux_path"]]
        when |state| state.phase == RetryPhase::Idle
        => [RetryUxState
            {
                screen : ScreenContext::CommentForm, retry_message_visible : true,
                retry_action_available : true, retry_message_placement :
                RetryMessagePlacement::BelowForm, phase : RetryPhase::ErrorShown, outcome
                : ApiStatus::ServerError
            }
        ];
        transition RetryAfterFailure [tags = ["allow_path", "recovery_path", "retry_path"]]
        when |state| state.retry_action_available && state.phase == RetryPhase::ErrorShown
        => [RetryUxState { phase : RetryPhase::Retrying, .. state }
        ];
        transition RecoverAfterRetry [tags = ["allow_path", "recovery_path", "retry_path"]]
        when |state| state.phase == RetryPhase::Retrying
        => [RetryUxState
            {
                retry_message_visible : false, retry_action_available : false,
                retry_message_placement : RetryMessagePlacement::None, phase :
                RetryPhase::Recovered, outcome : ApiStatus::Ok, .. state
            }
        ];
    }
    properties {
        invariant P_RETRY_LIST_FAILURE_USES_TOP_BANNER |state|
            !(state.screen == ScreenContext::ListScreen && state.phase ==
            RetryPhase::ErrorShown) || state.retry_message_placement ==
            RetryMessagePlacement::TopBanner;
        invariant P_RETRY_FORM_FAILURE_USES_BELOW_FORM_MESSAGE |state|
            !((state.screen == ScreenContext::CreateForm || state.screen ==
            ScreenContext::EditForm || state.screen == ScreenContext::CommentForm) &&
            state.phase == RetryPhase::ErrorShown) || state.retry_message_placement ==
            RetryMessagePlacement::BelowForm;
        invariant P_RETRY_ERROR_STATE_IS_ACTIONABLE |state|
            state.phase != RetryPhase::ErrorShown ||
            (state.retry_message_visible && state.retry_action_available && state.outcome
            == ApiStatus::ServerError);
        invariant P_RETRY_RECOVERY_CLEARS_ERROR_MESSAGE |state|
            state.phase != RetryPhase::Recovered ||
            (state.retry_message_visible == false && state.retry_action_available == false
            && state.retry_message_placement == RetryMessagePlacement::None &&
            state.outcome == ApiStatus::Ok);
        invariant P_RETRY_RECOVERY_DOES_NOT_KEEP_STALE_SERVER_ERROR |state|
            state.phase != RetryPhase::Recovered || state.outcome !=
            ApiStatus::ServerError;
    }
}
