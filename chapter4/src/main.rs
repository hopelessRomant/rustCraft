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
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    // int types generally store values on stack
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    // Ownership and functions
    let s = String::from("home");
    new_owner(s);
}

fn new_owner(new_s: String){
    println!("{new_s}");
}