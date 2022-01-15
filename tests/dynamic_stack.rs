use adv_datastructs::dynamic_stack::DynamicStack;

#[test]
fn push_pop() {
    let test_clos = |bound| {
        let mut stack = DynamicStack::new();
        for i in 0..bound {
            stack.push(i);
        }
        for i in (0..bound).rev() {
            assert_eq!(stack.pop().unwrap(), i);
        }
    };
    test_clos(10);
    test_clos(100);
    test_clos(1000);
}

#[test]
fn push_peek() {
    let test_clos = |bound| {
        let mut stack = DynamicStack::new();
        for i in 0..bound {
            stack.push(i);
            assert_eq!(stack.peek().unwrap(), i);
        }
    };
    test_clos(10);
    test_clos(100);
    test_clos(1000);
}

#[test]
fn display() {
    let mut stack = DynamicStack::new();
    for i in 0..10 {
        stack.push(i);
    }
    stack.display();

