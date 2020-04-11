fn main() {
    while_sample();
    for_sample();
    enumerate_sample();
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
