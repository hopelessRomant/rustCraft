#[allow(dead_code)]
mod part1;

fn main() {
    let loc = part1::Point{x:10.0, y:15.0};
    println!("location is ({},{})",loc.x,loc.y);
    println!("mod of the point is: {}", loc.distance());
}
