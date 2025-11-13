fn main() {
    
}

fn _conditional_example_1() {
    let num: i32 = 10;

    if num < 5 {
        println!("{num} < 5");
    } else {
        println!("{num} >= 5");
    }
}

fn _conditional_example_2() {
    let num = if true {3} else {5};
    println!("{num}");
    
    let num = if false {3} else {5};
    println!("{num}");
}

fn _loops() {
    let mut i = 0;
    loop {
        if i > 9 {
            break;
        }
        println!("{i}");
        i = i + 1;
    }

    i = 0;
    while i < 10 {
        println!("{i}");
        i = i + 1;
    }

    for i in 1..10 {
        println!("{i}");
    }

}

fn _labeled_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
