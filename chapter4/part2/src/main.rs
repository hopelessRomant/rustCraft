fn main() {
// references and borrowing
    let mut s1 = String::from("gravity");
    let num = calc_lenght(&s1);

    println!("size of {s1} is {num}");

// mutable reference
    change(&mut s1);
    println!("{s1}");
}

fn calc_lenght(s : &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" is changed");
}