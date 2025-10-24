#[allow(dead_code)]
struct Student {
    name: String,
    roll: u32,
    department: String,
    email: String,
}

fn main() {
    let s1 = Student {
        name: String::from("Sahil"),
        roll: 22174019,
        department: String::from("Physics"),
        email: String::from("sahil@something.com"),
    };
    let s2 = Student {
        roll: 22174018,
        ..s1 // note that the String types are moved from s1 to s2 so they no longer exist in s1 attributes.
    };

// changing roll
    // s1 = new_roll(2214018, s1);
    let roll = s2.roll;
    println!("{roll}")

// Tupple structs
    
}

#[allow(dead_code)]
fn new_roll(roll: u32, mut s: Student) -> Student {
    s.roll = roll;
    s
}