fn main() {
    let s = String::from("Rust programming");
    let w = first_word(&s);

    println!("{}", w);

    // 文字列はスライス
    let s1 = "Hello world";
    // Stringのスライス
    let s2 = String::from("Hi");
    let s3 = &s2[..];

    let r1 = first_word_v3(s3);
    let r2 = first_word_v3(s1);

    println!("{}, {}", r1, r2)
}

// 文字列を受け取ってその文字列中の最初の単語を返す関数
fn first_word(s: &String) -> usize {
    // Stringオブジェクトをバイト配列に変換
    let bytes = s.as_bytes();

    // iterメソッドを使用してイテレータを生成
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_v2(s: &String) -> &str {
    // Stringオブジェクトをバイト配列に変換
    let bytes = s.as_bytes();

    // iterメソッドを使用してイテレータを生成
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    // 文字列全体のスライスを返す
    &s[..]
}

// &strを受け取って汎用性を上げる
fn first_word_v3(s: &str) -> &str {
    // Stringオブジェクトをバイト配列に変換
    let bytes = s.as_bytes();

    // iterメソッドを使用してイテレータを生成
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    // 文字列全体のスライスを返す
    &s[..]
}
