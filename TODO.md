## DONE

- [x] toc のリンクを絶対パスに変更 (2025-12-21)
- [x] toc において chapter より上の階層 (part) を表現する (2025-12-21)
    - path を持たない pages エントリを part として扱う
    - part の見出しを toc 上で表示（リンクなし、太字、uppercase）
    - ページは生成されない
- [x] table の見た目を改善 (2025-12-21)
    - 罫線を追加
    - ヘッダー行/偶数行/奇数行で背景色を変更
    - color theme に応じて色を変更
- [x] メニュー (toc) の開閉ボタンを追加 (2025-12-21)
    - theme-switcher の左隣に配置
        - いまある theme-switcher を右にずらす
    - メニューの開閉状態に依らず表示される
    - デフォルトでは開いている状態
- [x] `<code>` にはワンクリックでコピーできるボタンを追加する (2025-12-22)
    - クリップボードにコピーする機能を実装
    - コピー成功/失敗のフィードバックを表示
    - ホバー時にコピーボタンが表示される
    - モバイルでは常時表示（透明度を下げて表示）
- [x] serve や watch で差分ビルドを実装 (2025-12-22)
    - 変更があったページだけビルドする
    - book.tomlが変更された場合はフルビルド
    - buildコマンドは常にフルビルド
- [x] Prism ハイライトがタイミングによって正しく動作しない問題を修正 (2025-12-22)
    - window load イベントの100ms後にPrism.highlightAll()を再実行
    - すべてのリソース（スクリプトを含む）がロードされた後に実行される

## DONE

- [x] base_path を book.toml で設定できるようにする (2025-12-22)
    - base_path = '/gnuplot-book' としたらリンク `a/b/c` が `/gnuplot-book/a/b/c` になる
    - base_pathが `/` で始まっていない場合は自動で追加
    - base_pathが `/` で終わっていない場合は自動で追加
    - 空文字列の場合は従来通り `/` から始まるパスになる

## DONE

- [x] unidoc で失敗したら stderr をログに出力するようにする (2025-12-22)
    - exit code, stderr, stdout を eprintln! で出力
    - エラーメッセージをより明確に表示

## DONE

- [x] BookItem を BookPage にする (2025-12-23)
    - `[[pages]]` を `[[items]]` に変更
    - BookItem の構造を変更: enum から struct に
        - `title: String`, `level: ItemLevel`, `page: Option<PageInfo>`
    - PageInfo から title を削除（BookItem側に移動）
    - ItemLevel enum を追加: Page, Part, Chapter, Section, Subsection
    - デフォルトは Page
    - path は完全にオプショナル（すべてのレベルで指定可能/不可能）
    - TOC の CSS を階層ごとに調整
        - Part, Chapter, Section, Subsection: 背景色反転（color: var(--bg-primary), background: var(--text-primary)）
        - 不透明度で階層を表現: Part(100%) > Chapter(80%) > Section(60%) > Subsection(40%)
        - Page: 通常の表示（背景色なし）
    - すべてのテストとclippyが合格

## TODO


## @CLAUDE

- ひとつ実装が完了したらドキュメントを更新する
    - Update TODO.md, CHANGELOG.md, README.md
