## wasm-compiler-in-rust

Only function export is supported.

```sh
$ echo "fn add(a, b) { a + b }; fn multiple(a, b) { a * b };" > a.txt
$ cargo run
$ python3 -m http.server
```


<img width="300" alt="スクリーンショット 2023-02-23 20 07 41" src="https://user-images.githubusercontent.com/38310693/222907563-e534e340-80b0-494e-8401-dbd3a7c65be3.png">


[自作言語をwasm変換するコンパイラをrustで作った](https://karamaru-alpha.com/posts/4/)
