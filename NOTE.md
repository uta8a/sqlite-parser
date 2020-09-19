# SQLite Parser
- https://sqlite.org/index.html
- まずはSQLiteのdocumentationを見ていく
- [Database FileFormat](https://sqlite.org/fileformat2.html)
- `1.1. Hot Journals` ははじめは気にしなくてOK
- `1.3. The Database Header` を見る
- この最初の100byteをparseするのが目標
- Ubuntu20.04(on VMware in Windows10)で行った
- sqlite3をインストール
```
sqlite> .open data.sqlite
```
- fileが作られる
- 最初は空のファイルなので、書き込んでいく
```
sqlite> .open data.sqlite
sqlite> CREATE TABLE user (
   ...> id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
   ...> name TEXT,
   ...> email TEXT
   ...> );
sqlite> INSERT INTO user (name, email)
   ...> VALUES ('Robert Masen', 'r@robertmasen.com'),
   ...> ('Paul Bunyan', 'pb@example.com');
sqlite> select * from user;
1|Robert Masen|r@robertmasen.com
2|Paul Bunyan|pb@example.com

```
- sqliteはCで書かれているので、文字列はNULL終端
- cargo run
```
[83, 81, 76, 105, 116, 101, 32, 102, 111, 114, 109, 97, 116, 32, 51, 0]
```
- from_utf8_lossy
```
"SQLite format 3\u{0}"
```
- この3って何
  - SQLite since version 3.0.0 (2004-06-18). なのでVersionを指していると思う
- `The database page size in bytes`
  - pageって何
    - ???
```
[16, 0]
```
- `"multibyte fields" are "big-endian".` sqliteは複数byteがBig endianになっている
- u16::from_be_bytes BigEndian -> be
```
4096
```
- PageSizeは1が65536を指すという仕様なので、構造体でu32をwrapしてやる(u16に収まらない)
```
PageSize(4096)
```
- refactoring