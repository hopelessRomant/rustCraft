// Functions
fn main() {
    print_labeled_measurement(5, 'h');
    eg1();
    eg2();
    eg3();
    condition();
    arms();
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// statements and expressions
//Statements are instructions that perform some action and do not return a value.
//Expressions evaluate to a resultant value.

fn eg1() {
    let y: i32 = {
        let x: i32 = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5 // no semicolon, to declare it as a expression -> return value
}

fn eg2() {
    let x: i32 = five();

    println!("The value of x is: {x}");
}

fn eg3() {
    let x: i32 = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 // no semicolon, to declare it as a expression -> return value
}

// control flow --> conditional 'expressions'
fn condition() {
    let number = 2;
//  'if number {...}' throws an error because rust always needs a bool in the conditional statement.
    if number < 5 {
        println!("less than 5");
    } 
    else if number % 2 == 0 {
        println!("even number");
    }
    else{
        println!("condition false")
    }
}

//arms of if:
fn arms() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}