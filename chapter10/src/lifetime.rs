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

// The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, 
// both of which are string slices that live at least as long as lifetime 'a. The function signature also tells Rust 
// that the string slice returned from the function will live at least as long as lifetime 'a. In practice, it means 
// that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of 
// the values referred to by the function arguments. These relationships are what we want Rust to use when analyzing this code.

fn longest_lft<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y.

pub fn scope_lft() {
    let s1 = String::from("Colder");
    let result;
    {
        let s2 = String::from("Hoter");
        result = longest_lft(s1.as_str(), s2.as_str());
        println!("The longet string is: {}", result);
    };
    // println!("{}",result); // this returns an error 
}