# BSS valid demo

このディレクトリは、シンプルな掲示板アプリの要件定義を `valid` で形式化し、何が検証できるかを示すデモです。

## このデモの目的

このデモは「valid を使って実装を検証する」より一段前の段階、つまり「要件定義の時点で何を機械的に確認できるか」を見せることを目的にしています。

具体的には次の問いに答えるためのサンプルです。

- 仕様を state / action / property にどう分解するか
- 画面仕様、API 契約、業務ルール、UX 制約を同じ土台で扱えるか
- 正常系だけでなく、エラー系、境界値、削除後の不可視性、二重送信防止、メッセージ整合まで検証できるか

## 先に読むファイル

1. [`docs/rdd/README.md`](/Users/tatsuhiko/code/valid-demo/bss/docs/rdd/README.md)
2. [`docs/rdd/07_valid_models.md`](/Users/tatsuhiko/code/valid-demo/bss/docs/rdd/07_valid_models.md)
3. [`docs/rdd/08_BBS成立要件.md`](/Users/tatsuhiko/code/valid-demo/bss/docs/rdd/08_BBS成立要件.md)
4. [`docs/valid_registry_workflow.md`](/Users/tatsuhiko/code/valid-demo/bss/docs/valid_registry_workflow.md)

## モデル構成

実際に公開されているモデルは 14 個です。

- `board-common-spec`
  400/403/404/5xx、匿名補完、HTML エスケープなどの共通制約
- `board-post-list`
  一覧取得、ページング、空状態、リンク導線
- `board-post-create`
  投稿作成、入力異常、成功時遷移、送信中状態
- `board-post-detail`
  詳細表示、未存在時の扱い、更新日時表示、コメント数
- `board-edit-delete`
  編集キー、削除確認、削除後不可視
- `board-comment`
  コメント投稿、失敗時のフォーム保持、表示順
- `board-list-rendering`
  新着順、古い順、120 文字抜粋、継続 UI
- `board-presentation-contract`
  日時表示形式、本文レンダリング、成功 / 失敗メッセージ
- `board-api-contract`
  JSON 契約、成功 / 失敗レスポンス項目
- `board-edit-key-storage`
  編集キーの保存方針
- `board-retry-ux`
  リトライ導線、エラー表示位置、復帰時のメッセージ整理
- `board-submission-discipline`
  二重送信防止と再試行可能性
- `board-message-contract`
  主要文言がどの状態に束縛されるか
- `board-flow`
  一覧、詳細、編集、削除、コメントを横断した整合性

## valid で何ができるか

このデモでは、要件定義をもとに少なくとも次の確認ができます。

- `models`
  どの要件領域が独立したモデルとして切り出されているか一覧できる
- `inspect`
  state fields、actions、properties、read/write 情報を機械的に確認できる
- `check`
  要件を property として検証し、成立 / 不成立を判定できる
- `coverage`
  遷移、guard、分岐の未到達を可視化できる
- `contract check`
  model contract の drift を検知できる
- `lint`
  solver-ready か、説明や test generation に必要なメタデータが揃っているか確認できる
- `explain`
  property が壊れたときに、どの遷移と状態変化で破綻したか追跡できる

## 実行例

前提:

- Rust toolchain
- GitHub から `valid` 依存を取得できるネットワーク接続

モデル一覧:

```sh
cargo run --bin bss-valid-models -- models
```

共通仕様モデルの構造確認:

```sh
cargo run --bin bss-valid-models -- inspect board-common-spec --json
```

代表 property の検証:

```sh
cargo run --bin bss-valid-models -- check board-common-spec --property=P_COMMON_HTML_IS_ALWAYS_ESCAPED --json
```

coverage の確認:

```sh
cargo run --bin bss-valid-models -- coverage board-common-spec --json
```

一括検証:

```sh
./scripts/verify_valid_registry.sh
```

## 現在の検証状態

2026-03-08 時点では、`./scripts/verify_valid_registry.sh` の全件通過はしていません。

- `board-post-list`
- property: `P_LIST_EMPTY_STATE_MATCHES_VISIBLE_COUNT`

到達反例では、ページ overflow 時に `visible_posts == 0` である一方 `empty_state_visible == false` となっており、一覧の空状態定義とページ overflow 表現の境界が未整理であることが分かります。

これは public repository としては「既知の仕様検討ポイント」として扱うのが自然です。README を読んだ第三者が誤解しないよう、成功例だけでなく現状の未解決点も明示しています。

## 読みどころ

このデモで特に見る価値が高いのは次の点です。

- `board-common-spec`
  仕様の最小単位をどう invariant にするか
- `board-list-rendering`
  UI 表示ルールを単なる口頭説明ではなく検証可能な条件にする方法
- `board-retry-ux`
  UX 制約もモデル化できること
- `board-message-contract`
  文言も「どこで出るべきか」を契約化できること
- `board-flow`
  個別機能モデルを超えた整合性確認

## 注意点

このデモの `valid` 依存は GitHub の commit pin にしています。再現性は上がりますが、初回 build にはネットワーク接続が必要です。

また、このリポジトリ自体のライセンスはまだ未設定です。public repository として外部に公開する前に、利用条件を明示するライセンスファイルを追加することを推奨します。
