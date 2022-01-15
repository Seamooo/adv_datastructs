#[derive(Clone, Debug)]
struct DynamicStackNode<T> {
    val: T,
    next: Option<Box<DynamicStackNode<T>>>,
}

#[derive(Debug)]
pub struct DynamicStack<T> {
    head: Option<Box<DynamicStackNode<T>>>,
}

impl<T> DynamicStackNode<T> {
    // TODO restrict the visibility of this method to just DynamicStack
    pub fn new(val: T) -> Self {
        Self { val, next: None }
    }
}

impl<T> DynamicStack<T>
where
    T: Copy,
    T: std::fmt::Display,
{
    pub fn new() -> Self {
        Self { head: None }
    }
    pub fn push(&mut self, item: T) {
        let mut new_item = Box::new(DynamicStackNode::new(item));
        match self.head.take() {
            Some(x) => new_item.next = Some(x),
            _ => (),
        }
        self.head = Some(new_item);
    }
    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(mut x) => {
                self.head = x.next.take();
                Some(x.val)
            }
            None => None,
        }
    }
    pub fn peek(&self) -> Option<T> {
        match &self.head {
            Some(x) => Some(x.val),
            None => None,
        }
    }
    pub fn display(&self) {
        let mut node = &self.head;
        while let Some(x) = node {
            node = &x.next;
            print!("{},", x.val);
        }
        print!("\n");
    }
    pub fn is_empty(&self) -> bool {
        match self.head {
            None => true,
            _ => false,
        }
    }
}
