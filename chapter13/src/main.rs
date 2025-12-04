#[allow(dead_code)]
mod store;

use store::*;

fn main () {
    let _stock = Inventory::build(14, 13, 16);

}