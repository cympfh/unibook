# unibook

Markdownファイルから目次付きHTML書籍を生成するドキュメントジェネレーター

## 特徴

- 📚 **mdbook風のシンプルな設計** - 使いやすいコマンドラインインターフェース
- 📑 **自動目次生成** - 全ページに左サイドバーの目次を自動追加
- 🔍 **全文検索機能** - Ctrl+K でページ内を高速検索
- 🌓 **カラーテーマ** - ライト/ダークモード対応
- 📂 **H2セクション表示** - 目次にH2見出しを表示可能
- 🔄 **ライブリロード** - ファイル変更を監視して自動再ビルド
- 🌐 **HTTPサーバー内蔵** - 開発用のローカルサーバー搭載
- ⚡ **高速** - Rust製で軽量・高速
- 🎨 **レスポンシブデザイン** - モバイルでも読みやすい

## インストール

### 前提条件

- Rust (2024 edition対応版)
- [unidoc](https://github.com/cympfh/unidoc) - Markdown変換エンジン

```bash
# unidoc のインストール
cargo install unidoc

# unibook のビルド
git clone https://github.com/your-username/unibook
cd unibook
cargo build --release
```

バイナリは `target/release/unibook` に生成されます。

## クイックスタート

### 1. 新しい書籍を作成

```bash
unibook init my-book
cd my-book
```

生成されるファイル構成：

```
my-book/
├── book.toml       # 書籍の設定ファイル
└── src/
    ├── intro.md    # イントロダクション
    └── chapter1.md # 第1章
```

### 2. 書籍をビルド

```bash
unibook build
```

`docs/` ディレクトリにHTMLファイルが生成されます。

### 3. 開発サーバーで確認

```bash
unibook serve
```

ブラウザで http://localhost:3000/ を開きます。

ファイルを編集すると**自動的に再ビルド**されます！

## 設定ファイル (book.toml)

```toml
[book]
title = "私の書籍"              # 書籍のタイトル
description = "書籍の説明"      # 説明（オプション）
authors = ["著者名"]            # 著者（オプション）
language = "ja"                 # 言語（デフォルト: ja）
theme = "light"                 # カラーテーマ: "light" または "dark" (デフォルト: light)

[build]
src_dir = "src"                 # ソースディレクトリ（デフォルト: src）
output_dir = "docs"             # 出力ディレクトリ（デフォルト: docs）

[toc]
# H2セクションの表示設定
# - "current": 現在のページのみH2を表示（デフォルト）
# - "always": すべてのページでH2を表示
# - "never": H2を表示しない
show_sections = "current"

# ページの定義（この順番で目次に表示されます）
[[pages]]
title = "はじめに"
path = "intro.md"

[[pages]]
title = "第1章"
path = "chapter1.md"

[[pages]]
title = "第2章"
path = "chapter2.md"
```

## コマンド一覧

### `unibook init [ディレクトリ]`

新しい書籍プロジェクトを初期化します。

```bash
unibook init              # カレントディレクトリに作成
unibook init my-book      # my-book ディレクトリに作成
```

### `unibook build`

書籍をビルドしてHTMLを生成します。

```bash
unibook build             # カレントディレクトリの book.toml を使用
unibook build -d ../docs  # 別のディレクトリを指定
```

### `unibook serve`

開発用HTTPサーバーを起動します。**ファイル監視も自動的に有効になります。**

```bash
unibook serve              # ポート 3000 でサーブ
unibook serve -p 8080      # ポート指定
unibook serve -d ../docs   # 別のディレクトリ
```

機能：
- HTTPサーバーでHTMLを配信
- `src/` ディレクトリと `book.toml` を監視
- ファイル変更時に自動再ビルド
- ブラウザをリロードすれば最新版が表示される

### `unibook watch`

ファイル監視のみ実行します（サーバーなし）。

```bash
unibook watch              # カレントディレクトリを監視
unibook watch -d ../docs   # 別のディレクトリ
unibook watch --dev        # 開発モード（unibookソースコード自体のホットリロード）
```

## 開発ワークフロー

### 通常の執筆

```bash
# ターミナル1: サーバー起動
$ unibook serve
Serving book at http://localhost:3000/
Watching for changes in ./src...

# ターミナル2: ファイルを編集
$ vim src/chapter2.md

# 保存すると自動的にビルドされる
# ブラウザをリロードして確認
```

### ポートを変更したい

```bash
unibook serve --port 8080
```

### ビルドのみ実行

CI/CDや本番環境では：

```bash
unibook build
```

## 機能詳細

### 全文検索

生成されたHTMLには全文検索機能が組み込まれています：

- **Ctrl+K** で検索ダイアログを開く
- サイドバーの検索ボタンをクリック
- タイトルと本文から検索
- リアルタイムでフィルタリング

### カラーテーマ

`book.toml` で初期テーマを設定できます：

```toml
[book]
theme = "light"  # または "dark"
```

生成されたHTMLでテーマ切り替えボタンから変更可能です。

### H2セクション表示

目次にH2見出しを表示するかどうかを制御できます：

```toml
[toc]
show_sections = "current"  # 現在のページのみ（デフォルト）
# show_sections = "always"   # すべてのページで表示
# show_sections = "never"    # 表示しない
```

- **current**: 現在開いているページのH2見出しのみ表示
- **always**: すべてのページのH2見出しを常に表示
- **never**: H2見出しを表示しない

## 出力例

生成されるHTML構造：

```
docs/
├── index.html      # トップページ（自動生成、最初のページにリダイレクト）
├── intro.html      # イントロダクション
├── chapter1.html   # 第1章
└── chapter2.html   # 第2章
```

各HTMLファイルには：
- 左サイドバーに全ページの目次
- 現在のページがハイライト表示
- H2見出しのセクション表示（設定により制御可能）
- 全文検索機能（Ctrl+K または検索ボタン）
- カラーテーマ切り替え
- レスポンシブデザイン（スマホ対応）

## トラブルシューティング

### `unidoc not found`

unidocがインストールされていません：

```bash
cargo install unidoc
```

### `book.toml not found`

カレントディレクトリに `book.toml` がありません：

```bash
unibook init  # 新規作成
# または
unibook build -d /path/to/book  # ディレクトリを指定
```

### ポートが使用中

別のポートを指定してください：

```bash
unibook serve --port 3001
```

## 技術スタック

- **言語**: Rust (edition 2024)
- **Markdown変換**: [unidoc](https://github.com/cympfh/unidoc)
- **設定ファイル**: TOML
- **HTTPサーバー**: tiny_http
- **ファイル監視**: notify

## ライセンス

このプロジェクトのライセンスについては LICENSE ファイルを参照してください。

## 関連プロジェクト

- [mdBook](https://github.com/rust-lang/mdBook) - Rust製の書籍ジェネレーター
- [unidoc](https://github.com/cympfh/unidoc) - Markdown to HTML コンバーター

## Contributing

Issue や Pull Request は大歓迎です！

## @CLAUDE

- Rust, rustc 1.92.0
    - cargo fmt --all -- --check
    - cargo clippy --all-targets --all-features -- -D warnings
- Check @TODO.md

---

**Happy Writing! 📝**
