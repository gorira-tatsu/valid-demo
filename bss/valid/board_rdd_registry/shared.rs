use valid::{registry::run_registry_cli, valid_model, valid_models, ValidAction, ValidEnum, ValidState};

/*
docs/rdd を valid に落とすときの読み方

- 仕様文:
  人間が読む業務ルール。長さ制限、404、削除済み非表示など。
- valid state:
  仕様のうち「検証したい事実」を有限状態へ圧縮したもの。
- valid action:
  画面操作や API 呼び出しを 1 ステップの遷移として表したもの。
- valid property:
  仕様として常に守られてほしい不変条件。

つまりこのファイル群は、仕様書をそのまま実装しているのではなく、
仕様書の中の「壊してはいけないルール」だけを有限状態機械として
抽出したもの。
*/

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidEnum)]
enum ApiStatus {
    Idle,
    Ok,
    BadRequest,
    Forbidden,
    NotFound,
    ServerError,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidEnum)]
enum CreatePhase {
    Editing,
    Submitting,
    Success,
    Failure,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidEnum)]
enum DetailScreen {
    Loading,
    Detail,
    NotFound,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidEnum)]
enum SortOrder {
    NewestFirst,
    OldestFirst,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidEnum)]
enum ContinuationUi {
    None,
    Pagination,
    LoadMore,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidEnum)]
enum RetryMessagePlacement {
    None,
    TopBanner,
    BelowForm,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidEnum)]
enum SuccessMessageKind {
    None,
    PostCreatedCompleted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidEnum)]
enum ApiEndpoint {
    ListPosts,
    CreatePost,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidEnum)]
enum ScreenContext {
    ListScreen,
    CreateForm,
    EditForm,
    CommentForm,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidEnum)]
enum RetryPhase {
    Idle,
    ErrorShown,
    Retrying,
    Recovered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidEnum)]
enum SubmissionContext {
    CreatePost,
    CommentPost,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidEnum)]
enum SubmissionPhase {
    Idle,
    Submitting,
    Failed,
    Recovered,
    Succeeded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidEnum)]
enum MessageTemplate {
    None,
    EmptyPostList,
    PostCreatedCompleted,
    InvalidEditKey,
}
