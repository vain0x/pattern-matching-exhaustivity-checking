# パターンマッチ網羅性検査

**状況**: 実装中

スペース (Space) の概念を用いたパターンマッチの網羅性検査のサンプルです。

## プロジェクト

- [アナライザー](./pmxc_analyzer)
    - 網羅性検査のサンプルのためのミニ言語の処理系
- [プレイグラウンド](./pmxc_playground)
    - アナライザーを試すためのウェブアプリ

## 参考

網羅性検査の実装は以下の記事を参考にしました。

- [A generic algorithm for checking exhaustivity of pattern matching](https://infoscience.epfl.ch/record/225497)
- [Swiftコンパイラで採用されているパターンマッチの網羅性チェックの理論と実装](https://qiita.com/ukitaka/items/7345e74116e11eb10f33)

プレイグランドは monaco-editor を使用していて、以下のサンプルを参考に構築しています。

- [microsoft/monaco-editor\: A browser based code editor](https://github.com/microsoft/monaco-editor)
    - https://github.com/microsoft/monaco-editor-samples/tree/master/browser-esm-webpack-typescript
