fn main() {
// references and borrowing
    let mut s1 = String::from("gravity");
    let num = calc_lenght(&s1);

    println!("size of {s1} is {num}");

// mutable reference --> there can only be one mutable reference to a value in a given scope
    change(&mut s1);
    println!("{s1}");

    {
        let r1 = &mut s1;
        r1.push_str(" -> first edit");
        println!("{r1}");
    }
    {
        let r2 = &mut s1;
        r2.push_str(" -> second edit");
        println!("{r2}");
    }
}

fn calc_lenght(s : &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" is changed");
}