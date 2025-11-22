#[allow(dead_code)]
mod generic;

#[allow(dead_code)]
mod traits;

// use traits::{Summary, NewsAtricle};

fn main() {
    // --- generic ---
    // let loc = generic::Point{x:10.0, y:15.0};
    // println!("location is ({},{})",loc.x,loc.y);
    // println!("mod of the point is: {}", loc.distance());
    // loc.cmp();

    // --- traits ---
    // let news1 = NewsAtricle::data("World is going to end !!!".to_string());
    // println!("latest news of the day: {}\n presented by: {}", news1.headline(), news1.authors());

    // --- lifetime ---
    let r;
    {
        let x =3;
        r = &x;
        println!("{:#?}", r);
    }

}
