/*
対応する要件定義:
- docs/rdd/03_投稿作成機能.md
- docs/rdd/06_コメント機能.md
- docs/rdd/08_BBS成立要件.md

この model が担うこと:
- 作成/コメント送信中の多重送信防止
- 失敗後の再送可能性
- 回復後の通常フロー復帰
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidState)]
struct SubmissionDisciplineState {
    #[valid(enum)]
    context: SubmissionContext,
    submit_disabled: bool,
    request_in_flight: bool,
    duplicate_submit_blocked: bool,
    retry_available: bool,
    form_preserved: bool,
    #[valid(enum)]
    phase: SubmissionPhase,
    #[valid(enum)]
    outcome: ApiStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidAction)]
enum SubmissionDisciplineAction {
    #[valid(action_id = "BEGIN_CREATE_SUBMIT", reads = ["phase"], writes = ["context", "submit_disabled", "request_in_flight", "duplicate_submit_blocked", "retry_available", "form_preserved", "phase", "outcome"])]
    BeginCreateSubmit,
    #[valid(action_id = "BEGIN_COMMENT_SUBMIT", reads = ["phase"], writes = ["context", "submit_disabled", "request_in_flight", "duplicate_submit_blocked", "retry_available", "form_preserved", "phase", "outcome"])]
    BeginCommentSubmit,
    #[valid(action_id = "REJECT_DUPLICATE_SUBMIT", reads = ["request_in_flight"], writes = ["duplicate_submit_blocked", "outcome"])]
    RejectDuplicateSubmit,
    #[valid(action_id = "FAIL_SUBMIT", reads = ["request_in_flight"], writes = ["submit_disabled", "request_in_flight", "retry_available", "form_preserved", "phase", "outcome"])]
    FailSubmit,
    #[valid(action_id = "RETRY_SUBMIT", reads = ["retry_available", "phase"], writes = ["submit_disabled", "request_in_flight", "duplicate_submit_blocked", "retry_available", "form_preserved", "phase", "outcome"])]
    RetrySubmit,
    #[valid(action_id = "COMPLETE_SUBMIT", reads = ["request_in_flight"], writes = ["submit_disabled", "request_in_flight", "retry_available", "form_preserved", "phase", "outcome"])]
    CompleteSubmit,
}

valid_model! {
    model SubmissionDisciplineModel<SubmissionDisciplineState, SubmissionDisciplineAction>;
    init [SubmissionDisciplineState {
        context: SubmissionContext::CreatePost,
        submit_disabled: false,
        request_in_flight: false,
        duplicate_submit_blocked: false,
        retry_available: false,
        form_preserved: false,
        phase: SubmissionPhase::Idle,
        outcome: ApiStatus::Idle,
    }];
    transitions {
        transition BeginCreateSubmit [tags = ["allow_path", "create_path", "state_gate_path", "submit_path"]]
        when |state| state.phase == SubmissionPhase::Idle || state.phase ==
            SubmissionPhase::Recovered
        => [SubmissionDisciplineState
            {
                context : SubmissionContext::CreatePost, submit_disabled : true,
                request_in_flight : true, duplicate_submit_blocked : false,
                retry_available : false, form_preserved : false, phase :
                SubmissionPhase::Submitting, outcome : ApiStatus::Idle
            }
        ];
        transition BeginCommentSubmit [tags = ["allow_path", "comment_path", "state_gate_path", "submit_path"]]
        when |state| state.phase == SubmissionPhase::Idle || state.phase ==
            SubmissionPhase::Recovered
        => [SubmissionDisciplineState
            {
                context : SubmissionContext::CommentPost, submit_disabled : true,
                request_in_flight : true, duplicate_submit_blocked : false,
                retry_available : false, form_preserved : false, phase :
                SubmissionPhase::Submitting, outcome : ApiStatus::Idle
            }
        ];
        transition RejectDuplicateSubmit [tags = ["deny_path", "duplicate_path", "state_gate_path", "submit_path"]]
        when |state| state.request_in_flight
        => [SubmissionDisciplineState
            { duplicate_submit_blocked : true, outcome : ApiStatus::BadRequest, .. state }
        ];
        transition FailSubmit [tags = ["exception_path", "retry_path", "submit_path"]]
        when |state| state.request_in_flight
        => [SubmissionDisciplineState
            {
                submit_disabled : false, request_in_flight : false, retry_available :
                true, form_preserved : true, phase : SubmissionPhase::Failed, outcome :
                ApiStatus::ServerError, .. state
            }
        ];
        transition RetrySubmit [tags = ["allow_path", "recovery_path", "retry_path", "state_gate_path", "submit_path"]]
        when |state| state.retry_available && state.phase == SubmissionPhase::Failed
        => [SubmissionDisciplineState
            {
                submit_disabled : true, request_in_flight : true, duplicate_submit_blocked
                : false, retry_available : false, form_preserved : true, phase :
                SubmissionPhase::Recovered, outcome : ApiStatus::Idle, .. state
            }
        ];
        transition CompleteSubmit [tags = ["allow_path", "state_gate_path", "submit_path"]]
        when |state| state.request_in_flight
        => [SubmissionDisciplineState
            {
                submit_disabled : false, request_in_flight : false,
                duplicate_submit_blocked : false, retry_available : false, form_preserved
                : false, phase : SubmissionPhase::Succeeded, outcome : ApiStatus::Ok, ..
                state
            }
        ];
    }
    properties {
        invariant P_SUBMIT_IN_FLIGHT_DISALLOWS_SECOND_SUBMIT |state|
            state.request_in_flight == false || state.submit_disabled;
        invariant P_SUBMIT_DUPLICATE_ATTEMPT_IS_BLOCKED |state|
            state.duplicate_submit_blocked == false || state.request_in_flight ||
            state.phase == SubmissionPhase::Failed;
        invariant P_SUBMIT_FAILURE_ENSURES_RETRYABLE_FORM |state|
            state.phase != SubmissionPhase::Failed ||
            (state.retry_available && state.form_preserved && state.outcome ==
            ApiStatus::ServerError);
        invariant P_SUBMIT_SUCCESS_OR_RECOVERY_REENABLES_NORMAL_FLOW |state|
            !((state.phase == SubmissionPhase::Succeeded) ||
            (state.phase == SubmissionPhase::Recovered && state.request_in_flight ==
            false)) || (state.submit_disabled == false && state.retry_available == false);
    }
}
