/*
対応する要件定義:
- docs/rdd/00_前提とスコープ.md
- docs/rdd/02_投稿一覧機能.md

この model が担うこと:
- 削除済みデータを一覧に出さないこと
- 20 件上限、空状態、詳細導線、新規投稿導線
- `page` `limit` の境界条件
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidState)]
struct PostListState {
    #[valid(range = "0..=40")]
    active_posts: u8,
    #[valid(range = "0..=40")]
    deleted_posts: u8,
    #[valid(range = "0..=20")]
    visible_posts: u8,
    #[valid(range = "0..=3")]
    total_pages: u8,
    #[valid(range = "1..=3")]
    current_page: u8,
    #[valid(range = "0..=3")]
    requested_page: u8,
    #[valid(range = "0..=25")]
    requested_limit: u8,
    empty_state_visible: bool,
    has_pagination: bool,
    detail_navigation_available: bool,
    create_navigation_available: bool,
    #[valid(enum)]
    outcome: ApiStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidAction)]
enum PostListAction {
    #[valid(action_id = "SETUP_EMPTY_LIST", reads = [], writes = ["active_posts", "deleted_posts", "visible_posts", "current_page", "requested_page", "requested_limit", "empty_state_visible", "has_pagination", "detail_navigation_available", "create_navigation_available", "outcome"])]
    SetupEmptyList,
    #[valid(action_id = "SETUP_SINGLE_PAGE_LIST", reads = [], writes = ["active_posts", "deleted_posts", "visible_posts", "current_page", "requested_page", "requested_limit", "empty_state_visible", "has_pagination", "detail_navigation_available", "create_navigation_available", "outcome"])]
    SetupSinglePageList,
    #[valid(action_id = "SETUP_MULTI_PAGE_LIST", reads = [], writes = ["active_posts", "deleted_posts", "visible_posts", "current_page", "requested_page", "requested_limit", "empty_state_visible", "has_pagination", "detail_navigation_available", "create_navigation_available", "outcome"])]
    SetupMultiPageList,
    #[valid(action_id = "SETUP_INVALID_PAGE_REQUEST", reads = [], writes = ["active_posts", "deleted_posts", "visible_posts", "current_page", "requested_page", "requested_limit", "empty_state_visible", "has_pagination", "detail_navigation_available", "create_navigation_available", "outcome"])]
    SetupInvalidPageRequest,
    #[valid(action_id = "SETUP_INVALID_LIMIT_REQUEST", reads = [], writes = ["active_posts", "deleted_posts", "visible_posts", "current_page", "requested_page", "requested_limit", "empty_state_visible", "has_pagination", "detail_navigation_available", "create_navigation_available", "outcome"])]
    SetupInvalidLimitRequest,
    #[valid(action_id = "SETUP_PAGE_OVERFLOW_REQUEST", reads = [], writes = ["active_posts", "deleted_posts", "visible_posts", "total_pages", "current_page", "requested_page", "requested_limit", "empty_state_visible", "has_pagination", "detail_navigation_available", "create_navigation_available", "outcome"])]
    SetupPageOverflowRequest,
    #[valid(action_id = "LIST_EMPTY", reads = ["active_posts"], writes = ["visible_posts", "empty_state_visible", "has_pagination", "detail_navigation_available", "create_navigation_available", "outcome"])]
    ListEmpty,
    #[valid(action_id = "LIST_SINGLE_PAGE", reads = ["active_posts"], writes = ["visible_posts", "empty_state_visible", "has_pagination", "detail_navigation_available", "create_navigation_available", "outcome"])]
    ListSinglePage,
    #[valid(action_id = "LIST_MULTI_PAGE", reads = ["active_posts"], writes = ["visible_posts", "empty_state_visible", "has_pagination", "detail_navigation_available", "create_navigation_available", "outcome"])]
    ListMultiPage,
    #[valid(action_id = "LIST_INVALID_PAGE", reads = ["requested_page"], writes = ["outcome"])]
    ListInvalidPage,
    #[valid(action_id = "LIST_INVALID_LIMIT", reads = ["requested_limit"], writes = ["outcome"])]
    ListInvalidLimit,
    #[valid(action_id = "LIST_PAGE_OVERFLOW", reads = ["requested_page", "total_pages"], writes = ["visible_posts", "empty_state_visible", "detail_navigation_available", "create_navigation_available", "outcome"])]
    ListPageOverflow,
}

valid_model! {
    model PostListModel<PostListState, PostListAction>;
    init [PostListState {
        active_posts: 0,
        deleted_posts: 0,
        visible_posts: 0,
        total_pages: 0,
        current_page: 1,
        requested_page: 1,
        requested_limit: 20,
        empty_state_visible: false,
        has_pagination: false,
        detail_navigation_available: false,
        create_navigation_available: false,
        outcome: ApiStatus::Idle,
    }];
    transitions {
        transition SetupEmptyList [tags = ["empty_path", "list_path", "setup_path"]]
        when |state| state.outcome == ApiStatus::Idle
        => [PostListState
            {
                active_posts : 0, deleted_posts : 0, visible_posts : 0, total_pages : 0,
                current_page : 1, requested_page : 1, requested_limit : 20,
                empty_state_visible : false, has_pagination : false,
                detail_navigation_available : false, create_navigation_available : false,
                outcome : ApiStatus::NotFound
            }
        ];
        transition SetupSinglePageList [tags = ["list_path", "setup_path"]]
        when |state| state.outcome == ApiStatus::Idle
        => [PostListState
            {
                active_posts : 7, deleted_posts : 2, visible_posts : 0, total_pages : 1,
                current_page : 1, requested_page : 1, requested_limit : 20,
                empty_state_visible : false, has_pagination : false,
                detail_navigation_available : false, create_navigation_available : false,
                outcome : ApiStatus::NotFound
            }
        ];
        transition SetupMultiPageList [tags = ["list_path", "pagination_path", "setup_path"]]
        when |state| state.outcome == ApiStatus::Idle
        => [PostListState
            {
                active_posts : 25, deleted_posts : 5, visible_posts : 0, total_pages : 2,
                current_page : 1, requested_page : 1, requested_limit : 20,
                empty_state_visible : false, has_pagination : false,
                detail_navigation_available : false, create_navigation_available : false,
                outcome : ApiStatus::NotFound
            }
        ];
        transition SetupInvalidPageRequest [tags = ["boundary_path", "list_path", "setup_path"]]
        when |state| state.outcome == ApiStatus::Idle
        => [PostListState
            {
                active_posts : 7, deleted_posts : 2, visible_posts : 0, total_pages : 1,
                current_page : 1, requested_page : 0, requested_limit : 20,
                empty_state_visible : false, has_pagination : false,
                detail_navigation_available : false, create_navigation_available : false,
                outcome : ApiStatus::NotFound
            }
        ];
        transition SetupInvalidLimitRequest [tags = ["boundary_path", "list_path", "setup_path"]]
        when |state| state.outcome == ApiStatus::Idle
        => [PostListState
            {
                active_posts : 7, deleted_posts : 2, visible_posts : 0, total_pages : 1,
                current_page : 1, requested_page : 1, requested_limit : 0,
                empty_state_visible : false, has_pagination : false,
                detail_navigation_available : false, create_navigation_available : false,
                outcome : ApiStatus::NotFound
            }
        ];
        transition SetupPageOverflowRequest [tags = ["boundary_path", "list_path", "setup_path"]]
        when |state| state.outcome == ApiStatus::Idle
        => [PostListState
            {
                active_posts : 7, deleted_posts : 2, visible_posts : 0, total_pages : 1,
                current_page : 1, requested_page : 2, requested_limit : 20,
                empty_state_visible : false, has_pagination : false,
                detail_navigation_available : false, create_navigation_available : false,
                outcome : ApiStatus::NotFound
            }
        ];
        transition ListEmpty [tags = ["allow_path", "empty_path", "list_path"]]
        when |state| state.active_posts == 0 && state.requested_page == 1
        => [PostListState
            {
                visible_posts : 0, empty_state_visible : true, has_pagination : false,
                detail_navigation_available : false, create_navigation_available : true,
                outcome : ApiStatus::Ok, .. state
            }
        ];
        transition ListSinglePage [tags = ["allow_path", "list_path"]]
        when |state| state.active_posts >= 1 && state.active_posts <= 20 &&
            state.requested_page >= 1 && state.requested_page <= state.total_pages
        => [PostListState
            {
                visible_posts : state.active_posts, empty_state_visible : false,
                has_pagination : false, detail_navigation_available : true,
                create_navigation_available : true, outcome : ApiStatus::Ok, .. state
            }
        ];
        transition ListMultiPage [tags = ["allow_path", "list_path", "pagination_path"]]
        when |state| state.active_posts >= 21 && state.requested_page >= 1 &&
            state.requested_page <= state.total_pages
        => [PostListState
            {
                visible_posts : 20, empty_state_visible : false, has_pagination : true,
                detail_navigation_available : true, create_navigation_available : true,
                outcome : ApiStatus::Ok, .. state
            }
        ];
        transition ListInvalidPage [tags = ["boundary_path", "deny_path", "list_path"]]
        when |state| state.requested_page == 0
        => [PostListState { outcome : ApiStatus::BadRequest, .. state }
        ];
        transition ListInvalidLimit [tags = ["boundary_path", "deny_path", "list_path"]]
        when |state| state.requested_limit == 0 || state.requested_limit >= 21
        => [PostListState { outcome : ApiStatus::BadRequest, .. state }
        ];
        transition ListPageOverflow [tags = ["allow_path", "boundary_path", "list_path"]]
        when |state| state.requested_page >= 1 && state.total_pages >= 1 &&
            state.requested_page > state.total_pages
        => [PostListState
            {
                visible_posts : 0, empty_state_visible : false,
                detail_navigation_available : false, create_navigation_available : true,
                outcome : ApiStatus::Ok, .. state
            }
        ];
    }
    properties {
        invariant P_LIST_PAGE_SIZE_IS_CAPPED_AT_20 |state|
            state.visible_posts <= 20;
        invariant P_LIST_EMPTY_STATE_MATCHES_VISIBLE_COUNT |state|
            state.outcome != ApiStatus::Ok ||
            ((state.empty_state_visible && state.visible_posts == 0) ||
            (state.empty_state_visible == false && state.visible_posts != 0));
        invariant P_LIST_DELETED_POSTS_NEVER_SURPASS_TOTAL |state|
            state.deleted_posts + state.active_posts <= 40;
        invariant P_LIST_SUCCESS_EXPOSES_CREATE_NAVIGATION |state|
            state.outcome != ApiStatus::Ok || state.create_navigation_available;
        invariant P_LIST_SUCCESS_WITH_POSTS_EXPOSES_DETAIL_NAVIGATION |state|
            !(state.outcome == ApiStatus::Ok && state.visible_posts >= 1) ||
            state.detail_navigation_available;
        invariant P_LIST_INVALID_PAGE_REQUEST_RETURNS_BAD_REQUEST |state|
            state.requested_page != 0 || state.outcome == ApiStatus::BadRequest;
        invariant P_LIST_INVALID_LIMIT_REQUEST_RETURNS_BAD_REQUEST |state|
            !((state.requested_limit == 0 || state.requested_limit >= 21)) ||
            state.outcome == ApiStatus::BadRequest;
        invariant P_LIST_PAGE_OVERFLOW_RETURNS_EMPTY_RESULT |state|
            !(state.outcome == ApiStatus::Ok && state.total_pages >= 1 &&
            state.requested_page > state.total_pages) ||
            (state.visible_posts == 0 && state.empty_state_visible == false &&
            state.detail_navigation_available == false);
    }
}
