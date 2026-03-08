/*
対応する要件定義:
- docs/rdd/01_共通仕様.md
- docs/rdd/03_投稿作成機能.md

この model が担うこと:
- 匿名投稿時の補完
- 成功時の詳細遷移
- 失敗時の入力保持
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidState)]
struct PostCreateState {
    #[valid(range = "0..=10")]
    post_count: u8,
    #[valid(range = "0..=101")]
    title_len: u8,
    #[valid(range = "0..=5001")]
    body_len: u16,
    #[valid(range = "0..=33")]
    edit_key_len: u8,
    author_defaulted: bool,
    created_at_recorded: bool,
    navigated_to_detail: bool,
    form_preserved: bool,
    submit_disabled: bool,
    #[valid(enum)]
    phase: CreatePhase,
    #[valid(enum)]
    outcome: ApiStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidAction)]
enum PostCreateAction {
    #[valid(action_id = "BEGIN_SUBMIT", reads = ["phase"], writes = ["phase", "submit_disabled"])]
    BeginSubmit,
    #[valid(action_id = "CREATE_VALID_POST", reads = ["title_len", "body_len", "edit_key_len"], writes = ["post_count", "created_at_recorded", "navigated_to_detail", "form_preserved", "submit_disabled", "phase", "outcome"])]
    CreateValidPost,
    #[valid(action_id = "CREATE_ANONYMOUS_POST", reads = ["title_len", "body_len", "edit_key_len"], writes = ["post_count", "author_defaulted", "created_at_recorded", "navigated_to_detail", "form_preserved", "submit_disabled", "phase", "outcome"])]
    CreateAnonymousPost,
    #[valid(action_id = "CREATE_INVALID_POST", reads = ["title_len", "body_len", "edit_key_len"], writes = ["form_preserved", "submit_disabled", "phase", "outcome"])]
    CreateInvalidPost,
    #[valid(action_id = "CREATE_SAVE_FAILURE", reads = ["phase"], writes = ["form_preserved", "submit_disabled", "phase", "outcome"])]
    CreateSaveFailure,
}

valid_model! {
    model PostCreateModel<PostCreateState, PostCreateAction>;
    init [PostCreateState {
        post_count: 0,
        title_len: 1,
        body_len: 1,
        edit_key_len: 4,
        author_defaulted: false,
        created_at_recorded: false,
        navigated_to_detail: false,
        form_preserved: false,
        submit_disabled: false,
        phase: CreatePhase::Editing,
        outcome: ApiStatus::Idle,
    }];
    transitions {
        transition BeginSubmit [tags = ["allow_path", "create_path", "submit_path"]]
        when |state| state.phase == CreatePhase::Editing
        => [PostCreateState
            { phase : CreatePhase::Submitting, submit_disabled : true, .. state }
        ];
        transition CreateValidPost [tags = ["allow_path", "create_path"]]
        when |state| state.phase == CreatePhase::Submitting && state.title_len >= 1 &&
            state.title_len <= 100 && state.body_len >= 1 && state.body_len <= 5000 &&
            state.edit_key_len >= 4 && state.edit_key_len <= 32 && state.post_count <= 9
        => [PostCreateState
            {
                post_count : state.post_count + 1, created_at_recorded : true,
                navigated_to_detail : true, form_preserved : false, submit_disabled :
                false, phase : CreatePhase::Success, outcome : ApiStatus::Ok, .. state
            }
        ];
        transition CreateAnonymousPost [tags = ["allow_path", "create_path", "defaulting_path"]]
        when |state| state.phase == CreatePhase::Submitting && state.title_len >= 1 &&
            state.title_len <= 100 && state.body_len >= 1 && state.body_len <= 5000 &&
            state.edit_key_len >= 4 && state.edit_key_len <= 32 && state.post_count <= 9
        => [PostCreateState
            {
                post_count : state.post_count + 1, author_defaulted : true,
                created_at_recorded : true, navigated_to_detail : true, form_preserved :
                false, submit_disabled : false, phase : CreatePhase::Success, outcome :
                ApiStatus::Ok, .. state
            }
        ];
        transition CreateInvalidPost [tags = ["create_path", "deny_path", "validation_path"]]
        when |state| state.phase == CreatePhase::Submitting &&
            (state.title_len == 0 || state.title_len == 101 || state.body_len == 0 ||
            state.body_len == 5001 || state.edit_key_len == 0 || state.edit_key_len == 33)
        => [PostCreateState
            {
                form_preserved : true, submit_disabled : false, phase :
                CreatePhase::Failure, outcome : ApiStatus::BadRequest, .. state
            }
        ];
        transition CreateSaveFailure [tags = ["create_path", "exception_path", "retry_path"]]
        when |state| state.phase == CreatePhase::Submitting
        => [PostCreateState
            {
                form_preserved : true, submit_disabled : false, phase :
                CreatePhase::Failure, outcome : ApiStatus::ServerError, .. state
            }
        ];
    }
    properties {
        invariant P_CREATE_SUCCESS_RECORDS_CREATED_AT |state|
            state.phase != CreatePhase::Success || state.created_at_recorded;
        invariant P_CREATE_SUCCESS_NAVIGATES_TO_DETAIL |state|
            state.phase != CreatePhase::Success || state.navigated_to_detail;
        invariant P_CREATE_FAILURE_PRESERVES_FORM |state|
            state.phase != CreatePhase::Failure || state.form_preserved;
        invariant P_CREATE_SUBMITTING_DISABLES_SUBMIT |state|
            state.phase != CreatePhase::Submitting || state.submit_disabled;
        invariant P_CREATE_ANONYMOUS_SUCCESS_DEFAULTS_AUTHOR |state|
            state.author_defaulted == false || state.phase == CreatePhase::Success;
    }
}
