#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // 長方形の幅と高さを定義
    let width1 = 30;
    let height1 = 50;
    println!(
        // 面積は、{}平方ピクセルです
        "The area of the rectangle is {} square pixels.",
        area_solve(width1, height1)
    );

    // 長方形の幅と高さを定義
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_solve2(rect1)
    );

    // 長方形の幅と高さを定義   
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_solve3(&rect2)
    );


    // rect2は{}です
    // Debug整形を使用して値を出力する
    println!("rect2 is {:?}", rect2);
    println!("rect2 is {:#?}", rect2);


    let scale = 100;
    let rect4 = Rectangle {
        width: dbg!(30 * scale),
        height: 50 ,
    };


    println!(
        "The area of the rectangle is {} square pixels.",
        area_solve3(&rect4)
    );

    dbg!(&rect4);
}

fn area_solve(width: u32, height: u32) -> u32 {
    width * height
}


fn area_solve2(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

//これらの値に説明的な名前を与えられるため、コードの可読性が向上する
fn area_solve3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
