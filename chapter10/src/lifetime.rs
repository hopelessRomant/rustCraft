pub fn scope() {
    let r;
    {
        let x =3;
        r = &x;
        println!("{:#?}", r);
    }
    // println!{"{:#?}", r};
}

pub fn slices() {
    let s1 = "pirate".to_string();
    let s2 = "bay";

    // always a nice idea to use 'str' references in the function that can take both Strings and str
    let result = longest_lft(s1.as_str(), s2);
    println!("the longest string is: {}", result);
}

fn longest(s1: &str, s2:&str) -> String {
    if s1 >= s2 {
        s1.to_string()
    } else {
        s2.to_string()
    }
}

fn longest_lft<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}