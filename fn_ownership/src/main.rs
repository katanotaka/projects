fn main() {
    let s = String::from("hello");  // sがスコープに入る
    takes_ownership(s.clone());     // sの値が関数にムーブされ...
                                    // ... ここではもう有効ではない  (s.clone()を使うことで
    println!("s: {s}"); // ここでsを使おうとするとエラーになる        sは引き続き有効)

    let x = 5;                      // xがスコープに入る
    makes_copy(x);                  // xも関数にムーブされるが、
                                    // i32はCopyなので、この後にxを使っても
                                    // 大丈夫
    println!("x: {x}"); // ここでxを使うことができる
}

fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
    println!("{some_string}" );
}   // ここでsome_stringがスコープを抜ける。
    // 後ろ盾してたメモリが解放される。

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{some_integer}");
}   // ここでsome_integerがスコープを抜ける。
    // 何も特別なことはない。
