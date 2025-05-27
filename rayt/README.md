# 概要

これは「動かして学ぶ！　Rust入門」の第4章2節のレイトレーシングプログラムのサンプルコードです．
次のような構成になっています．

```text
rayt/
  examples/     各中間コードがあるサンプルフォルダ
  resources/    画像テクスチャが格納されたフォルダ
  src/          完成したソースコード
  Cargo.toml    マニフェストファイル
  Cargo.lock    バージョンファイル
  LICENSE       ライセンス
  README.md     このファイル
```

## サンプルコード

| 名前    | ページ番号 |
| ------- | ---------- |
| code100 | p.257      |
| code101 | p.259      |
| code102 | p.281      |
| code103 | p.283      |
| code104 | p.286      |
| code105 | p.289      |
| code106 | p.292      |
| code107 | p.300      |
| code108 | p.302      |
| code109 | p.308      |
| code110 | p.309      |
| code111 | p.314      |
| code112 | p.316      |
| code113 | p.318      |
| code114 | p.322      |
| code115 | p.326      |
| code116 | p.329      |
| code117 | p.333      |
| code201 | p.339      |
| code202 | p.341      |
| code203 | p.346      |
| code204 | p.349      |
| code205 | p.354      |
| code206 | p.357      |
| code207 | p.360      |
| code208 | p.363      |
| code209 | p.366      |
| code301 | p.373      |
| code302 | p.381      |
| code303 | p.384      |
| code304 | p.385      |
| code305 | p.388      |
| code306 | p.391      |
| code307 | p.393      |
| code308 | p.398      |
| code309 | p.401      |
| code310 | p.408      |
| code311 | p.411      |

## 使い方

次のコマンドを実行してください．

```shell
cargo run --release
```

## 各コードの実行

紙面に記載されている `codeXXX` (XXXは番号) を指定して実行できます．
次のように実行します．

```shell
cargo run --release --example code{番号}
```

## 不具合

### リリースビルドでウィンドウが白い画面になる

[minifb の問題](https://github.com/emoon/rust_minifb/issues/351)のようです．最新版（0.28.0）でリリースビルドでも正常に描画されることを確認しました．
また、minifb 0.28.0 版で確認したところ、 `limit_update_rate` メソッドが非推奨になっていたため、 `set_target_fps` メソッドに置き換えました．

## リポジトリ

最新版のサンプルは以下の場所にあります．また、問い合わせや不具合などの報告はGitHub上でも受け付けています．

- https://github.com/mebiusbox/rust-book
