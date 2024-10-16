fn main() {
    let x = 3;
    let y = 4;
    println!(
        "The result of x+3 times y+5 is {}",
        times(add_3(x), add_5(y))
    );
}

fn add_3(x: i16) -> i16 {
    x + 3
}

fn add_5(x: i16) -> i16 {
    x + 5
}

fn times(x: i16, y: i16) -> i16 {
    x * y
}
