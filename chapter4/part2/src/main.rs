fn main() {
// references and borrowing
    let s1 = String::from("gravity");
    let num = calc_lenght(&s1);

    println!("size of {s1} is {num}");
}

fn calc_lenght(s : &String) -> usize {
    s.len()
}
