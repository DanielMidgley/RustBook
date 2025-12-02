use std::fs::File;

/* 
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

fn main() {
    let greeting_file_result = File::open("hello.txt");

    /* 
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };*/

    let greeting_file = match greeting_file_result {
        Ok(file) => fiel,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    // typically not good practice to use this
    let greeting_file = File::open("hello.txt").unwrap(); 

    let greeting_file = File::open("hello.txt")
        .expect("hello.txt not found");
}
