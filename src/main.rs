fn main() {
    print_number(5);
    print_sum(5, 6);
    println!("{}", add_one(2));

    let f: fn(i32) -> i32 = add_one;
    
    let six = f(5);
    println!("{}", six);

    // 配列
    let a = [1, 2, 3];
    let mut m = [1, 2, 3];
    let a = [0; 20];

    // 配列のスライス
    let a = [0, 1, 2, 3, 4];
    let complete = &a[..];
    let middle = &a[1..4];
    println!("{}", complete[0]);

    // タプル
    let mut x = (1, 2);
    let y = (2, 3);
    let (x, y, z) = (1, 2, 3);
    println!("x is {}", x);

    let tuple = (1, 2, 3);
    let x = tuple.0;
    println!("x is {}", x);
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn diverges() -> ! {
    panic!("This function never returns!");
}
