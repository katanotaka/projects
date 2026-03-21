fn main() {
    let s = String::from("Hello, world!");
    println!("sentence = {s}");

    let (word, index) = first_word(&s);
    println!("1st word = {word} (index: {index})");
}

fn first_word(s: &String) -> (&str, usize) {             //&str（戻り値）：文字列の一部を指す「付箋」
    // 文字列をバイトの配列に変換。これにより、
    // 文字列を1バイトずつ処理できるようになる。
    let bytes = s.as_bytes();

    //空白を表すバイトを検索。空白が見つかったら、その位置を返す。
    //それ以外の場合、文字列の長さを返す。
    //文字列スライスとは、Stringの一部への参照であるため、
    //Stringの所有権を奪うことなく、文字列の一部を操作できる。
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return (&s[0..i], i);     //先頭から空白の位置までのスライスを返す  
        }
    }
    //空白が見つからなかった場合、文字列全体のスライスを返す
    return (&s[..], s.len());
}