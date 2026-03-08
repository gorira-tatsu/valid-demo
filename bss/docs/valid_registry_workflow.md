# valid-registry MCP Workflow

## 目的
- `valid/board_rdd_registry.rs` を一次ソースとして扱い、`valid-registry` MCP で contract drift と property 検証を再現可能にする。
- 「ビルド済み binary が古い」「どの model / property を見るべきか不明」という運用ミスを減らす。

## 前提
- registry binary は `target/debug/bss-valid-models` を使う。
- ソース変更後は必ず `cargo build` を先に実行する。MCP は Rust ソースではなく binary を読む。
- 契約固定には [valid/contract-lock.json](/Users/tatsuhiko/code/valid-demo/bss/valid/contract-lock.json) を使う。

## 最小手順
1. `cargo build`
2. `target/debug/bss-valid-models contract check valid/contract-lock.json --json`
3. model ごとに `inspect` と `lint`
4. 確認対象の property を `check` で明示検証
5. `coverage` で guard と transition の未到達を確認

## MCP で使う順番
1. `valid_list_models`
2. `valid_contract_check`
3. `valid_inspect`
4. `valid_lint`
5. `valid_check`
6. `valid_coverage`
7. 必要なら `valid_explain` と `valid_testgen`

## 代表コマンド
```sh
cargo build
./target/debug/bss-valid-models contract check valid/contract-lock.json --json
./target/debug/bss-valid-models inspect board-common-spec --json
./target/debug/bss-valid-models check board-common-spec --property=P_COMMON_BAD_REQUEST_HIDES_INVALID_RESOURCE --json
./target/debug/bss-valid-models coverage board-common-spec --json
./scripts/verify_valid_registry.sh
cargo valid suite --json
```

`./scripts/verify_valid_registry.sh` は以下をまとめて行う:
- contract lock の drift check
- 14 model すべての lint
- 全 invariant の property check
- coverage JSON の収集と gate fail の可視化
- `board-flow` を含む suite 検証の前提整備

`valid.toml` には MCP から直接回せる設定も置く:
- `critical_properties`: 各 model の代表 invariant を 1 件ずつ
- `property_suites.smoke`: 横断的に壊れやすい重要 property を少数束ねた確認用 suite

## 重点モデル
- `board-common-spec`
  400/403/404/5xx、匿名補完、HTML エスケープの基底制約
- `board-edit-delete`
  編集キー一致、削除確認、論理削除後の非表示
- `board-list-rendering`
  並び順、120 文字抜粋、継続 UI
- `board-retry-ux`
  エラー表示位置と再試行回復
- `board-message-contract`
  画面ごとの主要文言束縛
- `board-flow`
  作成後の可視性、削除後の非表示、コメント拒否を横断的に確認

## 運用ルール
- property 名を推測しない。先に `inspect` で `property_details` を読む。
- `solver_ready` を `lint` か `inspect` で確認する前に「形式検証済み」と言わない。
- contract drift が出たら先に差分の妥当性をレビューし、問題なければ lock を更新する。
- coverage fail は即バグとは限らないが、未到達 guard を放置したまま「検証十分」とは言わない。

## 現在の固定対象
- exported models は 14 個:
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
