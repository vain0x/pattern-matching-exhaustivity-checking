# パターンマッチ網羅性検査 アナライザー

パターンマッチの網羅性検査のサンプル実装、およびそのための自作言語の処理系です。

網羅性検査のサンプルは [match_exhaustivity ディレクトリ](./src/match_exhaustivity) に配置されていて、それ以外が処理系になります。

## 記述例

```rust
// enum 型
enum Boolean {
    False,
    True,
}

// match 式
match True {
    // バリアントパターン
    False => {}

    // 破棄パターン
    _ => {}
}
```

## 開発環境

以下のツールをインストールしてください。

- rustup/rustc/cargo (https://rustlang.org)
- [wasm-pack](https://github.com/rustwasm/wasm-pack)

ビルドは [dev-build](./dev-build) を参考にしてください。pmxc_playground で (ブラウザ上で) 動作させるために WebAssembly をターゲットにしています。

テストは `cargo test` です。一部のテストはスナップショットテストとなっていて、tests ディレクトリ以下にあるソースコード `*.pmxclang` の構文解析等の結果が `*_snapshot.txt` にダンプされます。出力結果の検証は目視確認です。
