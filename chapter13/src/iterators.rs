pub fn inter() {
    let l1 = vec![1,3,5,7];
    let mut l1_itr = l1.iter(); // we need to make it mutable because .next() changes its state.

    assert_eq!(l1_itr.next(), Some(&1));
}