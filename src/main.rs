
fn main() {
    let x = 5;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("somethig else"),
    }

    let number = match x {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        _ => "something else",
    };
    println!("{}", number);
}

