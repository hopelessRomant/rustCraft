fn main() {
    let mut s = String::from("Hello"); // Strings can be mutated since they use heaps not stacks
    s.push_str(", world!!");
    println!("{s}");
}
