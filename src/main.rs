use std::sync::Arc;
use std::cell::Cell;

fn main() {
    let x = Arc::new(5);
    let y = x.clone();

    // 構造体自体をミュータブルにする。
    let mut a = Point { x: 5, y: 6 };
    a.x = 10;

    let b = Point2 { x: 5, y: Cell::new(6) };
    b.y.set(7);
    println!("y: {:?}",  b.y);

    let y = &5;
    let f = Foo { x: y };
    println!("{}", f.x());
}

struct Point {
    x: i32,
    // mut y: i32, //構造体の一部だけをミュータブルにはできない
    y: i32,
}

struct Point2 {
    x: i32,
    y: Cell<i32>, // Cell<T>でフィールド単位でミュータブルできる
}

fn hoge(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    42
}

struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 { self.x }
}

fn bar<'a>(x: &'a i32) {

}

fn foo() {
    // let v = vec![1, 2, 3];
    // let v2 = v;
    // 所有権が移動しているため以下でコンパイルエラー
    // println!("v[0] is:{}", v[0]);

    let v = 1;
    let v2 = v;
    // プリミティブ型はCopyトレイトが実装されているためコンパイルできる
    println!("v is: {}", v);

    let a = 5;
    let _y = double(a);
    println!("{}", a);

    let a = true;
    let _y = change_truth(a);
    println!("{}", a);
}

fn double(x: i32) -> i32 {
    x * 2
}

fn change_truth(x: bool) -> bool {
    !x
}

fn while_sample() {
    println!("while sample");
    let mut x = 5;
    let mut done = false;
    while !done {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 {
            done = true;
        }
    }
}

fn for_sample() {
    println!("for sample");
    for x in 0..10 {
        println!("{}", x);
    }
}

fn enumerate_sample() {
    println!("enumerate sample");
    for (i, j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);
    }
}
