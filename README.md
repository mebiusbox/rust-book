# 『動かして学ぶ！Rust入門 サンプルコード』

これは「動かして学ぶ！Rust入門」のサンプルコードです．
翔泳社のサポートサイトでもサンプルデータをダウンロードできます．
何か不具合や間違いなど見つけましたらIssueやメールなどでご連絡していただけると幸いです．

## 正誤表

### p.v

- 【誤】比較的優しい
- 【正】比較的易しい

### p.99

- 【誤】スライスは参照の1つで、別のオブジェクト内の連続した要素を指し示すものです．
- 【正】スライスは連続した要素を指し示すものです．

### p.99

- 【誤】スライスの型は参照なので、例えば配列だと &[T] 型となります．
- 【正】スライスは動的サイズ型と呼ばれるもので、例えばリスト3.86の場合、&[T]型となります．（動的サイズ型については第12節「トレイト(Trait)」で説明します）．

### p.104

- 【誤】このような仕組みを短絡評価またはショートサーキットといいます．
- （これは間違いです．無視してください．大変失礼しました）

### p.114

- 【誤】.演算子
- 【正】.（ドット）

### p.117

- 【誤】ヒープメモリはプログラム実行中に利用できるメモリで、スタックメモリよりも大きなサイズを利用できます．ただし、利用するにはオーバーヘッドがかかります．また、スタックメモリのサイズはヒープメモリに比べてかなり限られているので、ヒープメモリを積極的に利用することになります．
- 【正】ここもスタックメモリと同様にプログラム実行中に利用できるメモリですが、スタックメモリとは管理する方法が異なり、利用するにはオーバーヘッドがかかります．また、スタックメモリに格納する場合はそのサイズがわかっている必要があります．コンパイル時にサイズが不明だったり、動的にサイズが変わる場合はヒープメモリを利用することになります．


## 補足

### p.114 コンビネータ

コンビネータにおいて解説が不十分で誤解を招きやすい表現をしてしまったようです．

「リスト3.115の open.read.replace.write.close で考えてみると close(write(replace(read(open())))) の関係に見えないでしょうか．ここで .演算子がコンビネータであり、その役を担っているのでのが Option型や Result型と考えられます．」

ここで、`open.read.replace.write.close` というのは疑似的なコードであって Rust における .演算子のメソッド呼び出しのことを指しているわけではありません．ここで伝えたかったことはエラー伝搬（?演算子）によってわかりやすく書けるということであって、コンビネータとドット演算子によるメソッド呼び出しが同一であるということを示唆した訳ではありません．

誤解を招きやすい表現でした．この部分は読み飛ばしてもらって構いません．大変失礼いたしました．

## 構成

### guess_word

第4章1節「単語推理ゲーム」のサンプルプロジェクトです．

### rayt

第4章2節「レイトレーシング」のサンプルプロジェクトです．

## 購入サイト

- [SE Book (翔泳社)](https://www.shoeisha.co.jp/book/detail/9784798177236)
- [Amazon](https://www.amazon.co.jp/dp/B0BWR1T5QK)

## Notes

- 2025-05-27: 1.87.0 で動作確認済み
- 2023-12-12: 1.74.1 で動作確認済み
- 2023-04-24: 1.69 で動作確認済み
- 2023-04-16: 1.68 で動作確認済み
