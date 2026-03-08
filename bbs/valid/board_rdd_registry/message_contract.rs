/*
対応する要件定義:
- docs/rdd/02_投稿一覧機能.md
- docs/rdd/05_投稿編集・削除機能.md

この model が担うこと:
- 空状態文言の束縛
- 投稿成功文言の束縛
- 編集用キー不一致文言の束縛
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidState)]
struct MessageContractState {
    #[valid(enum)]
    screen: ScreenContext,
    #[valid(enum)]
    message_template: MessageTemplate,
    visible: bool,
    #[valid(enum)]
    outcome: ApiStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidAction)]
enum MessageContractAction {
    #[valid(action_id = "SHOW_EMPTY_POST_LIST_MESSAGE", reads = ["screen"], writes = ["screen", "message_template", "visible", "outcome"])]
    ShowEmptyPostListMessage,
    #[valid(action_id = "SHOW_POST_CREATED_COMPLETED_MESSAGE", reads = ["screen"], writes = ["screen", "message_template", "visible", "outcome"])]
    ShowPostCreatedCompletedMessage,
    #[valid(action_id = "SHOW_INVALID_EDIT_KEY_MESSAGE", reads = ["screen"], writes = ["screen", "message_template", "visible", "outcome"])]
    ShowInvalidEditKeyMessage,
}

valid_model! {
    model MessageContractModel<MessageContractState, MessageContractAction>;
    init [MessageContractState {
        screen: ScreenContext::ListScreen,
        message_template: MessageTemplate::None,
        visible: false,
        outcome: ApiStatus::Idle,
    }];
    transitions {
        transition ShowEmptyPostListMessage [tags = ["allow_path", "list_path", "message_path"]]
        when |state| true
        => [MessageContractState
            {
                screen : ScreenContext::ListScreen, message_template :
                MessageTemplate::EmptyPostList, visible : true, outcome : ApiStatus::Ok
            }
        ];
        transition ShowPostCreatedCompletedMessage [tags = ["allow_path", "create_path", "message_path"]]
        when |state| true
        => [MessageContractState
            {
                screen : ScreenContext::CreateForm, message_template :
                MessageTemplate::PostCreatedCompleted, visible : true, outcome :
                ApiStatus::Ok
            }
        ];
        transition ShowInvalidEditKeyMessage [tags = ["deny_path", "edit_path", "message_path", "security_path"]]
        when |state| true
        => [MessageContractState
            {
                screen : ScreenContext::EditForm, message_template :
                MessageTemplate::InvalidEditKey, visible : true, outcome :
                ApiStatus::Forbidden
            }
        ];
    }
    properties {
        invariant P_MESSAGE_EMPTY_LIST_IS_BOUND_TO_LIST_SCREEN |state|
            state.message_template != MessageTemplate::EmptyPostList ||
            (state.visible && state.screen == ScreenContext::ListScreen && state.outcome
            == ApiStatus::Ok);
        invariant P_MESSAGE_POST_CREATED_COMPLETED_IS_BOUND_TO_CREATE_SUCCESS |state|
            state.message_template != MessageTemplate::PostCreatedCompleted ||
            (state.visible && state.screen == ScreenContext::CreateForm && state.outcome
            == ApiStatus::Ok);
        invariant P_MESSAGE_INVALID_EDIT_KEY_IS_BOUND_TO_FORBIDDEN_EDIT |state|
            state.message_template != MessageTemplate::InvalidEditKey ||
            (state.visible && state.screen == ScreenContext::EditForm && state.outcome ==
            ApiStatus::Forbidden);
    }
}
