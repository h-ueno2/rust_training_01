fn main() {
    let mut x: i32 = 1;
    println!("{}", x);
    x = 7;
    let x = x;
    println!("{}", x);

    let y = 4;
    println!("{}", y);
    let y = "I can also be bound to text!";
    println!("{}", y);
}
