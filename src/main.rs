use std::sync::Arc;
use std::cell::Cell;

fn main() {
    let mut origin = Point { x: 0, y: 0};
    origin.x = 5;
    println!("The origin is at ({}, {})", origin.x, origin.y);

    // 同じ型であれば、値をコピーできる
    let mut point = Point3d { x:0, y:0, z:0 };
    point = Point3d { y: 1, .. point }; 

    let length = Inches(10);

    let Inches(integer_length) = length;
    println!("length is {} inches", integer_length);

}

// タプル構造体
struct Inches(i32);

// Unit-like構造体：何もメンバを持たない。
struct Electron;

struct Point {
    x: i32,
    // mut y: i32, //構造体の一部だけをミュータブルにはできない
    y: i32,
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
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
