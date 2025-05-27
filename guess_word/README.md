# 概要

これは「動かして学ぶ！　Rust入門」の第4章1節の単語推理ゲームのサンプルコードです．
次のような構成になっています．

```text
guess_word/
  examples/     各中間コードがあるサンプルフォルダ
  src/          完成したソースコード
  Cargo.toml    マニフェストファイル
  Cargo.lock    バージョンファイル
  LICENSE       ライセンス
  README.md     このファイル
```

## サンプルコード

| 名前    | ページ番号 |
| ------- | ---------- |
| code100 | p.221      |
| code101 | p.223      |
| code102 | p.226      |
| code103 | p.227      |
| code104 | p.237      |
| code110 | p.244      |
| code111 | p.248      |
| code112 | p.250      |

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

## リポジトリ

最新版のサンプルは以下の場所にあります．また、問い合わせや不具合などの報告はGitHub上でも受け付けています．

- https://github.com/mebiusbox/rust-book
