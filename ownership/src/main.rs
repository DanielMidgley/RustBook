fn main() {
    example7();
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

fn _example3() {
    // This works:
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1: {s1}, s2: {s2}");
    // clone() creates a copy of the string in the heap, at a different location
}

fn _example4() {
    // Like in example2, this does NOT work:
    /*
        let s = String::from("Hello");  // s enters scope
        some_function(s);               // s leave's scope -- some_function(s) takes ownership
        some_other_function(s); // error here
    
     */
}

fn _example5() {
    let s1 = String::from("Hello");     // s1 enters scope

    let s2 = example_function(s1);      // example_function takes ownership of s1,
                                        // and then copies s1 to s2 which is in scope
    println!("{s2}");                   // print works correctly

    fn example_function(s: String) -> String {
        // s enters scope here

        return s // s enters scope of calling function
    }
}

fn _example6() {
    let s = String::from("Hello");
    println!("{s}");
    println!("{s}");
}

// Borrowing
fn example7() {
    let s1 = String::from("Hello");

    let len = calc_length(&s1);

    println!("The length of {s1} is {len}");

    fn calc_length(s: &String) -> usize {
        s.len()
    }
    // &s1 is a reference to s1. This allows calc_length() to access s1 without taking ownership
}

fn _example8() {
    // This does not work:
    /* 
    let mut s = String::from("Hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{r1}, {r2}");
    */
    // You cannot have multiple mutable references to the same value

    // This does work:
    let mut s = String::from("Hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;

}
