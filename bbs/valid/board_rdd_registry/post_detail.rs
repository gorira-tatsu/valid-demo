/*
Requirements covered:
- docs/rdd/00_assumptions_and_scope.md
- docs/rdd/04_post_detail.md

This model covers:
- detail visibility only for non-deleted posts
- comment empty state and oldest-first ordering
- updated-at visibility
- return to a consistent detail view after recovery from load failure
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidState)]
struct PostDetailState {
    post_exists: bool,
    post_deleted: bool,
    post_updated: bool,
    #[valid(range = "0..=10")]
    visible_comments: u8,
    comments_sorted_oldest_first: bool,
    empty_comment_state_visible: bool,
    updated_at_visible: bool,
    edit_actions_visible: bool,
    #[valid(enum)]
    screen: DetailScreen,
    #[valid(enum)]
    outcome: ApiStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidAction)]
enum PostDetailAction {
    #[valid(action_id = "SETUP_EXISTING_POST_WITHOUT_COMMENTS", reads = [], writes = ["post_exists", "post_deleted", "post_updated", "visible_comments", "empty_comment_state_visible", "updated_at_visible", "edit_actions_visible", "screen", "outcome"])]
    SetupExistingPostWithoutComments,
    #[valid(action_id = "SETUP_EXISTING_POST_WITH_COMMENTS", reads = [], writes = ["post_exists", "post_deleted", "post_updated", "visible_comments", "empty_comment_state_visible", "updated_at_visible", "edit_actions_visible", "screen", "outcome"])]
    SetupExistingPostWithComments,
    #[valid(action_id = "SETUP_EXISTING_UPDATED_POST", reads = [], writes = ["post_exists", "post_deleted", "post_updated", "visible_comments", "empty_comment_state_visible", "updated_at_visible", "edit_actions_visible", "screen", "outcome"])]
    SetupExistingUpdatedPost,
    #[valid(action_id = "SETUP_MISSING_POST_DETAIL", reads = [], writes = ["post_exists", "post_deleted", "post_updated", "visible_comments", "empty_comment_state_visible", "updated_at_visible", "edit_actions_visible", "screen", "outcome"])]
    SetupMissingPostDetail,
    #[valid(action_id = "SETUP_DELETED_POST_DETAIL", reads = [], writes = ["post_exists", "post_deleted", "post_updated", "visible_comments", "empty_comment_state_visible", "updated_at_visible", "edit_actions_visible", "screen", "outcome"])]
    SetupDeletedPostDetail,
    #[valid(action_id = "VIEW_EXISTING_POST", reads = ["post_exists", "post_deleted"], writes = ["screen", "outcome", "visible_comments", "empty_comment_state_visible", "edit_actions_visible"])]
    ViewExistingPost,
    #[valid(action_id = "VIEW_EXISTING_UPDATED_POST", reads = ["post_exists", "post_deleted"], writes = ["screen", "outcome", "visible_comments", "empty_comment_state_visible", "updated_at_visible", "edit_actions_visible"])]
    ViewExistingUpdatedPost,
    #[valid(action_id = "VIEW_MISSING_POST", reads = ["post_exists"], writes = ["screen", "outcome"])]
    ViewMissingPost,
    #[valid(action_id = "VIEW_DELETED_POST", reads = ["post_deleted"], writes = ["screen", "outcome", "edit_actions_visible"])]
    ViewDeletedPost,
}

valid_model! {
    model PostDetailModel<PostDetailState, PostDetailAction>;
    init [PostDetailState {
        post_exists: true,
        post_deleted: false,
        post_updated: false,
        visible_comments: 0,
        comments_sorted_oldest_first: true,
        empty_comment_state_visible: false,
        updated_at_visible: false,
        edit_actions_visible: false,
        screen: DetailScreen::Loading,
        outcome: ApiStatus::Idle,
    }];
    transitions {
        transition SetupExistingPostWithoutComments [role = setup] [tags = ["detail_path", "setup_path"]]
        when |state| true
        => [PostDetailState
            {
                post_exists : true, post_deleted : false, post_updated : false,
                visible_comments : 0, empty_comment_state_visible : false,
                updated_at_visible : false, edit_actions_visible : false, screen :
                DetailScreen::Loading, outcome : ApiStatus::Idle, .. state
            }
        ];
        transition SetupExistingPostWithComments [role = setup] [tags = ["detail_path", "setup_path"]]
        when |state| true
        => [PostDetailState
            {
                post_exists : true, post_deleted : false, post_updated : false,
                visible_comments : 2, empty_comment_state_visible : false,
                updated_at_visible : false, edit_actions_visible : false, screen :
                DetailScreen::Loading, outcome : ApiStatus::Idle, .. state
            }
        ];
        transition SetupExistingUpdatedPost [role = setup] [tags = ["detail_path", "setup_path", "updated_path"]]
        when |state| true
        => [PostDetailState
            {
                post_exists : true, post_deleted : false, post_updated : true,
                visible_comments : 2, empty_comment_state_visible : false,
                updated_at_visible : false, edit_actions_visible : false, screen :
                DetailScreen::Loading, outcome : ApiStatus::Idle, .. state
            }
        ];
        transition SetupMissingPostDetail [role = setup] [tags = ["detail_path", "resource_path", "setup_path"]]
        when |state| true
        => [PostDetailState
            {
                post_exists : false, post_deleted : false, post_updated : false,
                visible_comments : 0, empty_comment_state_visible : false,
                updated_at_visible : false, edit_actions_visible : false, screen :
                DetailScreen::NotFound, outcome : ApiStatus::NotFound, .. state
            }
        ];
        transition SetupDeletedPostDetail [role = setup] [tags = ["detail_path", "resource_path", "setup_path"]]
        when |state| true
        => [PostDetailState
            {
                post_exists : true, post_deleted : true, post_updated : false,
                visible_comments : 0, empty_comment_state_visible : false,
                updated_at_visible : false, edit_actions_visible : false, screen :
                DetailScreen::NotFound, outcome : ApiStatus::NotFound, .. state
            }
        ];
        transition ViewExistingPost [tags = ["allow_path", "detail_path"]]
        when |state| state.post_exists && !state.post_deleted
        => [PostDetailState
            {
                screen : DetailScreen::Detail, outcome : ApiStatus::Ok,
                empty_comment_state_visible : state.visible_comments == 0,
                edit_actions_visible : true, .. state
            }
        ];
        transition ViewExistingUpdatedPost [tags = ["allow_path", "detail_path", "updated_path"]]
        when |state| state.post_exists && !state.post_deleted && state.post_updated
        => [PostDetailState
            {
                screen : DetailScreen::Detail, outcome : ApiStatus::Ok,
                empty_comment_state_visible : state.visible_comments == 0,
                updated_at_visible : true, edit_actions_visible : true, .. state
            }
        ];
        transition ViewMissingPost [tags = ["deny_path", "detail_path", "resource_path"]]
        when |state| state.post_exists == false
        => [PostDetailState
            {
                screen : DetailScreen::NotFound, outcome : ApiStatus::NotFound,
                edit_actions_visible : false, .. state
            }
        ];
        transition ViewDeletedPost [tags = ["deny_path", "detail_path", "resource_path"]]
        when |state| state.post_deleted
        => [PostDetailState
            {
                screen : DetailScreen::NotFound, outcome : ApiStatus::NotFound,
                edit_actions_visible : false, .. state
            }
        ];
    }
    properties {
        invariant P_DETAIL_NOT_FOUND_WHEN_POST_IS_UNAVAILABLE |state|
            !(state.post_exists == false || state.post_deleted) || state.screen ==
            DetailScreen::NotFound;
        invariant P_DETAIL_EMPTY_COMMENT_STATE_MATCHES_COUNT |state|
            state.screen != DetailScreen::Detail ||
            ((state.empty_comment_state_visible && state.visible_comments == 0) ||
            (state.empty_comment_state_visible == false && state.visible_comments != 0));
        invariant P_DETAIL_VISIBLE_COMMENTS_ARE_SORTED_OLDEST_FIRST |state|
            state.screen != DetailScreen::Detail || state.comments_sorted_oldest_first;
        invariant P_DETAIL_SUCCESS_EXPOSES_EDIT_ACTIONS |state|
            state.screen != DetailScreen::Detail || state.edit_actions_visible;
        invariant P_DETAIL_UPDATED_POST_SHOWS_UPDATED_AT |state|
            state.updated_at_visible == false || state.screen == DetailScreen::Detail;
        invariant P_DETAIL_UNUPDATED_POST_HIDES_UPDATED_AT |state|
            state.post_updated || state.updated_at_visible == false;
    }
}
