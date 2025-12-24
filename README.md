# unibook

Markdownファイルから目次付きHTML書籍を生成するドキュメントジェネレーター

## 特徴

- 📚 **mdbook風のシンプルな設計** - 使いやすいコマンドラインインターフェース
- 📑 **自動目次生成** - 全ページに左サイドバーの目次を自動追加
- 📖 **Part階層のサポート** - 3段階の階層（Part/Chapter/Section）とページのネスト表示
- 🔍 **全文検索機能** - Ctrl+K でページ内を高速検索
- 🌓 **カラーテーマ** - ライト/ダークモード対応
- 📂 **H2セクション表示** - 目次にH2見出しを表示可能（階層に応じて自動インデント）
- 📋 **コピーボタン** - コードブロックにワンクリックコピー機能
- 🔄 **ライブリロード** - ファイル変更を監視して自動再ビルド
- ⚡ **差分ビルド** - 変更されたページだけを高速リビルド
- 🌐 **HTTPサーバー内蔵** - 開発用のローカルサーバー搭載
- 🚀 **高速** - Rust製で軽量・高速
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
base_path = ""                  # ベースパス（デフォルト: ""）
                                # 例: "/gnuplot-book" → リンクが /gnuplot-book/page.html になる
                                # GitHub Pagesなどでサブディレクトリにデプロイする場合に便利

[toc]
# H2セクションの表示設定
# - "current": 現在のページのみH2を表示（デフォルト）
# - "always": すべてのページでH2を表示
# - "never": H2を表示しない
show_sections = "current"

# Partの折りたたみレベル
# 指定したレベル以上のPartがデフォルトで折りたたまれます
# 0 = 折りたたみなし（デフォルト）
# 1 = Level 1以上を折りたたむ
# 2 = Level 2以上を折りたたむ
# 3 = Level 3以上を折りたたむ
foldlevel = 0

# ページの定義（この順番で目次に表示されます）

# トップレベルのページ（Partに属さない独立したページ）
[[pages]]
title = "はじめに"
path = "intro.md"

# Part（見出しのみ、子ページをネスト表示）
# level: 1=大見出し/Part（デフォルト）, 2=中見出し/Chapter, 3=小見出し/Section
[[pages]]
title = "Part 1: 基礎編"
level = 1
# items を省略すると、次のPartまでのページが自動的に子ページになる

# Part の子ページ（自動グループ化）
[[pages]]
title = "第1章: はじめの一歩"
path = "chapter1.md"

[[pages]]
title = "第2章: 基本操作"
path = "chapter2.md"

# 次の Part（ここまでが Part 1 の子）
[[pages]]
title = "Part 2: 応用編"
level = 1

# 独立したページ（どのPartにも属さない）
[[pages]]
title = "第3章"
path = "chapter3.md"

# Level 2 の Part（中見出し、opacity: 0.8）
[[pages]]
title = "Appendix"
level = 2

[[pages]]
title = "付録A"
path = "appendix-a.md"

# Level 3 の Part（小見出し、opacity: 0.6）
[[pages]]
title = "References"
level = 3
```

**もちろん、明示的に items を指定することもできます：**

```toml
[[pages]]
title = "Part 1: 基礎編"
level = 1
items = [
  { title = "第1章: はじめの一歩", path = "chapter1.md" },
  { title = "第2章: 基本操作", path = "chapter2.md" }
]
```

### Part の階層構造

Partは3段階の階層レベルを持ち、視覚的に区別されます：

| Level | 用途 | 見た目 | インデント |
|-------|------|--------|-----------|
| 1 | 大見出し（Part） | 通常の背景色 | 子ページ: 16px<br>セクション: 40px |
| 2 | 中見出し（Chapter） | opacity: 0.8 | 子ページ: 24px<br>セクション: 48px |
| 3 | 小見出し（Section） | opacity: 0.6 | 子ページ: 32px<br>セクション: 56px |

目次での表示例：

```
はじめに
Part 1: 基礎編
  ├─ 第1章: はじめの一歩
  │   ├─ Getting Started (H2)
  │   └─ Installation (H2)
  └─ 第2章: 基本操作
Part 2: 応用編
第3章
Appendix (中見出し)
  └─ 付録A
References (小見出し)
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
- ファイル変更時に自動再ビルド（差分ビルドで高速）
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

### Part階層と目次構造

unibookは3段階のPart階層をサポートし、明確な目次構造を実現します：

**ページの種類：**
- **Part（見出しのみ）**: `path` を指定せず、子ページを定義
  - `items` で明示的に指定
  - または `items` を省略すると、次のPartまでのページが自動的に子ページになる（貪欲マッチ）
- **トップレベルページ**: `path` を指定、Partに属さない独立したページ
- **子ページ**: Partの直後に続くページ（自動グループ化）または `items` で明示的に指定されたページ

**階層レベル：**
- **Level 1（大見出し/Part）**: メインセクションの区切り
- **Level 2（中見出し/Chapter）**: サブセクション
- **Level 3（小見出し/Section）**: 詳細な分類

**折りたたみ機能：**
- `foldlevel` を設定することで、指定レベル以上のPartをデフォルトで折りたたみ表示
- クリックで開閉をトグル
- 子ページとH2セクションは自動的にインデント表示され、どのPartに属しているか一目で分かります

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
theme = "light"  # 初期テーマを指定
```

利用可能なテーマ：
- `light` - ライトモード（デフォルト）
- `dark` - ダークモード
- `solarized-light` - Solarized Light
- `solarized-dark` - Solarized Dark
- `kanagawa-light` - Kanagawa Light
- `kanagawa-dark` - Kanagawa Dark

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
- 左サイドバーに全ページの目次（階層構造とインデントで見やすく表示）
- 現在のページがハイライト表示
- H2見出しのセクション表示（設定により制御可能、階層に応じて自動インデント）
- 全文検索機能（Ctrl+K または検索ボタン）
- カラーテーマ切り替え（6種類のテーマから選択可能）
- コードブロックにコピーボタン（ホバーで表示）
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
