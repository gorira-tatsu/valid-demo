# 07. valid モデル化

## 目的
- `docs/rdd` の要件を `valid` で検証可能な有限状態モデルへ写像する。
- 画面文言や JSON 形式そのものではなく、業務ルールと遷移制約を invariant と transition に落とす。

## 配置
- registry: `valid/board_rdd_registry.rs`
- exported models:
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

## モデル化方針
- 文字列の実値ではなく、`title_len` `body_len` `edit_key_len` のような有限属性へ抽象化する。
- 画面仕様は `screen` `phase` `empty_state_visible` `form_preserved` などの UI 事実に還元する。
- API 仕様は `ApiStatus` を通じて `Ok` `BadRequest` `Forbidden` `NotFound` `ServerError` に正規化する。
- 論理削除は `post_deleted` と `post_visible` の組み合わせで扱い、一覧・詳細からの非表示性を invariant で固定する。

## RDD との対応

### `00_前提とスコープ.md`
- 認証なし、編集用キーで本人確認:
  - `CommonSpecModel`
  - `EditDeleteModel`
- 削除済みデータを表示しない:
  - `PostListModel`
  - `PostDetailModel`
  - `EditDeleteModel`

### `01_共通仕様.md`
- 投稿/コメントの必須入力と文字数制約:
  - `CommonSpecModel`
- 匿名時の `名無しさん` 補完:
  - `CommonSpecModel`
  - `PostCreateModel`
  - `CommentModel`
- `updatedAt` の表示有無と更新成功時のみの反映:
  - `PostDetailModel`
  - `BoardFlowModel`
- `400 / 403 / 404 / 5xx` の使い分け:
  - `CommonSpecModel`
  - `EditDeleteModel`
  - `CommentModel`
- JSON エラー応答の基本契約:
  - `ApiContractModel`
- HTML エスケープ前提:
  - `CommonSpecModel`
  - `PresentationContractModel`
- 日時表示フォーマット、改行保持、再試行導線の配置:
  - `PresentationContractModel`
  - `RetryUxModel`
- API の JSON 応答構造:
  - `ApiContractModel`
- `editKey` のハッシュ保存方針:
  - `EditKeyStorageModel`

### `02_投稿一覧機能.md`
- 20 件上限、空状態、詳細導線、新規投稿導線:
  - `PostListModel`
- `page` `limit` 境界条件と不正値拒否:
  - `PostListModel`
- 「新しい順 / 古い順」の timestamp 比較、120 文字抜粋、ページネーション or もっと見る:
  - `ListRenderingModel`
- 同一データ集合に対する安定順序:
  - `ListRenderingModel`
- 空状態文言:
  - `MessageContractModel`

### `03_投稿作成機能.md`
- 成功時の詳細遷移、失敗時の入力保持、送信中の多重送信防止:
  - `PostCreateModel`
  - `SubmissionDisciplineModel`

### `04_投稿詳細機能.md`
- 未削除投稿のみ詳細表示、コメント空状態、古い順表示、更新日時表示:
  - `PostDetailModel`

### `05_投稿編集・削除機能.md`
- 編集用キー一致時のみ更新/削除、削除確認、論理削除、削除後非表示:
  - `EditDeleteModel`
- 編集用キー不一致メッセージ:
  - `MessageContractModel`

### `06_コメント機能.md`
- 削除済み投稿へのコメント拒否、成功時の詳細反映、失敗時の入力保持:
  - `CommentModel`
- コメント送信中の重複送信防止、失敗後再送規律:
  - `SubmissionDisciplineModel`

### `08_BBS成立要件.md`
- 投稿作成から一覧、詳細、更新反映、コメント件数整合、削除後非表示、コメント拒否までの横断整合:
  - `BoardFlowModel`
- 一覧再取得や詳細再取得を跨いだ整合維持:
  - `BoardFlowModel`
- 更新済み投稿の詳細取得失敗から retry 回復後に、`updatedAt` 表示とコメント件数整合を復元する横断整合:
  - `BoardFlowModel`
- 成功メッセージと retry banner の排他、失敗後再試行導線:
  - `PresentationContractModel`
  - `RetryUxModel`
- 作成/コメント送信中の重複送信防止と回復後の通常復帰:
  - `SubmissionDisciplineModel`

## 検証観点
- 1 つの巨大モデルにせず、機能単位で分割して状態空間を抑える。
- 各 model は章ごとの中核 invariant を持ち、`valid-registry` で個別に inspect / verify できる。
- 実装時に追加したい仕様が増えたら、まず該当章の model に action と invariant を追加する。

## valid-registry 運用
- binary 基準で検証するため、`valid/board_rdd_registry.rs` を更新したら先に `cargo build` する。
- contract drift の基準は [valid/contract-lock.json](/Users/tatsuhiko/code/valid-demo/bss/valid/contract-lock.json) に固定する。
- 運用手順は [docs/valid_registry_workflow.md](/Users/tatsuhiko/code/valid-demo/bss/docs/valid_registry_workflow.md) を参照する。
- 最低限の確認順:
  - `valid_contract_check`
  - `valid_inspect`
  - `valid_lint`
  - `valid_check`
  - `valid_coverage`

## 今回追加した厳密化
- `ListRenderingModel`
  - `SortOrder` と先頭 2 件の `timestamp` 比較で、新しい順 / 古い順を不変条件として検証する。
  - 本文抜粋は `body_len` と `excerpt_len` に圧縮し、120 文字超過時は `excerpt_ellipsized` を必須化する。
  - 続きの UI は `ContinuationUi::Pagination | LoadMore` として差分を残す。
- `RetryUxModel`
  - 一覧の読み込み失敗は `TopBanner`、各フォーム送信失敗は `BelowForm` に固定する。
  - 再試行は `ErrorShown -> Retrying -> Recovered` の回復遷移として検証する。
- `MessageContractModel`
  - 主要文言を `EmptyPostList / PostCreatedCompleted / InvalidEditKey` の有限集合に落とし、画面と結果コードに結び付ける。
- `PresentationContractModel`
  - 日時表示 `YYYY-MM-DD HH:mm` を桁数と区切り文字数へ分解して検証する。
  - 本文表示は `html_escaped` と `newline_preserved` の両立を要求する。
  - 再試行導線は `RetryMessagePlacement::TopBanner | BelowForm` で位置を明示する。
  - 投稿成功文言は `SuccessMessageKind::PostCreatedCompleted` に固定する。
- `BoardFlowModel`
  - 更新済み投稿の詳細取得失敗を `RetryPhase::ErrorShown` に落とし、retry 可能かつ一覧へ戻れることを invariant にする。
  - 回復後は `RetryPhase::Recovered` で、`updatedAt` 表示と一覧/詳細コメント件数整合が同時に復元されることを検証する。
- `ApiContractModel`
  - `GET /posts` と `POST /posts` のレスポンスフィールド集合を invariant として固定する。
  - エラー時も JSON 基本構造を維持する前提を API 契約として扱う。
- `EditKeyStorageModel`
  - `editKey` 保存時に平文永続化しないこと、保存済みならハッシュ化されていることをポリシーとして固定する。
