fn main() {
    let y = &5;
    let f = Foo { x: y };
    println!("{}", f.x());
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
