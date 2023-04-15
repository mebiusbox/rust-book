# 概要

これは「動かして学ぶ！　Rust入門」の第4章2節のレイトレーシングプログラムのサンプルコードです．
次のような構成になっています．

```
rayt/
  examples/     各中間コードがあるサンプルフォルダ
  resources/    画像テクスチャが格納されたフォルダ
  src/          完成したソースコード
  Cargo.toml    マニフェストファイル
  Cargo.lock    バージョンファイル
  LICENSE       ライセンス
  README.md     このファイル
```


## 使い方

次のコマンドを実行してください．

```
cargo run --release
```

## 各コードの実行

紙面に記載されている `codeXXX` (XXXは番号) を指定して実行できます．
次のように実行します．

```
cargo run --release --example code{番号}
```


## リポジトリ

最新版のサンプルは以下の場所にあります．また、問い合わせや不具合などの報告はGitHub上でも受け付けています．

- https://github.com/mebiusbox/rust-book

