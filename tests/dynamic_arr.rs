use adv_datastructs::dynamic_arr::DynamicArray;

#[test]
fn add_delete() {
    let test_clos = |bound| {
        let mut arr = DynamicArray::new();
        for i in 0..bound {
            arr.add(i);
        }
        for i in 0..bound {
            assert_eq!(arr.query(i).unwrap(), i)
        }
        for i in (0..bound).rev() {
            arr.add(i);
        }
        for i in 0..bound {
            assert_eq!(arr.delete().unwrap(), i);
        }
    };
    test_clos(10);
    test_clos(100);
    test_clos(1000);
}

#[test]
fn add_delete_at() {
    let test_clos = |bound| {
        let mut arr = DynamicArray::new();
        for i in 0..bound {
            arr.add_at_idx(i, i);
        }
        for i in 0..bound {
            assert_eq!(arr.query(i).unwrap(), i);
        }
        for i in 0..bound {
            arr.add_at_idx(0, i);
        }
        for i in 0..bound {
            assert_eq!(arr.query(i).unwrap(), bound - i - 1);
        }
        for i in 0..bound {
            assert_eq!(arr.delete_at_idx(0).unwrap(), bound - i - 1);
        }
        for i in (0..bound).rev() {
            assert_eq!(arr.delete_at_idx(i).unwrap(), i);
        }
    };
    test_clos(10);
    test_clos(100);
    test_clos(1000);
}
