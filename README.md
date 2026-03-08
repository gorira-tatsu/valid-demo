# valid-demo

`valid-demo` は、要件定義の段階で形式検証言語 `valid` を使うと何ができるかを理解するためのデモリポジトリです。

単なる仕様書置き場ではなく、要件を以下のような検証可能な形に落とし込む流れを確認できます。

- 仕様書を機能ごとに分割して整理する
- 仕様から状態、操作、業務ルールを `valid` のモデルに落とす
- 性質を property として明示し、自動で検査する
- coverage や explain を使って、未到達分岐や仕様の穴を見つける

現在のデモ対象は `bss` ディレクトリにあるシンプルな掲示板アプリです。

## このリポジトリで分かること

- 要件定義を「画面仕様」だけで終わらせず、検証可能な制約へ変換するやり方
- `valid` で state / action / property をどう切るか
- 1 画面単位の仕様だけでなく、一覧 -> 作成 -> 詳細 -> 編集/削除 -> コメントのような横断フローも検証できること
- 契約 drift、property 検証、coverage 確認をどの順番で回すと運用しやすいか

## リポジトリ構成

- [`bss/README.md`](/Users/tatsuhiko/code/valid-demo/bss/README.md)
  掲示板デモの入口。最初に読む前提資料、モデル構成、実行コマンドをまとめています。
- [`bss/docs/rdd/README.md`](/Users/tatsuhiko/code/valid-demo/bss/docs/rdd/README.md)
  要件定義ドキュメントの目次です。
- [`bss/docs/valid_registry_workflow.md`](/Users/tatsuhiko/code/valid-demo/bss/docs/valid_registry_workflow.md)
  `valid-registry` ベースの検証運用をまとめています。

## デモの見方

1. RDD を読み、掲示板の要件を把握する
2. `valid/board_rdd_registry.rs` と配下のモデルを見て、仕様がどのように state / action / property に落ちているか確認する
3. `inspect`, `check`, `coverage` を実行して、仕様の見え方がどう変わるか確認する
4. `board-flow` のような横断モデルを見て、機能単体では見えない整合性をどう扱うか確認する

## 現在のデモ内容

`bss` には 14 個のモデルがあります。

- 共通仕様
- 投稿一覧
- 投稿作成
- 投稿詳細
- 投稿編集 / 削除
- コメント
- 一覧描画
- 画面表示契約
- API 契約
- 編集キー保存
- リトライ UX
- 二重送信防止
- 文言契約
- 横断フロー

この構成により、正常系だけでなく次のような仕様も検証対象にできます。

- HTML エスケープやエラーコードの一貫性
- 削除済み投稿の不可視性
- 並び順や抜粋ルールの成立
- UI 文言やエラーメッセージ配置の契約
- 一覧と詳細の内容整合
- リトライ後の回復挙動

## 実行について

`bss` は `valid` を GitHub 上の commit pin で参照する形に変更しています。clone 後に Rust toolchain があれば、ローカルに `../../valid` を置かなくても依存解決できる前提です。

一方で、このリポジトリ自体のライセンスはまだ未設定です。public repository として公開する前に、配布条件を明示するライセンスファイルは追加した方がよい状態です。
