use adv_datastructs::b_tree::BTree;

#[test]
fn add_query() {
    let test_clos = |bound, order| {
        let mut set = BTree::new(order);
        for i in 0..bound {
            set.insert(i);
        }
        for i in 0..bound {
            assert_eq!(set.query(&i), true);
        }
        assert_eq!(set.query(&bound), false);
    };
    test_clos(10, 3);
    test_clos(100, 3);
    test_clos(1000, 3);
    test_clos(10, 10);
    test_clos(100, 10);
    test_clos(1000, 10);
}
