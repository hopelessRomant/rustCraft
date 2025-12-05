#[cfg(test)]
mod test {
    #[test]
    fn iter() {
        let l1 = vec![1,3,5,7];
        let mut l1_itr = l1.iter(); // we need to make it mutable because .next() changes its state.

        assert_eq!(l1_itr.next(), Some(&1));
        assert_eq!(l1_itr.next(), Some(&3));
        assert_eq!(l1_itr.next(), Some(&5));
        assert_eq!(l1_itr.next(), Some(&7));
        assert_eq!(l1_itr.next(), None);
    }

    #[test]
    fn sum() {
        let v1 = vec![2,3,5,7];
        let v1_itr = v1.iter();

        let sum: i32 = v1_itr.sum(); // takes ownerhship of the iter 'consumes the iter'
        // let test = v1_itr.next();
        assert_eq!(sum, 17);
    }
}
