# valid-registry MCP Workflow

## Goal
- Treat `valid/board_rdd_registry.rs` as the source of truth and make contract drift checks and property verification reproducible through the `valid-registry` MCP flow.
- Avoid stale binaries and unclear model/property selection.

## Assumptions
- The registry binary is `target/debug/bbs-valid-models`.
- After changing the source, always run `cargo build` first. The MCP flow reads the built binary, not the Rust source files directly.
- Use [valid/contract-lock.json](/Users/tatsuhiko/code/valid-demo/bbs/valid/contract-lock.json) as the contract lock.

## Minimal Procedure
1. `cargo build`
2. `target/debug/bbs-valid-models contract check valid/contract-lock.json --json`
3. Run `inspect` and `lint` for each model
4. Explicitly verify the properties you care about with `check`
5. Use `coverage` to review uncovered guards and transitions

## Recommended MCP Call Order
1. `valid_list_models`
2. `valid_contract_check`
3. `valid_inspect`
4. `valid_lint`
5. `valid_check`
6. `valid_coverage`
7. Use `valid_explain` and `valid_testgen` when needed

## Representative Commands
```sh
cargo build
./target/debug/bbs-valid-models contract check valid/contract-lock.json --json
./target/debug/bbs-valid-models inspect board-common-spec --json
./target/debug/bbs-valid-models check board-common-spec --property=P_COMMON_BAD_REQUEST_HIDES_INVALID_RESOURCE --json
./target/debug/bbs-valid-models coverage board-common-spec --json
./scripts/verify_valid_registry.sh
cargo valid suite --json
```

`./scripts/verify_valid_registry.sh` bundles the following:
- Drift checking for the contract lock
- Linting for all 14 models
- Property checks for all invariants
- Collection of coverage JSON plus gate-failure visibility
- Preparation for suite verification including `board-flow`

`valid.toml` also contains settings that can be driven directly through MCP:
- `critical_properties`: one representative invariant per model
- `property_suites.smoke`: a compact cross-model suite of high-value properties that are easy to regress

## High-Value Models
- `board-common-spec`
  Core constraints for 400/403/404/5xx behavior, anonymous defaults, and HTML escaping
- `board-edit-delete`
  Edit-key matching, delete confirmation, and invisibility after logical deletion
- `board-list-rendering`
  Ordering, 120-character excerpts, and continuation UI
- `board-retry-ux`
  Error placement and retry recovery
- `board-message-contract`
  Binding important messages to the right screen contexts
- `board-flow`
  Cross-cutting verification of visibility after creation, invisibility after deletion, and comment rejection when unavailable

## Operating Rules
- Do not guess property names. Read `property_details` through `inspect` first.
- Do not claim something is "formally verified" before confirming `solver_ready` in `lint` or `inspect`.
- If contract drift appears, review the diff first and update the lock only when the change is intentional.
- A coverage failure is not automatically a bug, but uncovered guards should not be ignored while claiming verification is sufficient.

## Currently Exported Models
- There are 14 exported models:
  - `board-common-spec`
  - `board-post-list`
  - `board-post-create`
  - `board-post-detail`
  - `board-edit-delete`
  - `board-comment`
  - `board-list-rendering`
  - `board-presentation-contract`
  - `board-api-contract`
  - `board-edit-key-storage`
  - `board-retry-ux`
  - `board-submission-discipline`
  - `board-message-contract`
  - `board-flow`
