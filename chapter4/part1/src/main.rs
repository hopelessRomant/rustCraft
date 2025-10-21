fn main() {
    // Strings store value in Heap 
    let mut s = String::from("Hello"); // Strings can be mutated since they use heaps not stacks
    s.push_str(", world!!");
    println!("{s}");

    // double free error
    let s1 = String::from("sahil");
    let s2 = s1;
    // println!("{s1}"); --> `s1` has type `String`, which does not implement the `Copy` trait, it 'moves' the value
    println!("{s2}");

    // clone method
    let s1 = String::from("alice");
    let s2 = s1.clone(); // this copies the data in the heap to s2
    println!("s1 = {s1}, s2 = {s2}");

    // int types generally store values on stack
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    // Ownership and functions
    let s = String::from("home");
    new_owner(s);

    let s1 = give_ownership();
    println!("{s1}, is red");

    let s2 = String::from("Alice");
    let s3 = transfer_owner(s2);
    println!("{s3} and Bob");

    let (s,lenght) = calc_lenght(s3);
    println!("{lenght}, {s}");
}

fn new_owner(new_s: String){
    println!("{new_s}");
}

fn give_ownership() -> String{
    let new_string = String::from("apple");
    new_string
}

fn transfer_owner(old : String) -> String {
    let new =old;
    new
}

fn calc_lenght(s: String) -> (String,usize) {
    let lenght = s.len();
    (s, lenght)
}