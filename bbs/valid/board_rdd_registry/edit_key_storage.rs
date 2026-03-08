/*
Requirements covered:
- docs/rdd/01_common_specification.md

This model covers:
- never persisting `editKey` in plaintext
- requiring stored keys to be hashed
- a storage policy that does not rely on re-displaying the user-entered key
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidState)]
struct EditKeyStorageState {
    save_attempted: bool,
    edit_key_hashed: bool,
    plaintext_edit_key_persisted: bool,
    #[valid(enum)]
    outcome: ApiStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValidAction)]
enum EditKeyStorageAction {
    #[valid(action_id = "STORE_HASHED_EDIT_KEY", reads = ["save_attempted"], writes = ["save_attempted", "edit_key_hashed", "plaintext_edit_key_persisted", "outcome"])]
    StoreHashedEditKey,
}

valid_model! {
    model EditKeyStorageModel<EditKeyStorageState, EditKeyStorageAction>;
    init [EditKeyStorageState {
        save_attempted: false,
        edit_key_hashed: false,
        plaintext_edit_key_persisted: false,
        outcome: ApiStatus::Idle,
    }];
    transitions {
        transition StoreHashedEditKey [tags = ["allow_path", "security_path", "storage_path"]]
        when |state| state.save_attempted == false
        => [EditKeyStorageState
            {
                save_attempted : true, edit_key_hashed : true,
                plaintext_edit_key_persisted : false, outcome : ApiStatus::Ok
            }
        ];
    }
    properties {
        invariant P_EDIT_KEY_STORAGE_NEVER_PERSISTS_PLAINTEXT |state|
            state.plaintext_edit_key_persisted == false;
        invariant P_EDIT_KEY_STORAGE_USES_HASH_WHEN_SAVED |state|
            state.save_attempted == false || state.edit_key_hashed;
    }
}
