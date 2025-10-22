fn main() {
    use std::io;

    println!("enter your scprit below");
    let mut script = String::new();
    io::stdin()
        .read_line(&mut script)
        .expect("failed to read line");
    
    let word = lead_word(&script);
    println!("first word of the script is '{word}'")
}
// type signifying string slice is called 'str'
fn lead_word(s: &String) -> &str { 
    // as_bytes method converts the String into array of byte.
    let bytes = s.as_bytes();

    // we create an iterator over the array of bytes using the iter method and eumerate() wraps the result is iter int (index, reff of byte):
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]

}

