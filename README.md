## wasm-compiler-in-rust

Only function export is supported.

```sh
$ echo "fn add(a, b) { a + b }; fn multiple(a, b) { a * b };" > a.txt
$ cargo run
$ python3 -m http.server
```

<img width="148" alt="スクリーンショット 2023-02-23 20 07 41" src="https://user-images.githubusercontent.com/38310693/222907482-6da74d13-c7f1-46b1-9983-526e029dd7b8.png">
