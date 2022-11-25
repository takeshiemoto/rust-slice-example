# rust-slice-example

## スライス

コレクション全体ではなく、 その内の一連の要素を参照する。`[starting_index..ending_index]`と指定する。

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];


let s2 = String::from("hello");
let len = s2.len();

let slice = &s2[3..len];
let slice = &s2[3..];

```
