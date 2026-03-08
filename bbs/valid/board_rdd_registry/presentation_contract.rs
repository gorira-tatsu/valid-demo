/*
Requirements covered:
- docs/rdd/01_common_specification.md
- docs/rdd/08_bbs_acceptance_requirements.md

This model covers:
- datetime display format
- HTML escaping and newline preservation
- presentation contracts for success messages and retry-message placement
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidState)]
struct PresentationContractState {
    timestamp_rendered: bool,
    #[valid(range = "0..=4")]
    year_digits: u8,
    #[valid(range = "0..=2")]
    month_digits: u8,
    #[valid(range = "0..=2")]
    day_digits: u8,
    #[valid(range = "0..=2")]
    hour_digits: u8,
    #[valid(range = "0..=2")]
    minute_digits: u8,
    #[valid(range = "0..=2")]
    hyphen_count: u8,
    #[valid(range = "0..=1")]
    colon_count: u8,
    #[valid(range = "0..=1")]
    space_count: u8,
    body_rendered: bool,
    html_escaped: bool,
    newline_preserved: bool,
    retry_message_visible: bool,
    retry_action_available: bool,
    #[valid(enum)]
    retry_message_placement: RetryMessagePlacement,
    #[valid(enum)]
    success_message: SuccessMessageKind,
    #[valid(enum)]
    outcome: ApiStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidAction)]
enum PresentationContractAction {
    #[valid(action_id = "RENDER_TIMESTAMP", reads = [], writes = ["timestamp_rendered", "year_digits", "month_digits", "day_digits", "hour_digits", "minute_digits", "hyphen_count", "colon_count", "space_count"])]
    RenderTimestamp,
    #[valid(action_id = "RENDER_ESCAPED_BODY_WITH_NEWLINES", reads = [], writes = ["body_rendered", "html_escaped", "newline_preserved"])]
    RenderEscapedBodyWithNewlines,
    #[valid(action_id = "SHOW_RETRY_MESSAGE_AT_TOP", reads = [], writes = ["retry_message_visible", "retry_action_available", "retry_message_placement", "outcome"])]
    ShowRetryMessageAtTop,
    #[valid(action_id = "SHOW_RETRY_MESSAGE_BELOW_FORM", reads = [], writes = ["retry_message_visible", "retry_action_available", "retry_message_placement", "outcome"])]
    ShowRetryMessageBelowForm,
    #[valid(action_id = "SHOW_POST_CREATED_SUCCESS_MESSAGE", reads = [], writes = ["success_message", "outcome"])]
    ShowPostCreatedSuccessMessage,
}

valid_model! {
    model PresentationContractModel<PresentationContractState, PresentationContractAction>;
    init [PresentationContractState {
        timestamp_rendered: false,
        year_digits: 0,
        month_digits: 0,
        day_digits: 0,
        hour_digits: 0,
        minute_digits: 0,
        hyphen_count: 0,
        colon_count: 0,
        space_count: 0,
        body_rendered: false,
        html_escaped: false,
        newline_preserved: false,
        retry_message_visible: false,
        retry_action_available: false,
        retry_message_placement: RetryMessagePlacement::None,
        success_message: SuccessMessageKind::None,
        outcome: ApiStatus::Idle,
    }];
    transitions {
        transition RenderTimestamp [tags = ["allow_path", "format_path", "render_path"]]
        when |state| state.timestamp_rendered == false
        => [PresentationContractState
            {
                timestamp_rendered : true, year_digits : 4, month_digits : 2, day_digits :
                2, hour_digits : 2, minute_digits : 2, hyphen_count : 2, colon_count : 1,
                space_count : 1, .. state
            }
        ];
        transition RenderEscapedBodyWithNewlines [tags = ["allow_path", "render_path", "sanitization_path"]]
        when |state| state.body_rendered == false
        => [PresentationContractState
            {
                body_rendered : true, html_escaped : true, newline_preserved : true, ..
                state
            }
        ];
        transition ShowRetryMessageAtTop [tags = ["exception_path", "retry_path", "ux_path"]]
        when |state| state.retry_message_visible == false
        => [PresentationContractState
            {
                retry_message_visible : true, retry_action_available : true,
                retry_message_placement : RetryMessagePlacement::TopBanner,
                success_message : SuccessMessageKind::None, outcome :
                ApiStatus::ServerError, .. state
            }
        ];
        transition ShowRetryMessageBelowForm [tags = ["exception_path", "retry_path", "ux_path"]]
        when |state| state.retry_message_visible == false
        => [PresentationContractState
            {
                retry_message_visible : true, retry_action_available : true,
                retry_message_placement : RetryMessagePlacement::BelowForm,
                success_message : SuccessMessageKind::None, outcome :
                ApiStatus::ServerError, .. state
            }
        ];
        transition ShowPostCreatedSuccessMessage [tags = ["allow_path", "create_path", "message_path"]]
        when |state| state.success_message == SuccessMessageKind::None
        => [PresentationContractState
            {
                success_message : SuccessMessageKind::PostCreatedCompleted, outcome :
                ApiStatus::Ok, .. state
            }
        ];
    }
    properties {
        invariant P_PRESENTATION_DATETIME_FORMAT_IS_YYYY_MM_DD_HH_MM |state|
            state.timestamp_rendered == false ||
            (state.year_digits == 4 && state.month_digits == 2 && state.day_digits == 2 &&
            state.hour_digits == 2 && state.minute_digits == 2 && state.hyphen_count == 2
            && state.colon_count == 1 && state.space_count == 1);
        invariant P_PRESENTATION_BODY_RENDERING_ESCAPES_HTML_AND_PRESERVES_NEWLINES |state|
            state.body_rendered == false ||
            (state.html_escaped && state.newline_preserved);
        invariant P_PRESENTATION_SERVER_ERROR_EXPOSES_RETRY_MESSAGE_WITH_PLACEMENT |state|
            state.outcome != ApiStatus::ServerError ||
            (state.retry_message_visible && state.retry_action_available &&
            state.retry_message_placement != RetryMessagePlacement::None);
        invariant P_PRESENTATION_SUCCESS_MESSAGE_IS_EXPLICIT |state|
            state.success_message == SuccessMessageKind::None ||
            (state.success_message == SuccessMessageKind::PostCreatedCompleted &&
            state.outcome == ApiStatus::Ok);
    }
}
