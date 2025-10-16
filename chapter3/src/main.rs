// Functions
fn main() {
    print_labeled_measurement(5, 'h');
    eg1();
    eg2();
    eg3();
    condition();
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// statements and expressions
//Statements are instructions that perform some action and do not return a value.
//Expressions evaluate to a resultant value.

fn eg1() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5 // no semicolon to declare it as a statement.
}

fn eg2() {
    let x = five();

    println!("The value of x is: {x}");
}

fn eg3() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// control flow --> conditional expressions
fn condition() {
    let number = 3;
//  if number {...} throws an error because rust always needs a bool in the conditional statement.
    if number != 0 {
        println!("number was something other than zero");
    }
}