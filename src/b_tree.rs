#[derive(Debug)]
struct BTreeNode<T>
where
    T: Ord,
    T: std::fmt::Debug,
{
    // TODO restrict access to BTree
    pub keys: Vec<T>,
    pub children: Vec<BTreeNode<T>>,
}

#[derive(Debug)]
pub struct BTree<T>
where
    T: Ord,
    T: std::fmt::Debug,
{
    root: Option<BTreeNode<T>>,
    max_order: usize,
}

impl<T> BTreeNode<T>
where
    T: Ord,
    T: std::fmt::Debug,
{
    pub fn new() -> Self {
        Self {
            keys: vec![],
            children: vec![],
        }
    }
    pub fn from_vecs(keys: Vec<T>, children: Vec<BTreeNode<T>>) -> Self {
        Self { keys, children }
    }
    pub fn search(&self, key: &T) -> bool {
        // debug assert
        assert!(self.keys.len() + 1 == self.children.len() || self.children.len() == 0);
        match self.keys.binary_search(key) {
            Ok(_) => true,
            Err(x) => {
                // if not leaf node, recurse
                if !self.is_leaf() {
                    self.children[x].search(key)
                } else {
                    false
                }
            }
        }
    }
    // WARNING
    // below function is a utility for insert
    // should not be used in isolation
    fn split_leaf_node(&mut self) -> (T, Self, Self) {
        assert!(self.is_leaf(), "must be leaf node to split_leaf_node");
        let rhs: Vec<T> = self.keys.drain((self.keys.len() / 2)..).collect();
        let mut lhs: Vec<T> = self.keys.drain(..).collect();
        let mid = lhs.pop().unwrap();
        (
            mid,
            BTreeNode::from_vecs(lhs, vec![]),
            BTreeNode::from_vecs(rhs, vec![]),
        )
    }
    // WARNING
    // below function is a utility function for insert
    // should not be used in isolation
    fn split_branch_node(&mut self) -> (T, Self, Self) {
        assert!(
            !self.is_leaf(),
            "cannot populate leaf node with child nodes"
        );
        assert!(self.keys.len() > 1, "cannot support order 1 nodes");
        let tp_mid = self.keys.len() / 2;
        let rhs_keys: Vec<T> = self.keys.drain(tp_mid..).collect();
        let rhs_children: Vec<Self> = self.children.drain(tp_mid..).collect();
        let mut lhs_keys: Vec<T> = self.keys.drain(..).collect();
        let lhs_children: Vec<Self> = self.children.drain(..).collect();
        // at this point lhs_keys and lhs_children have the same length
        // but this is solved with popping the key to promote
        let mid = lhs_keys.pop().unwrap();
        (
            mid,
            BTreeNode::from_vecs(lhs_keys, lhs_children),
            BTreeNode::from_vecs(rhs_keys, rhs_children),
        )
    }
    pub fn insert(&mut self, key: T, max_order: usize) -> Option<(T, Self, Self)> {
        match self.keys.binary_search(&key) {
            // found key hence no-op
            Ok(_) => None,
            Err(x) => {
                if self.is_leaf() {
                    self.keys.insert(x, key);
                    if self.keys.len() > max_order {
                        Some(self.split_leaf_node())
                    } else {
                        None
                    }
                } else {
                    match self.children[x].insert(key, max_order) {
                        Some((promoted, lhs, rhs)) => match self.keys.binary_search(&promoted) {
                            Ok(_) => unreachable!(),
                            Err(idx) => {
                                self.keys.insert(idx, promoted);
                                self.children[idx] = rhs;
                                self.children.insert(idx, lhs);
                                if self.keys.len() > max_order {
                                    Some(self.split_branch_node())
                                } else {
                                    None
                                }
                            }
                        },
                        None => None,
                    }
                }
            }
        }
    }
    pub fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }
}

impl<T> BTree<T>
where
    T: Ord,
    T: std::fmt::Debug,
{
    pub fn new(max_order: usize) -> Self {
        assert!(max_order >= 2, "order 2, minimum required");
        Self {
            root: None,
            max_order,
        }
    }
    pub fn insert(&mut self, key: T) {
        match &mut self.root {
            Some(node) => {
                match node.insert(key, self.max_order) {
                    Some((promoted, lhs, rhs)) => {
                        self.root = Some(BTreeNode::from_vecs(vec![promoted], vec![lhs, rhs]));
                    }
                    None => (),
                };
            }
            None => {
                let mut node = BTreeNode::new();
                node.keys.push(key);
                self.root = Some(node);
            }
        };
    }
    pub fn delete(&mut self, key: &T) {
        todo!();
    }
    pub fn query(&self, key: &T) -> bool {
        match &(self.root) {
            Some(x) => x.search(key),
            None => false,
        }
    }
}
