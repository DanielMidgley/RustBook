
fn main() {
    let x = 5;
    let y = double(x);
    println!("Double {x} is {y}");
}

fn _hello_world() {
    println!("Hello");
}

fn double(x: i32) -> i32 {
    return x + x;
}
