/*
Requirements covered:
- docs/rdd/00_assumptions_and_scope.md
- docs/rdd/01_common_specification.md
- docs/rdd/05_post_edit_and_delete.md

This model covers:
- allowing update/delete only when the edit key matches
- delete confirmation and logical deletion
- invisibility from list/detail after deletion
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidState)]
struct EditDeleteState {
    post_exists: bool,
    post_deleted: bool,
    edit_key_matches: bool,
    form_prefilled: bool,
    form_preserved: bool,
    delete_dialog_visible: bool,
    deletion_confirmed: bool,
    updated_at_changed: bool,
    post_visible: bool,
    #[valid(enum)]
    outcome: ApiStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidAction)]
enum EditDeleteAction {
    #[valid(action_id = "SETUP_EDITABLE_POST_WITH_VALID_KEY", reads = [], writes = ["post_exists", "post_deleted", "edit_key_matches", "form_prefilled", "form_preserved", "delete_dialog_visible", "deletion_confirmed", "updated_at_changed", "post_visible", "outcome"])]
    SetupEditablePostWithValidKey,
    #[valid(action_id = "SETUP_EDITABLE_POST_WITH_WRONG_KEY", reads = [], writes = ["post_exists", "post_deleted", "edit_key_matches", "form_prefilled", "form_preserved", "delete_dialog_visible", "deletion_confirmed", "updated_at_changed", "post_visible", "outcome"])]
    SetupEditablePostWithWrongKey,
    #[valid(action_id = "SETUP_DELETE_CONFIRMATION_WITH_VALID_KEY", reads = [], writes = ["post_exists", "post_deleted", "edit_key_matches", "form_prefilled", "form_preserved", "delete_dialog_visible", "deletion_confirmed", "updated_at_changed", "post_visible", "outcome"])]
    SetupDeleteConfirmationWithValidKey,
    #[valid(action_id = "SETUP_DELETE_CONFIRMATION_WITH_WRONG_KEY", reads = [], writes = ["post_exists", "post_deleted", "edit_key_matches", "form_prefilled", "form_preserved", "delete_dialog_visible", "deletion_confirmed", "updated_at_changed", "post_visible", "outcome"])]
    SetupDeleteConfirmationWithWrongKey,
    #[valid(action_id = "SETUP_MISSING_POST_EDIT", reads = [], writes = ["post_exists", "post_deleted", "edit_key_matches", "form_prefilled", "form_preserved", "delete_dialog_visible", "deletion_confirmed", "updated_at_changed", "post_visible", "outcome"])]
    SetupMissingPostEdit,
    #[valid(action_id = "SETUP_DELETED_POST_EDIT", reads = [], writes = ["post_exists", "post_deleted", "edit_key_matches", "form_prefilled", "form_preserved", "delete_dialog_visible", "deletion_confirmed", "updated_at_changed", "post_visible", "outcome"])]
    SetupDeletedPostEdit,
    #[valid(action_id = "OPEN_EDIT_FORM", reads = ["post_exists", "post_deleted"], writes = ["form_prefilled", "outcome"])]
    OpenEditForm,
    #[valid(action_id = "EDIT_WITH_VALID_KEY", reads = ["post_exists", "post_deleted", "edit_key_matches"], writes = ["updated_at_changed", "form_preserved", "outcome"])]
    EditWithValidKey,
    #[valid(action_id = "EDIT_WITH_WRONG_KEY", reads = ["post_exists", "post_deleted", "edit_key_matches"], writes = ["form_preserved", "outcome"])]
    EditWithWrongKey,
    #[valid(action_id = "SHOW_DELETE_DIALOG", reads = ["post_exists", "post_deleted"], writes = ["delete_dialog_visible"])]
    ShowDeleteDialog,
    #[valid(action_id = "DELETE_WITH_VALID_KEY", reads = ["post_exists", "post_deleted", "edit_key_matches", "delete_dialog_visible"], writes = ["post_deleted", "deletion_confirmed", "post_visible", "outcome"])]
    DeleteWithValidKey,
    #[valid(action_id = "DELETE_WITH_WRONG_KEY", reads = ["post_exists", "post_deleted", "edit_key_matches", "delete_dialog_visible"], writes = ["form_preserved", "outcome"])]
    DeleteWithWrongKey,
    #[valid(action_id = "OPERATE_MISSING_POST", reads = ["post_exists"], writes = ["outcome"])]
    OperateMissingPost,
}

valid_model! {
    model EditDeleteModel<EditDeleteState, EditDeleteAction>;
    init [EditDeleteState {
        post_exists: true,
        post_deleted: false,
        edit_key_matches: true,
        form_prefilled: false,
        form_preserved: false,
        delete_dialog_visible: false,
        deletion_confirmed: false,
        updated_at_changed: false,
        post_visible: true,
        outcome: ApiStatus::Idle,
    }];
    transitions {
        transition SetupEditablePostWithValidKey [role = setup] [tags = ["edit_path", "setup_path"]]
        when |state| true
        => [EditDeleteState
            {
                post_exists : true, post_deleted : false, edit_key_matches : true,
                form_prefilled : false, form_preserved : false, delete_dialog_visible :
                false, deletion_confirmed : false, updated_at_changed : false,
                post_visible : true, outcome : ApiStatus::Idle
            }
        ];
        transition SetupEditablePostWithWrongKey [role = setup] [tags = ["edit_path", "security_path", "setup_path"]]
        when |state| true
        => [EditDeleteState
            {
                post_exists : true, post_deleted : false, edit_key_matches : false,
                form_prefilled : false, form_preserved : false, delete_dialog_visible :
                false, deletion_confirmed : false, updated_at_changed : false,
                post_visible : true, outcome : ApiStatus::Idle
            }
        ];
        transition SetupDeleteConfirmationWithValidKey [role = setup] [tags = ["delete_path", "setup_path"]]
        when |state| true
        => [EditDeleteState
            {
                post_exists : true, post_deleted : false, edit_key_matches : true,
                form_prefilled : true, form_preserved : false, delete_dialog_visible :
                true, deletion_confirmed : false, updated_at_changed : false, post_visible
                : true, outcome : ApiStatus::Ok
            }
        ];
        transition SetupDeleteConfirmationWithWrongKey [role = setup] [tags = ["delete_path", "security_path", "setup_path"]]
        when |state| true
        => [EditDeleteState
            {
                post_exists : true, post_deleted : false, edit_key_matches : false,
                form_prefilled : true, form_preserved : false, delete_dialog_visible :
                true, deletion_confirmed : false, updated_at_changed : false, post_visible
                : true, outcome : ApiStatus::Ok
            }
        ];
        transition SetupMissingPostEdit [role = setup] [tags = ["edit_path", "resource_path", "setup_path"]]
        when |state| true
        => [EditDeleteState
            {
                post_exists : false, post_deleted : false, edit_key_matches : false,
                form_prefilled : false, form_preserved : false, delete_dialog_visible :
                false, deletion_confirmed : false, updated_at_changed : false,
                post_visible : false, outcome : ApiStatus::Idle
            }
        ];
        transition SetupDeletedPostEdit [role = setup] [tags = ["edit_path", "resource_path", "setup_path"]]
        when |state| true
        => [EditDeleteState
            {
                post_exists : true, post_deleted : true, edit_key_matches : true,
                form_prefilled : false, form_preserved : false, delete_dialog_visible :
                false, deletion_confirmed : true, updated_at_changed : false, post_visible
                : false, outcome : ApiStatus::NotFound
            }
        ];
        transition OpenEditForm [tags = ["allow_path", "edit_path"]]
        when |state| state.post_exists && !state.post_deleted
        => [EditDeleteState { form_prefilled : true, outcome : ApiStatus::Ok, .. state }
        ];
        transition EditWithValidKey [tags = ["allow_path", "edit_path"]]
        when |state| state.post_exists && !state.post_deleted && state.edit_key_matches
        => [EditDeleteState
            {
                updated_at_changed : true, form_preserved : false, outcome :
                ApiStatus::Ok, .. state
            }
        ];
        transition EditWithWrongKey [tags = ["deny_path", "edit_path", "security_path"]]
        when |state| state.post_exists && !state.post_deleted && state.edit_key_matches == false
        => [EditDeleteState
            { form_preserved : true, outcome : ApiStatus::Forbidden, .. state }
        ];
        transition ShowDeleteDialog [tags = ["allow_path", "delete_path"]]
        when |state| state.post_exists && !state.post_deleted
        => [EditDeleteState { delete_dialog_visible : true, .. state }
        ];
        transition DeleteWithValidKey [tags = ["allow_path", "delete_path"]]
        when |state| state.post_exists && !state.post_deleted && state.edit_key_matches &&
            state.delete_dialog_visible
        => [EditDeleteState
            {
                post_deleted : true, deletion_confirmed : true, form_prefilled : false,
                post_visible : false, outcome : ApiStatus::Ok, .. state
            }
        ];
        transition DeleteWithWrongKey [tags = ["delete_path", "deny_path", "security_path"]]
        when |state| state.post_exists && !state.post_deleted && state.edit_key_matches == false &&
            state.delete_dialog_visible
        => [EditDeleteState
            { form_preserved : true, outcome : ApiStatus::Forbidden, .. state }
        ];
        transition OperateMissingPost [tags = ["deny_path", "resource_path"]]
        when |state| state.post_exists == false || state.post_deleted
        => [EditDeleteState
            { outcome : ApiStatus::NotFound, post_visible : false, .. state }
        ];
    }
    properties {
        invariant P_EDIT_FORM_IS_PREFILLED_WHEN_OPENED |state|
            state.outcome != ApiStatus::Ok || state.form_prefilled ||
            state.updated_at_changed || state.post_deleted;
        invariant P_EDIT_WRONG_KEY_RETURNS_FORBIDDEN |state|
            state.outcome != ApiStatus::Forbidden || state.form_preserved;
        invariant P_DELETE_REQUIRES_CONFIRMATION |state|
            state.post_deleted == false || state.deletion_confirmed;
        invariant P_DELETE_HIDES_POST_FROM_VIEWS |state|
            state.post_deleted == false || state.post_visible == false;
        invariant P_DELETED_POST_CANNOT_BE_REEDITED |state|
            state.post_deleted == false || state.form_prefilled == false;
    }
}
