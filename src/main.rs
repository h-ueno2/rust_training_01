fn main() {
    let x = 5;
    if x == 5 {
        println!("x is 5");
    } else {
        println!("x is not 5");
    }

    let y = if x == 5 {
        10
    } else {
        15
    };
    println!("y is {}", y);

    let y = if x == 5 { 10 } else { 15 };
    println!("y is {}", y);
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
