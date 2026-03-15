fn main() {
    let s1 = gives_ownership();         // gives_ownershipは、戻り値をs1にムーブする
                                        // s1は関数からString::from("yours")を受け取る
    println!("{s1}");

    let s2 = String::from("hello");     // s2がスコープに入る
    let s3 = takes_and_gives_back(s2);  // s2はtakes_and_gives_backにムーブされ
    println!("{s3}");
  // 戻り値もs3にムーブされる
} // ここで、s3はスコープを抜け、ドロップされる。
  // s1もスコープを抜け、ドロップされる。

fn gives_ownership() -> String {             // gives_ownershipは、戻り値（String型）を
                                             // 呼び出した関数にムーブする
    let some_string = String::from("yours"); // String::from()とすることでsome_stringがスコープに入る
    some_string                              // some_stringが返され（return）、呼び出し元関数にムーブされる
}

// この関数は、Stringを一つ受け取り、返す。
fn takes_and_gives_back(mut a_string: String) -> String { // a_stringがスコープに入る。
    a_string.push_str(", world!!!");
    a_string  // a_stringが返され、呼び出し元関数にムーブされる
}
