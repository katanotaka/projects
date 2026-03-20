fn main() {
    let mut s1 = String::from("hello");
    let mut len = calculate_length(&mut s1);

    // '{}'の長さは、{}です
    println!("The length of '{s1}' is {len}." );
    
    change(&mut s1);
    len = calculate_length(&mut s1);
    
    println!("The length of '{s1}' is {len}" );
        
}

fn calculate_length(s: &mut String) -> usize {
    s.len()
}

fn change(s: &mut String){
    s.push_str(", world.");
}


//fn dangle() -> &String { // dangleはStringへの参照を返す
//    let s = String::from("hello"); // sは新しいString
//    &s // String sへの参照を返す
// ここで、sはスコープを抜け、ドロップされる。そのメモリは消される。
// 危険だ

