#[allow(dead_code)]
mod generic;

#[allow(dead_code)]
mod traits;

fn main() {
    let loc = generic::Point{x:10.0, y:15.0};
    println!("location is ({},{})",loc.x,loc.y);
    println!("mod of the point is: {}", loc.distance());
}
