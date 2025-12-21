# リリースノート

## 新機能 (2025-12-21)

- **Part階層のサポート**: 目次でchapterより上の階層（part）を表現できるようになりました
  - `book.toml` で `path` を指定しない `[[pages]]` エントリを part として扱います
  - part は目次に太字・uppercaseで表示され、リンクはありません（見出しのみ）
  - HTMLページは生成されません
  - 設定例:
    ```toml
    [[pages]]
    title = "Part 1: 基礎編"
    # path なし = part

    [[pages]]
    title = "第1章"
    path = "chapter1.md"
    ```
  - 変更されたファイル: `src/config.rs`, `src/book.rs`, `src/toc.rs`, `src/builder.rs`, `src/search.rs`

## バグ修正・改善 (2025-12-21)

- **サブディレクトリ構造の保持**: `src/a/b.md` が `docs/b.html` ではなく正しく `docs/a/b.html` に出力されるように修正
  - `src/book.rs`: `source_to_html_filename` 関数がディレクトリ構造を保持するように変更
  - `src/builder.rs`: 出力ファイルの親ディレクトリが存在しない場合に自動作成するように対応
  - `src/builder.rs`: TOCファイル名生成時にパスセパレータをアンダースコアに置き換えて一時ディレクトリへの書き込みエラーを修正
  - テストケースを追加してサブディレクトリパスの動作を検証

- **目次リンクを絶対パスに変更**: ナビゲーションの信頼性を向上
  - `src/toc.rs`: ページリンクとセクションリンクを相対パス (`page.html`) から絶対パス (`/page.html`) に変更
  - サブディレクトリ構造でもリンクが正しく機能するようになりました
  - テストケースを更新して絶対パスの動作を検証
