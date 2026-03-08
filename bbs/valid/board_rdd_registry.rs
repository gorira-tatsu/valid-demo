include!("board_rdd_registry/shared.rs");
include!("board_rdd_registry/common_spec.rs");
include!("board_rdd_registry/post_list.rs");
include!("board_rdd_registry/board_flow.rs");
include!("board_rdd_registry/post_create.rs");
include!("board_rdd_registry/post_detail.rs");
include!("board_rdd_registry/edit_delete.rs");
include!("board_rdd_registry/comment.rs");
include!("board_rdd_registry/list_rendering.rs");
include!("board_rdd_registry/presentation_contract.rs");
include!("board_rdd_registry/submission_discipline.rs");
include!("board_rdd_registry/retry_ux.rs");
include!("board_rdd_registry/message_contract.rs");
include!("board_rdd_registry/api_contract.rs");
include!("board_rdd_registry/edit_key_storage.rs");

fn main() {
    run_registry_cli(valid_models![
        "board-common-spec" => CommonSpecModel,
        "board-post-list" => PostListModel,
        "board-flow" => BoardFlowModel,
        "board-post-create" => PostCreateModel,
        "board-post-detail" => PostDetailModel,
        "board-edit-delete" => EditDeleteModel,
        "board-comment" => CommentModel,
        "board-list-rendering" => ListRenderingModel,
        "board-presentation-contract" => PresentationContractModel,
        "board-submission-discipline" => SubmissionDisciplineModel,
        "board-retry-ux" => RetryUxModel,
        "board-message-contract" => MessageContractModel,
        "board-api-contract" => ApiContractModel,
        "board-edit-key-storage" => EditKeyStorageModel,
    ]);
}
