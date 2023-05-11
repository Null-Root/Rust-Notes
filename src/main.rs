use std::io; // import

// I'm just adding these files to let the rust-analyzer and cargo analyze these shits
mod enums_file;
mod structs_file;
mod vectors_file;

// consts
const global_c: &str = "sasda";

// entry point
fn main() {
    println!("Guess the number!"); // normal print stmt

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    let mut a = 2;
    let b = 4;
    
    // mutable vars can have reassignment and mutations
    a = b;

    let heart_eyed_cat = '😻';

    println!("{} {}", a, b);

    println!("{}", heart_eyed_cat);

    // this is a tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    print!("{} {} {}", tup.0, tup.1, tup.2);

    // destructure tuple (similar to destructuring on typescript)
    let ( d, e, f ) = tup;

    // this is an array
    let a = [1, 2, 3, 4, 5];

    println!("{} {}", a[0], a[4]);

    // functions
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    // can directly call function inside println
    println!("{}", func_with_val());
}

fn func_with_val() -> i32 {
    return 5; // return keyword can be removed, just placed there for clarity
}