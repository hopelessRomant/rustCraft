#[allow(dead_code)]
mod boxt;

#[allow(dead_code)]
mod drop;

#[allow(dead_code)]
mod rct;

#[allow(dead_code)]
mod refcell;

#[allow(dead_code)]
mod refcycle;

#[allow(dead_code)]
mod tree;
fn main() {
    // boxt::basic();
    // boxt::cons_list();
    // boxt::coerc();

    // drop::drop_test();

    // rct::multiple_owner();

    // refcell::multi_mut_ref();

    // refcycle::overflow();

    tree::sample_tree();
}
