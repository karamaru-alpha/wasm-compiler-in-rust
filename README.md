## wasm-compiler-in-rust

独自記法をwasmに変換するやつ。

対応記法は関数exportのみで、演算は四則演算のみ。


```sh
$ echo "fn add(a, b) { a + b }; fn multiple(a, b) { a * b };" > a.txt
$ cargo run
$ python3 -m http.server
```
