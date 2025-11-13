fn main() {
    let mut x: i32 = 5;
    println!("The value of x is: {x}");
    // New value can be assigned to x, because it is mutable
    x = 6; 
    println!("The value of x is: {x}");

    // constants require a type annotation
    const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // underscore at beginning of variable name supresses variable not used warning

    // Shadowing and scope
    let y = 5;
    let y = y + 1; // Note that we are changing y even though it is not mut!
    {
        let y = y * 2;
        println!("inner scope: {y}");
    }
    println!("outer scope: {y}");

    // String to Integer type conversion:
    let num: u32 = "42".parse().expect("Not a number");
    // Doesn't work without the type annotation
    println!("num: {num}");

    /*
    Types
        - Scalar Types
            - Integers (i8, u8, i16, ... i128, u128, isize, usize)
            - Floats (f32, f64)
            - Booleans (bool)
            - Characters (char)
        - Compound Types
            - Tuples
                Let a: (i32, u8, f32) = (23, 1, 4.5);
                let first = a.0;
            - Arrays e.g. [i32, 5] or [type, length]
                Let a: [i32, 5] = [1,2,3,4,5];
                Let first = a[0];
    */
    
}
