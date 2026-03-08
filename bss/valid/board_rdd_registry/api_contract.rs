/*
対応する要件定義:
- docs/rdd/01_共通仕様.md

この model が担うこと:
- `GET /posts` と `POST /posts` の JSON 応答構造
- API 応答が JSON であること
- 正常系の基本フィールド契約
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidState)]
struct ApiContractState {
    #[valid(enum)]
    endpoint: ApiEndpoint,
    returns_json: bool,
    has_message_field: bool,
    has_field_errors_field: bool,
    has_items_field: bool,
    has_page_field: bool,
    has_limit_field: bool,
    has_total_count_field: bool,
    has_id_field: bool,
    has_title_field: bool,
    has_body_field: bool,
    has_author_name_field: bool,
    has_created_at_field: bool,
    #[valid(enum)]
    outcome: ApiStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidAction)]
enum ApiContractAction {
    #[valid(action_id = "RETURN_LIST_POSTS_JSON", reads = [], writes = ["endpoint", "returns_json", "has_message_field", "has_field_errors_field", "has_items_field", "has_page_field", "has_limit_field", "has_total_count_field", "has_id_field", "has_title_field", "has_body_field", "has_author_name_field", "has_created_at_field", "outcome"])]
    ReturnListPostsJson,
    #[valid(action_id = "RETURN_CREATE_POST_JSON", reads = [], writes = ["endpoint", "returns_json", "has_message_field", "has_field_errors_field", "has_items_field", "has_page_field", "has_limit_field", "has_total_count_field", "has_id_field", "has_title_field", "has_body_field", "has_author_name_field", "has_created_at_field", "outcome"])]
    ReturnCreatePostJson,
    #[valid(action_id = "RETURN_LIST_POSTS_BAD_REQUEST_JSON", reads = [], writes = ["endpoint", "returns_json", "has_message_field", "has_field_errors_field", "has_items_field", "has_page_field", "has_limit_field", "has_total_count_field", "has_id_field", "has_title_field", "has_body_field", "has_author_name_field", "has_created_at_field", "outcome"])]
    ReturnListPostsBadRequestJson,
    #[valid(action_id = "RETURN_CREATE_POST_BAD_REQUEST_JSON", reads = [], writes = ["endpoint", "returns_json", "has_message_field", "has_field_errors_field", "has_items_field", "has_page_field", "has_limit_field", "has_total_count_field", "has_id_field", "has_title_field", "has_body_field", "has_author_name_field", "has_created_at_field", "outcome"])]
    ReturnCreatePostBadRequestJson,
    #[valid(action_id = "RETURN_CREATE_POST_SERVER_ERROR_JSON", reads = [], writes = ["endpoint", "returns_json", "has_message_field", "has_field_errors_field", "has_items_field", "has_page_field", "has_limit_field", "has_total_count_field", "has_id_field", "has_title_field", "has_body_field", "has_author_name_field", "has_created_at_field", "outcome"])]
    ReturnCreatePostServerErrorJson,
}

valid_model! {
    model ApiContractModel<ApiContractState, ApiContractAction>;
    init [ApiContractState {
        endpoint: ApiEndpoint::ListPosts,
        returns_json: false,
        has_message_field: false,
        has_field_errors_field: false,
        has_items_field: false,
        has_page_field: false,
        has_limit_field: false,
        has_total_count_field: false,
        has_id_field: false,
        has_title_field: false,
        has_body_field: false,
        has_author_name_field: false,
        has_created_at_field: false,
        outcome: ApiStatus::Idle,
    }];
    transitions {
        transition ReturnListPostsJson [tags = ["allow_path", "api_path", "list_path"]]
        when |state| true
        => [ApiContractState
            {
                endpoint : ApiEndpoint::ListPosts, returns_json : true,
                has_message_field : false, has_field_errors_field : false,
                has_items_field : true, has_page_field : true, has_limit_field : true,
                has_total_count_field : true, has_id_field : false,
                has_title_field : false, has_body_field : false,
                has_author_name_field : false, has_created_at_field : false,
                outcome : ApiStatus::Ok
            }
        ];
        transition ReturnCreatePostJson [tags = ["allow_path", "api_path", "create_path"]]
        when |state| true
        => [ApiContractState
            {
                endpoint : ApiEndpoint::CreatePost, returns_json : true,
                has_message_field : false, has_field_errors_field : false,
                has_items_field : false, has_page_field : false,
                has_limit_field : false, has_total_count_field : false,
                has_id_field : true, has_title_field : true, has_body_field : true,
                has_author_name_field : true, has_created_at_field : true,
                outcome : ApiStatus::Ok
            }
        ];
        transition ReturnListPostsBadRequestJson [tags = ["api_path", "deny_path", "list_path"]]
        when |state| true
        => [ApiContractState
            {
                endpoint : ApiEndpoint::ListPosts, returns_json : true,
                has_message_field : true, has_field_errors_field : true,
                has_items_field : false, has_page_field : false,
                has_limit_field : false, has_total_count_field : false,
                has_id_field : false, has_title_field : false, has_body_field : false,
                has_author_name_field : false, has_created_at_field : false,
                outcome : ApiStatus::BadRequest
            }
        ];
        transition ReturnCreatePostBadRequestJson [tags = ["api_path", "create_path", "deny_path"]]
        when |state| true
        => [ApiContractState
            {
                endpoint : ApiEndpoint::CreatePost, returns_json : true,
                has_message_field : true, has_field_errors_field : true,
                has_items_field : false, has_page_field : false,
                has_limit_field : false, has_total_count_field : false,
                has_id_field : false, has_title_field : false, has_body_field : false,
                has_author_name_field : false, has_created_at_field : false,
                outcome : ApiStatus::BadRequest
            }
        ];
        transition ReturnCreatePostServerErrorJson [tags = ["api_path", "create_path", "exception_path"]]
        when |state| true
        => [ApiContractState
            {
                endpoint : ApiEndpoint::CreatePost, returns_json : true,
                has_message_field : true, has_field_errors_field : false,
                has_items_field : false, has_page_field : false,
                has_limit_field : false, has_total_count_field : false,
                has_id_field : false, has_title_field : false, has_body_field : false,
                has_author_name_field : false, has_created_at_field : false,
                outcome : ApiStatus::ServerError
            }
        ];
    }
    properties {
        invariant P_API_RESPONSES_ARE_JSON |state|
            state.outcome == ApiStatus::Idle || state.returns_json;
        invariant P_API_LIST_POSTS_RESPONSE_FIELDS_MATCH_CONTRACT |state|
            !(state.outcome == ApiStatus::Ok && state.endpoint == ApiEndpoint::ListPosts)
            ||
            (state.has_items_field && state.has_page_field && state.has_limit_field &&
            state.has_total_count_field && state.has_id_field == false &&
            state.has_title_field == false && state.has_body_field == false &&
            state.has_author_name_field == false && state.has_created_at_field == false);
        invariant P_API_CREATE_POST_RESPONSE_FIELDS_MATCH_CONTRACT |state|
            !(state.outcome == ApiStatus::Ok && state.endpoint == ApiEndpoint::CreatePost)
            ||
            (state.has_items_field == false && state.has_page_field == false &&
            state.has_limit_field == false && state.has_total_count_field == false &&
            state.has_id_field && state.has_title_field && state.has_body_field &&
            state.has_author_name_field && state.has_created_at_field);
        invariant P_API_ERROR_RESPONSES_EXPOSE_MESSAGE |state|
            state.outcome == ApiStatus::Ok || state.outcome == ApiStatus::Idle ||
            state.has_message_field;
        invariant P_API_BAD_REQUEST_RESPONSES_EXPOSE_FIELD_ERRORS |state|
            state.outcome != ApiStatus::BadRequest || state.has_field_errors_field;
    }
}
