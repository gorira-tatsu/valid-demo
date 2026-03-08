use valid::{registry::run_registry_cli, valid_model, valid_models, ValidAction, ValidEnum, ValidState};

/*
How to read the `docs/rdd` to `valid` mapping:

- specification text:
  Human-readable business rules such as length limits, 404 behavior, and
  invisibility after deletion.
- valid state:
  A finite abstraction of the facts that should be checked.
- valid action:
  A single-step transition representing a screen action or API call.
- valid property:
  An invariant that should always hold as part of the requirement.

These files do not implement the specification directly. They extract only the
rules that must not be broken and express them as finite-state machines.
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
