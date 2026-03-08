/*
対応する要件定義:
- docs/rdd/02_投稿一覧機能.md

この model が担うこと:
- 新しい順 / 古い順の並び順
- 120 文字抜粋と省略記号
- ページネーションまたはもっと見る導線
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidState)]
struct ListRenderingState {
    #[valid(range = "0..=40")]
    active_posts: u8,
    #[valid(range = "0..=20")]
    visible_posts: u8,
    #[valid(enum)]
    sort_order: SortOrder,
    #[valid(range = "0..=10")]
    first_visible_timestamp: u8,
    #[valid(range = "0..=10")]
    second_visible_timestamp: u8,
    #[valid(range = "0..=10")]
    third_visible_timestamp: u8,
    #[valid(range = "0..=10")]
    first_visible_id: u8,
    #[valid(range = "0..=10")]
    second_visible_id: u8,
    #[valid(range = "0..=10")]
    third_visible_id: u8,
    #[valid(range = "0..=121")]
    excerpt_source_len: u8,
    #[valid(range = "0..=120")]
    excerpt_len: u8,
    excerpt_ellipsized: bool,
    #[valid(enum)]
    continuation_ui: ContinuationUi,
    #[valid(enum)]
    outcome: ApiStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidAction)]
enum ListRenderingAction {
    #[valid(action_id = "SETUP_NEWEST_SINGLE_PAGE", reads = [], writes = ["active_posts", "visible_posts", "sort_order", "first_visible_timestamp", "second_visible_timestamp", "third_visible_timestamp", "first_visible_id", "second_visible_id", "third_visible_id", "excerpt_source_len", "excerpt_len", "excerpt_ellipsized", "continuation_ui", "outcome"])]
    SetupNewestSinglePage,
    #[valid(action_id = "SETUP_NEWEST_SINGLE_PAGE_WITH_ELLIPSIS", reads = [], writes = ["active_posts", "visible_posts", "sort_order", "first_visible_timestamp", "second_visible_timestamp", "third_visible_timestamp", "first_visible_id", "second_visible_id", "third_visible_id", "excerpt_source_len", "excerpt_len", "excerpt_ellipsized", "continuation_ui", "outcome"])]
    SetupNewestSinglePageWithEllipsis,
    #[valid(action_id = "SETUP_NEWEST_MULTI_PAGE", reads = [], writes = ["active_posts", "visible_posts", "sort_order", "first_visible_timestamp", "second_visible_timestamp", "third_visible_timestamp", "first_visible_id", "second_visible_id", "third_visible_id", "excerpt_source_len", "excerpt_len", "excerpt_ellipsized", "continuation_ui", "outcome"])]
    SetupNewestMultiPage,
    #[valid(action_id = "SETUP_NEWEST_MULTI_PAGE_WITH_ELLIPSIS", reads = [], writes = ["active_posts", "visible_posts", "sort_order", "first_visible_timestamp", "second_visible_timestamp", "third_visible_timestamp", "first_visible_id", "second_visible_id", "third_visible_id", "excerpt_source_len", "excerpt_len", "excerpt_ellipsized", "continuation_ui", "outcome"])]
    SetupNewestMultiPageWithEllipsis,
    #[valid(action_id = "SETUP_OLDEST_SINGLE_PAGE", reads = [], writes = ["active_posts", "visible_posts", "sort_order", "first_visible_timestamp", "second_visible_timestamp", "third_visible_timestamp", "first_visible_id", "second_visible_id", "third_visible_id", "excerpt_source_len", "excerpt_len", "excerpt_ellipsized", "continuation_ui", "outcome"])]
    SetupOldestSinglePage,
    #[valid(action_id = "SETUP_OLDEST_SINGLE_PAGE_WITH_ELLIPSIS", reads = [], writes = ["active_posts", "visible_posts", "sort_order", "first_visible_timestamp", "second_visible_timestamp", "third_visible_timestamp", "first_visible_id", "second_visible_id", "third_visible_id", "excerpt_source_len", "excerpt_len", "excerpt_ellipsized", "continuation_ui", "outcome"])]
    SetupOldestSinglePageWithEllipsis,
    #[valid(action_id = "SETUP_OLDEST_MULTI_PAGE", reads = [], writes = ["active_posts", "visible_posts", "sort_order", "first_visible_timestamp", "second_visible_timestamp", "third_visible_timestamp", "first_visible_id", "second_visible_id", "third_visible_id", "excerpt_source_len", "excerpt_len", "excerpt_ellipsized", "continuation_ui", "outcome"])]
    SetupOldestMultiPage,
    #[valid(action_id = "SETUP_OLDEST_MULTI_PAGE_WITH_ELLIPSIS", reads = [], writes = ["active_posts", "visible_posts", "sort_order", "first_visible_timestamp", "second_visible_timestamp", "third_visible_timestamp", "first_visible_id", "second_visible_id", "third_visible_id", "excerpt_source_len", "excerpt_len", "excerpt_ellipsized", "continuation_ui", "outcome"])]
    SetupOldestMultiPageWithEllipsis,
    #[valid(action_id = "SETUP_NEWEST_TIE_BREAK", reads = [], writes = ["active_posts", "visible_posts", "sort_order", "first_visible_timestamp", "second_visible_timestamp", "third_visible_timestamp", "first_visible_id", "second_visible_id", "third_visible_id", "excerpt_source_len", "excerpt_len", "excerpt_ellipsized", "continuation_ui", "outcome"])]
    SetupNewestTieBreak,
    #[valid(action_id = "SETUP_OLDEST_TIE_BREAK", reads = [], writes = ["active_posts", "visible_posts", "sort_order", "first_visible_timestamp", "second_visible_timestamp", "third_visible_timestamp", "first_visible_id", "second_visible_id", "third_visible_id", "excerpt_source_len", "excerpt_len", "excerpt_ellipsized", "continuation_ui", "outcome"])]
    SetupOldestTieBreak,
    #[valid(action_id = "RENDER_NEWEST_SINGLE_PAGE", reads = ["active_posts", "first_visible_timestamp", "second_visible_timestamp", "third_visible_timestamp", "excerpt_source_len"], writes = ["visible_posts", "sort_order", "excerpt_len", "excerpt_ellipsized", "continuation_ui", "outcome"])]
    RenderNewestSinglePage,
    #[valid(action_id = "RENDER_NEWEST_SINGLE_PAGE_WITH_ELLIPSIS", reads = ["active_posts", "first_visible_timestamp", "second_visible_timestamp", "third_visible_timestamp", "excerpt_source_len"], writes = ["visible_posts", "sort_order", "excerpt_len", "excerpt_ellipsized", "continuation_ui", "outcome"])]
    RenderNewestSinglePageWithEllipsis,
    #[valid(action_id = "RENDER_NEWEST_MULTI_PAGE_WITH_PAGINATION", reads = ["active_posts", "first_visible_timestamp", "second_visible_timestamp", "third_visible_timestamp", "excerpt_source_len"], writes = ["visible_posts", "sort_order", "excerpt_len", "excerpt_ellipsized", "continuation_ui", "outcome"])]
    RenderNewestMultiPageWithPagination,
    #[valid(action_id = "RENDER_NEWEST_MULTI_PAGE_WITH_PAGINATION_AND_ELLIPSIS", reads = ["active_posts", "first_visible_timestamp", "second_visible_timestamp", "third_visible_timestamp", "excerpt_source_len"], writes = ["visible_posts", "sort_order", "excerpt_len", "excerpt_ellipsized", "continuation_ui", "outcome"])]
    RenderNewestMultiPageWithPaginationAndEllipsis,
    #[valid(action_id = "RENDER_NEWEST_MULTI_PAGE_WITH_LOAD_MORE", reads = ["active_posts", "first_visible_timestamp", "second_visible_timestamp", "third_visible_timestamp", "excerpt_source_len"], writes = ["visible_posts", "sort_order", "excerpt_len", "excerpt_ellipsized", "continuation_ui", "outcome"])]
    RenderNewestMultiPageWithLoadMore,
    #[valid(action_id = "RENDER_NEWEST_MULTI_PAGE_WITH_LOAD_MORE_AND_ELLIPSIS", reads = ["active_posts", "first_visible_timestamp", "second_visible_timestamp", "third_visible_timestamp", "excerpt_source_len"], writes = ["visible_posts", "sort_order", "excerpt_len", "excerpt_ellipsized", "continuation_ui", "outcome"])]
    RenderNewestMultiPageWithLoadMoreAndEllipsis,
    #[valid(action_id = "RENDER_OLDEST_SINGLE_PAGE", reads = ["active_posts", "first_visible_timestamp", "second_visible_timestamp", "third_visible_timestamp", "excerpt_source_len"], writes = ["visible_posts", "sort_order", "excerpt_len", "excerpt_ellipsized", "continuation_ui", "outcome"])]
    RenderOldestSinglePage,
    #[valid(action_id = "RENDER_OLDEST_SINGLE_PAGE_WITH_ELLIPSIS", reads = ["active_posts", "first_visible_timestamp", "second_visible_timestamp", "third_visible_timestamp", "excerpt_source_len"], writes = ["visible_posts", "sort_order", "excerpt_len", "excerpt_ellipsized", "continuation_ui", "outcome"])]
    RenderOldestSinglePageWithEllipsis,
    #[valid(action_id = "RENDER_OLDEST_MULTI_PAGE_WITH_PAGINATION", reads = ["active_posts", "first_visible_timestamp", "second_visible_timestamp", "third_visible_timestamp", "excerpt_source_len"], writes = ["visible_posts", "sort_order", "excerpt_len", "excerpt_ellipsized", "continuation_ui", "outcome"])]
    RenderOldestMultiPageWithPagination,
    #[valid(action_id = "RENDER_OLDEST_MULTI_PAGE_WITH_PAGINATION_AND_ELLIPSIS", reads = ["active_posts", "first_visible_timestamp", "second_visible_timestamp", "third_visible_timestamp", "excerpt_source_len"], writes = ["visible_posts", "sort_order", "excerpt_len", "excerpt_ellipsized", "continuation_ui", "outcome"])]
    RenderOldestMultiPageWithPaginationAndEllipsis,
    #[valid(action_id = "RENDER_OLDEST_MULTI_PAGE_WITH_LOAD_MORE", reads = ["active_posts", "first_visible_timestamp", "second_visible_timestamp", "third_visible_timestamp", "excerpt_source_len"], writes = ["visible_posts", "sort_order", "excerpt_len", "excerpt_ellipsized", "continuation_ui", "outcome"])]
    RenderOldestMultiPageWithLoadMore,
    #[valid(action_id = "RENDER_OLDEST_MULTI_PAGE_WITH_LOAD_MORE_AND_ELLIPSIS", reads = ["active_posts", "first_visible_timestamp", "second_visible_timestamp", "third_visible_timestamp", "excerpt_source_len"], writes = ["visible_posts", "sort_order", "excerpt_len", "excerpt_ellipsized", "continuation_ui", "outcome"])]
    RenderOldestMultiPageWithLoadMoreAndEllipsis,
}

valid_model! {
    model ListRenderingModel<ListRenderingState, ListRenderingAction>;
    init [ListRenderingState {
        active_posts: 0,
        visible_posts: 0,
        sort_order: SortOrder::NewestFirst,
        first_visible_timestamp: 0,
        second_visible_timestamp: 0,
        third_visible_timestamp: 0,
        first_visible_id: 0,
        second_visible_id: 0,
        third_visible_id: 0,
        excerpt_source_len: 0,
        excerpt_len: 0,
        excerpt_ellipsized: false,
        continuation_ui: ContinuationUi::None,
        outcome: ApiStatus::Idle,
    }];
    transitions {
        transition SetupNewestSinglePage [tags = ["list_path", "setup_path", "sort_path"]]
        when |state| state.active_posts == 0
        => [ListRenderingState
            {
                active_posts : 3, visible_posts : 0, sort_order : SortOrder::NewestFirst,
                first_visible_timestamp : 9, second_visible_timestamp : 6,
                third_visible_timestamp : 2, first_visible_id : 9,
                second_visible_id : 6, third_visible_id : 2, excerpt_source_len : 80,
                excerpt_len : 80, excerpt_ellipsized : false,
                continuation_ui : ContinuationUi::None, outcome : ApiStatus::Idle
            }
        ];
        transition SetupNewestSinglePageWithEllipsis [tags = ["list_path", "setup_path", "sort_path"]]
        when |state| state.active_posts == 0
        => [ListRenderingState
            {
                active_posts : 3, visible_posts : 0, sort_order : SortOrder::NewestFirst,
                first_visible_timestamp : 9, second_visible_timestamp : 6,
                third_visible_timestamp : 2, first_visible_id : 9,
                second_visible_id : 6, third_visible_id : 2, excerpt_source_len : 121,
                excerpt_len : 120, excerpt_ellipsized : true,
                continuation_ui : ContinuationUi::None, outcome : ApiStatus::Idle
            }
        ];
        transition SetupNewestMultiPage [tags = ["list_path", "pagination_path", "setup_path", "sort_path"]]
        when |state| state.active_posts == 0
        => [ListRenderingState
            {
                active_posts : 25, visible_posts : 0, sort_order : SortOrder::NewestFirst,
                first_visible_timestamp : 9, second_visible_timestamp : 6,
                third_visible_timestamp : 2, first_visible_id : 9,
                second_visible_id : 6, third_visible_id : 2, excerpt_source_len : 80,
                excerpt_len : 80, excerpt_ellipsized : false,
                continuation_ui : ContinuationUi::Pagination, outcome : ApiStatus::Idle
            }
        ];
        transition SetupNewestMultiPageWithEllipsis [tags = ["list_path", "pagination_path", "setup_path", "sort_path"]]
        when |state| state.active_posts == 0
        => [ListRenderingState
            {
                active_posts : 25, visible_posts : 0, sort_order : SortOrder::NewestFirst,
                first_visible_timestamp : 9, second_visible_timestamp : 6,
                third_visible_timestamp : 2, first_visible_id : 9,
                second_visible_id : 6, third_visible_id : 2, excerpt_source_len : 121,
                excerpt_len : 120, excerpt_ellipsized : true,
                continuation_ui : ContinuationUi::Pagination, outcome : ApiStatus::Idle
            }
        ];
        transition SetupOldestSinglePage [tags = ["list_path", "setup_path", "sort_path"]]
        when |state| state.active_posts == 0
        => [ListRenderingState
            {
                active_posts : 3, visible_posts : 0, sort_order : SortOrder::OldestFirst,
                first_visible_timestamp : 2, second_visible_timestamp : 6,
                third_visible_timestamp : 9, first_visible_id : 2,
                second_visible_id : 6, third_visible_id : 9, excerpt_source_len : 80,
                excerpt_len : 80, excerpt_ellipsized : false,
                continuation_ui : ContinuationUi::None, outcome : ApiStatus::Idle
            }
        ];
        transition SetupOldestSinglePageWithEllipsis [tags = ["list_path", "setup_path", "sort_path"]]
        when |state| state.active_posts == 0
        => [ListRenderingState
            {
                active_posts : 3, visible_posts : 0, sort_order : SortOrder::OldestFirst,
                first_visible_timestamp : 2, second_visible_timestamp : 6,
                third_visible_timestamp : 9, first_visible_id : 2,
                second_visible_id : 6, third_visible_id : 9, excerpt_source_len : 121,
                excerpt_len : 120, excerpt_ellipsized : true,
                continuation_ui : ContinuationUi::None, outcome : ApiStatus::Idle
            }
        ];
        transition SetupOldestMultiPage [tags = ["list_path", "pagination_path", "setup_path", "sort_path"]]
        when |state| state.active_posts == 0
        => [ListRenderingState
            {
                active_posts : 25, visible_posts : 0, sort_order : SortOrder::OldestFirst,
                first_visible_timestamp : 2, second_visible_timestamp : 6,
                third_visible_timestamp : 9, first_visible_id : 2,
                second_visible_id : 6, third_visible_id : 9, excerpt_source_len : 80,
                excerpt_len : 80, excerpt_ellipsized : false,
                continuation_ui : ContinuationUi::Pagination, outcome : ApiStatus::Idle
            }
        ];
        transition SetupOldestMultiPageWithEllipsis [tags = ["list_path", "pagination_path", "setup_path", "sort_path"]]
        when |state| state.active_posts == 0
        => [ListRenderingState
            {
                active_posts : 25, visible_posts : 0, sort_order : SortOrder::OldestFirst,
                first_visible_timestamp : 2, second_visible_timestamp : 6,
                third_visible_timestamp : 9, first_visible_id : 2,
                second_visible_id : 6, third_visible_id : 9, excerpt_source_len : 121,
                excerpt_len : 120, excerpt_ellipsized : true,
                continuation_ui : ContinuationUi::Pagination, outcome : ApiStatus::Idle
            }
        ];
        transition SetupNewestTieBreak [tags = ["list_path", "setup_path", "sort_path"]]
        when |state| state.active_posts == 0
        => [ListRenderingState
            {
                active_posts : 3, visible_posts : 0, sort_order : SortOrder::NewestFirst,
                first_visible_timestamp : 9, second_visible_timestamp : 9,
                third_visible_timestamp : 2, first_visible_id : 9,
                second_visible_id : 4, third_visible_id : 2, excerpt_source_len : 80,
                excerpt_len : 80, excerpt_ellipsized : false,
                continuation_ui : ContinuationUi::None, outcome : ApiStatus::Idle
            }
        ];
        transition SetupOldestTieBreak [tags = ["list_path", "setup_path", "sort_path"]]
        when |state| state.active_posts == 0
        => [ListRenderingState
            {
                active_posts : 3, visible_posts : 0, sort_order : SortOrder::OldestFirst,
                first_visible_timestamp : 2, second_visible_timestamp : 2,
                third_visible_timestamp : 9, first_visible_id : 2,
                second_visible_id : 7, third_visible_id : 9, excerpt_source_len : 80,
                excerpt_len : 80, excerpt_ellipsized : false,
                continuation_ui : ContinuationUi::None, outcome : ApiStatus::Idle
            }
        ];
        transition RenderNewestSinglePage [tags = ["allow_path", "list_path", "render_path", "sort_path"]]
        when |state| state.sort_order == SortOrder::NewestFirst && state.active_posts >= 1 && state.active_posts <= 20 &&
            state.first_visible_timestamp >= state.second_visible_timestamp &&
            state.excerpt_source_len <= 120
        => [ListRenderingState
            {
                visible_posts : state.active_posts, sort_order : SortOrder::NewestFirst,
                excerpt_len : state.excerpt_source_len, excerpt_ellipsized : false,
                continuation_ui : ContinuationUi::None, outcome : ApiStatus::Ok, .. state
            }
        ];
        transition RenderNewestSinglePageWithEllipsis [tags = ["allow_path", "list_path", "render_path", "sort_path"]]
        when |state| state.sort_order == SortOrder::NewestFirst && state.active_posts >= 1 && state.active_posts <= 20 &&
            state.first_visible_timestamp >= state.second_visible_timestamp &&
            state.excerpt_source_len == 121
        => [ListRenderingState
            {
                visible_posts : state.active_posts, sort_order : SortOrder::NewestFirst,
                excerpt_len : 120, excerpt_ellipsized : true, continuation_ui :
                ContinuationUi::None, outcome : ApiStatus::Ok, .. state
            }
        ];
        transition RenderNewestMultiPageWithPagination [tags = ["allow_path", "list_path", "pagination_path", "render_path", "sort_path"]]
        when |state| state.sort_order == SortOrder::NewestFirst && state.active_posts >= 21 && state.first_visible_timestamp >=
            state.second_visible_timestamp && state.excerpt_source_len <= 120
        => [ListRenderingState
            {
                visible_posts : 20, sort_order : SortOrder::NewestFirst, excerpt_len :
                state.excerpt_source_len, excerpt_ellipsized : false, continuation_ui :
                ContinuationUi::Pagination, outcome : ApiStatus::Ok, .. state
            }
        ];
        transition RenderNewestMultiPageWithPaginationAndEllipsis [tags = ["allow_path", "list_path", "pagination_path", "render_path", "sort_path"]]
        when |state| state.sort_order == SortOrder::NewestFirst && state.active_posts >= 21 && state.first_visible_timestamp >=
            state.second_visible_timestamp && state.excerpt_source_len == 121
        => [ListRenderingState
            {
                visible_posts : 20, sort_order : SortOrder::NewestFirst, excerpt_len :
                120, excerpt_ellipsized : true, continuation_ui :
                ContinuationUi::Pagination, outcome : ApiStatus::Ok, .. state
            }
        ];
        transition RenderNewestMultiPageWithLoadMore [tags = ["allow_path", "list_path", "pagination_path", "render_path", "sort_path"]]
        when |state| state.sort_order == SortOrder::NewestFirst && state.active_posts >= 21 && state.first_visible_timestamp >=
            state.second_visible_timestamp && state.excerpt_source_len <= 120
        => [ListRenderingState
            {
                visible_posts : 20, sort_order : SortOrder::NewestFirst, excerpt_len :
                state.excerpt_source_len, excerpt_ellipsized : false, continuation_ui :
                ContinuationUi::LoadMore, outcome : ApiStatus::Ok, .. state
            }
        ];
        transition RenderNewestMultiPageWithLoadMoreAndEllipsis [tags = ["allow_path", "list_path", "pagination_path", "render_path", "sort_path"]]
        when |state| state.sort_order == SortOrder::NewestFirst && state.active_posts >= 21 && state.first_visible_timestamp >=
            state.second_visible_timestamp && state.excerpt_source_len == 121
        => [ListRenderingState
            {
                visible_posts : 20, sort_order : SortOrder::NewestFirst, excerpt_len :
                120, excerpt_ellipsized : true, continuation_ui :
                ContinuationUi::LoadMore, outcome : ApiStatus::Ok, .. state
            }
        ];
        transition RenderOldestSinglePage [tags = ["allow_path", "list_path", "render_path", "sort_path"]]
        when |state| state.sort_order == SortOrder::OldestFirst && state.active_posts >= 1 && state.active_posts <= 20 &&
            state.first_visible_timestamp <= state.second_visible_timestamp &&
            state.excerpt_source_len <= 120
        => [ListRenderingState
            {
                visible_posts : state.active_posts, sort_order : SortOrder::OldestFirst,
                excerpt_len : state.excerpt_source_len, excerpt_ellipsized : false,
                continuation_ui : ContinuationUi::None, outcome : ApiStatus::Ok, .. state
            }
        ];
        transition RenderOldestSinglePageWithEllipsis [tags = ["allow_path", "list_path", "render_path", "sort_path"]]
        when |state| state.sort_order == SortOrder::OldestFirst && state.active_posts >= 1 && state.active_posts <= 20 &&
            state.first_visible_timestamp <= state.second_visible_timestamp &&
            state.excerpt_source_len == 121
        => [ListRenderingState
            {
                visible_posts : state.active_posts, sort_order : SortOrder::OldestFirst,
                excerpt_len : 120, excerpt_ellipsized : true, continuation_ui :
                ContinuationUi::None, outcome : ApiStatus::Ok, .. state
            }
        ];
        transition RenderOldestMultiPageWithPagination [tags = ["allow_path", "list_path", "pagination_path", "render_path", "sort_path"]]
        when |state| state.sort_order == SortOrder::OldestFirst && state.active_posts >= 21 && state.first_visible_timestamp <=
            state.second_visible_timestamp && state.excerpt_source_len <= 120
        => [ListRenderingState
            {
                visible_posts : 20, sort_order : SortOrder::OldestFirst, excerpt_len :
                state.excerpt_source_len, excerpt_ellipsized : false, continuation_ui :
                ContinuationUi::Pagination, outcome : ApiStatus::Ok, .. state
            }
        ];
        transition RenderOldestMultiPageWithPaginationAndEllipsis [tags = ["allow_path", "list_path", "pagination_path", "render_path", "sort_path"]]
        when |state| state.sort_order == SortOrder::OldestFirst && state.active_posts >= 21 && state.first_visible_timestamp <=
            state.second_visible_timestamp && state.excerpt_source_len == 121
        => [ListRenderingState
            {
                visible_posts : 20, sort_order : SortOrder::OldestFirst, excerpt_len :
                120, excerpt_ellipsized : true, continuation_ui :
                ContinuationUi::Pagination, outcome : ApiStatus::Ok, .. state
            }
        ];
        transition RenderOldestMultiPageWithLoadMore [tags = ["allow_path", "list_path", "pagination_path", "render_path", "sort_path"]]
        when |state| state.sort_order == SortOrder::OldestFirst && state.active_posts >= 21 && state.first_visible_timestamp <=
            state.second_visible_timestamp && state.excerpt_source_len <= 120
        => [ListRenderingState
            {
                visible_posts : 20, sort_order : SortOrder::OldestFirst, excerpt_len :
                state.excerpt_source_len, excerpt_ellipsized : false, continuation_ui :
                ContinuationUi::LoadMore, outcome : ApiStatus::Ok, .. state
            }
        ];
        transition RenderOldestMultiPageWithLoadMoreAndEllipsis [tags = ["allow_path", "list_path", "pagination_path", "render_path", "sort_path"]]
        when |state| state.sort_order == SortOrder::OldestFirst && state.active_posts >= 21 && state.first_visible_timestamp <=
            state.second_visible_timestamp && state.excerpt_source_len == 121
        => [ListRenderingState
            {
                visible_posts : 20, sort_order : SortOrder::OldestFirst, excerpt_len :
                120, excerpt_ellipsized : true, continuation_ui :
                ContinuationUi::LoadMore, outcome : ApiStatus::Ok, .. state
            }
        ];
    }
    properties {
        invariant P_LIST_NEWEST_ORDER_IS_TIMESTAMP_DESCENDING |state|
            !(state.outcome == ApiStatus::Ok && state.visible_posts >= 2 &&
            state.sort_order == SortOrder::NewestFirst) || state.first_visible_timestamp
            >= state.second_visible_timestamp;
        invariant P_LIST_OLDEST_ORDER_IS_TIMESTAMP_ASCENDING |state|
            !(state.outcome == ApiStatus::Ok && state.visible_posts >= 2 &&
            state.sort_order == SortOrder::OldestFirst) || state.first_visible_timestamp
            <= state.second_visible_timestamp;
        invariant P_LIST_NEWEST_ORDER_IS_MONOTONIC_ACROSS_TOP3 |state|
            !(state.outcome == ApiStatus::Ok && state.visible_posts >= 3 &&
            state.sort_order == SortOrder::NewestFirst) ||
            (state.first_visible_timestamp >= state.second_visible_timestamp &&
            state.second_visible_timestamp >= state.third_visible_timestamp);
        invariant P_LIST_OLDEST_ORDER_IS_MONOTONIC_ACROSS_TOP3 |state|
            !(state.outcome == ApiStatus::Ok && state.visible_posts >= 3 &&
            state.sort_order == SortOrder::OldestFirst) ||
            (state.first_visible_timestamp <= state.second_visible_timestamp &&
            state.second_visible_timestamp <= state.third_visible_timestamp);
        invariant P_LIST_NEWEST_TIE_BREAK_IS_STABLE |state|
            !(state.outcome == ApiStatus::Ok && state.visible_posts >= 2 &&
            state.sort_order == SortOrder::NewestFirst && state.first_visible_timestamp ==
            state.second_visible_timestamp) || state.first_visible_id >=
            state.second_visible_id;
        invariant P_LIST_OLDEST_TIE_BREAK_IS_STABLE |state|
            !(state.outcome == ApiStatus::Ok && state.visible_posts >= 2 &&
            state.sort_order == SortOrder::OldestFirst && state.first_visible_timestamp ==
            state.second_visible_timestamp) || state.first_visible_id <=
            state.second_visible_id;
        invariant P_LIST_EXCERPT_WITHIN_LIMIT_IS_NOT_ELLIPSIZED |state|
            state.excerpt_source_len == 121 ||
            (state.excerpt_len == state.excerpt_source_len && state.excerpt_ellipsized ==
            false);
        invariant P_LIST_EXCERPT_OVER_LIMIT_IS_120_AND_ELLIPSIZED |state|
            state.excerpt_source_len <= 120 ||
            (state.excerpt_len == 120 && state.excerpt_ellipsized);
        invariant P_LIST_CONTINUATION_UI_IS_NONE_WITHIN_SINGLE_PAGE |state|
            state.active_posts >= 21 || state.continuation_ui == ContinuationUi::None;
        invariant P_LIST_CONTINUATION_UI_IS_EXPLICIT_FOR_MULTI_PAGE |state|
            state.active_posts <= 20 || state.continuation_ui != ContinuationUi::None;
    }
}
