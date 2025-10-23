#[allow(dead_code)]
struct Student {
    name: String,
    roll: u32,
    department: String,
    email: String,
}

fn main() {
    let mut s1 = Student {
        name: String::from("Sahil"),
        roll: 22174019,
        department: String::from("Physics"),
        email: String::from("sahil@something.com"),
    };
    s1 = Student {
        roll: 22174018,
        ..s1
    };
// changing roll
    // s1 = new_roll(2214018, s1);
    let roll = s1.roll;
    println!("{roll}")


}

#[allow(dead_code)]
fn new_roll(roll: u32, mut s: Student) -> Student {
    s.roll = roll;
    s
}