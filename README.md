# パターンマッチ網羅性検査

スペース (Space) の概念を用いたパターンマッチの網羅性検査のサンプル実装です。

解説記事を書きました:

- [スペースによるパターンマッチの網羅性検査](https://qiita.com/vain0x/items/47bdd322d5de05c2db0b)

ブラウザ上で動作を確認できます:

- [パターンマッチ網羅性検査 プレイグラウンド](https://vain0x.github.io/pattern-matching-exhaustivity-checking/index.html)

## プロジェクト

- [アナライザー](./pmxc_analyzer)
    - 網羅性検査のサンプルのためのミニ言語の処理系
- [プレイグラウンド](./pmxc_playground)
    - アナライザーを試すためのウェブアプリ

## 参考

網羅性検査の実装は以下の記事を参考にしました。

- [A generic algorithm for checking exhaustivity of pattern matching](https://infoscience.epfl.ch/record/225497)
- [Swiftコンパイラで採用されているパターンマッチの網羅性チェックの理論と実装](https://qiita.com/ukitaka/items/7345e74116e11eb10f33)

プレイグラウンドは monaco-editor を使用していて、以下のサンプルを参考に構築しています。

- [microsoft/monaco-editor\: A browser based code editor](https://github.com/microsoft/monaco-editor)
    - https://github.com/microsoft/monaco-editor-samples/tree/master/browser-esm-webpack-typescript
