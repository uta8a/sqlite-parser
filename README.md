# sqlite-parser
- [SQLite File Parser Pt. 1: The Header](https://wiredforge.com/blog/sqlite-parser-pt-1/index.html) をやりました。
- 内容としては、仕様を見ながら、SQLiteのヘッダの一部をスライスで見てみた！という感じ。

## step
- sqliteの仕様
- sqlite file作成
- Magic Header Stringの読み込み
  - "SQLite format 3\u{0}"
- PageSizeの読み込み
  - マルチバイトはBigEndian, 1は65536を指すなどの仕様
- ファイル分割

## 修正箇所
- ファイル分割後でmain.rsが微妙にエラー出る
  - mainがResultを返していない→Ok(())を返すようにした
  - use が `validate_magic_string` はtypoで `validate_header_string`が正しい

## ここまでの感想
- Rustはじめての人向けかな、説明が丁寧
- バイト列をスライスで切ってみていくだけなので(一度似たことをした人にとっては)面白くないかと思ったが、sqliteの仕様が面白くて楽しい。
- 続きが楽しみ

## 残った課題や疑問
- SQLiteのPageがしっくり理解できていない