fn main() {
    example3();
}

fn _example1() {
    let s1 = "Hello"; // string literal stored in stack
    println!("{s1}");

    let mut s2 = String::from("Hello"); // String type stored in heap
    println!("{s2}");
    s2 = "World".to_string();
    println!("{s2}");
    s2.push_str(", Hello!"); // String can be mutated
    println!("{s2}");
}

fn _example2() {
    // This works:
    let x = 5;
    let y = x;
    println!("x: {x}, y: {y}");

    // This does not:
        // let s1 = String::from("Hello");
        // let s2 = s1;
        // println!("s1: {s1}, s2: {s2}");
    // s1 is a pointer (stored in the stack) to the characters stored in the heap
    // let s2 = s1 copies that pointer to s2, so s2 then points to same location in heap
    // Can't have two pointers to the same data -- that would cause errors when freeing memory
    // So s1 is freed from memory during let s2 = s1
}

fn example3() {
    // This works:
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1: {s1}, s2: {s2}");
    // clone() creates a copy of the string in the heap, at a different location
}


