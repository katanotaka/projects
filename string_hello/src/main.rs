fn main() {
    let mut s1 = String::from("HELLO"); //ヒープを確保
    let mut s2 = s1;    //所有権が移動する コピーではない
                        //s1はもう使えない

//    s1.push_str(",world!!");
    s2.push_str(",world!!");

    let s3 = s2.clone();   //s2の内容をコピーしてs3に入れる。ヒープもコピーされる。

    //    println!("{s1}");
    println!("s2: {s2}");
    println!("s3: {s3}");

}   //スコープはおしまい。メモリは返還される
