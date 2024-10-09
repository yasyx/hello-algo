use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug)]
pub struct ArrayBinaryTree {
    tree: Vec<Option<i32>>,
}


impl ArrayBinaryTree {
    pub fn new(tree: Vec<Option<i32>>) -> Self {
        Self {
            tree
        }
    }
    fn size(&self) -> i32 {
        self.tree.len() as i32
    }

    fn val(&self, i: i32) -> Option<i32> {
        if i < 0 || i > (self.tree.len() - 1) as i32 {
            return None;
        }

        self.tree[i as usize]
    }

    fn left(&self, i: i32) -> i32 {
        2 * i + 1
    }

    fn right(&self, i: i32) -> i32 {
        2 * i + 2
    }


    fn parent(&self, i: i32) -> i32 {
        (i - 1) / 2
    }

    pub fn level_order(&self) -> Vec<i32> {
        let mut vec = vec![];
        for i in 0..self.size() {
            if let Some(val) = self.val(i) {
                vec.push(val)
            }
        }
        vec
    }


    /* 深度优先 */
    fn dfs(&self, i: i32, order: &str) -> Vec<i32> {
        let mut res = vec![];
        if self.val(i).is_none() {
            return res;
        }
        let val = self.val(i).unwrap();

        if order == "pre" {
            res.push(val);
        }
        // 遍历左子树
        res.extend(self.dfs(self.left(i), order));
        if order == "in" {
            res.push(val);
        }

        // 遍历右子树
        res.extend(self.dfs(self.right(i), order));
        if order == "post" {
            res.push(val);
        }
        res
    }

    pub fn pre_order(&self) -> Vec<i32> {
        self.dfs(0, "pre")
    }

    pub fn in_order(&self) -> Vec<i32> {
        self.dfs(0, "in")
    }
    pub fn post_order(&self) -> Vec<i32> {
        self.dfs(0, "post")
    }
}


#[derive(Debug)]
pub struct TreeNode<T> {
    val: T,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T> {
    fn new(val: T) -> Rc<RefCell<Self>> {
        Rc::new(
            RefCell::new(
                Self {
                    val,
                    left: None,
                    right: None,
                }
            )
        )
    }
}

pub fn post_order(root: Option<&Rc<RefCell<TreeNode<i32>>>>) -> Vec<i32> {
    let mut res = vec![];

    if let Some(n) = root {
        res.extend(post_order(n.borrow().left.as_ref()));
        res.extend(post_order(n.borrow().right.as_ref()));
        res.push(n.borrow().val)
    }
    res
}

pub fn in_order(root: Option<&Rc<RefCell<TreeNode<i32>>>>) -> Vec<i32> {
    let mut res = vec![];

    if let Some(node) = root {
        res.extend(in_order(node.borrow().left.as_ref()));
        res.push(node.borrow().val);
        res.extend(in_order(node.borrow().right.as_ref()));
    }
    res
}


pub fn pre_order(root: Option<&Rc<RefCell<TreeNode<i32>>>>) -> Vec<i32> {
    let mut res = vec![];

    if let Some(node) = root {
        res.push(node.borrow().val);
        res.extend(pre_order(node.borrow().left.as_ref()));
        res.extend(pre_order(node.borrow().right.as_ref()));
    }

    res
}

pub fn level_order(root: &Rc<RefCell<TreeNode<i32>>>) -> Vec<i32> {
    let mut queue = VecDeque::new();

    queue.push_back(root.clone());

    let mut vec = Vec::new();

    while let Some(node) = queue.pop_front() {
        vec.push(node.borrow().val);
        if let Some(left) = node.borrow().left.as_ref() {
            queue.push_back(left.clone());
        }

        if let Some(right) = node.borrow().right.as_ref() {
            queue.push_back(right.clone());
        }
    }

    vec
}

pub fn remove_node() -> Option<Rc<RefCell<TreeNode<i32>>>> {
    let root = test_binary_tree();

    let left = root.borrow_mut().left.clone();


    let left_left = left.unwrap().borrow_mut().left.clone();


    root.borrow_mut().left = left_left;

    Some(root)
}
pub fn insert_node() -> Option<Rc<RefCell<TreeNode<i32>>>> {
    let root = test_binary_tree();

    let node2 = root.borrow_mut().left.clone();

    let p = TreeNode::new(0);

    root.borrow_mut().left = Some(p.clone());

    p.borrow_mut().left = node2;

    Some(root)
}

pub fn build_perfect_tree(n: i32) -> Option<Rc<RefCell<TreeNode<i32>>>> {
    if n == 0 {
        return None;
    }

    let root = TreeNode::new(0);

    root.borrow_mut().left = build_perfect_tree(n - 1);
    root.borrow_mut().right = build_perfect_tree(n - 1);

    Some(root)
}

pub fn test_binary_tree() -> Rc<RefCell<TreeNode<i32>>> {
    let node1 = TreeNode::new(1);
    let node2 = TreeNode::new(2);
    let node3 = TreeNode::new(3);
    let node4 = TreeNode::new(4);
    let node5 = TreeNode::new(5);
    let node6 = TreeNode::new(6);
    let node7 = TreeNode::new(7);


    node1.borrow_mut().left = Some(node2.clone());
    node1.borrow_mut().right = Some(node3.clone());

    node2.borrow_mut().left = Some(node4);
    node2.borrow_mut().right = Some(node5);

    node3.borrow_mut().left = Some(node6);
    node3.borrow_mut().right = Some(node7);
    node1
}


pub fn find_in_binary_search_tree(num: i32, root: Option<Rc<RefCell<TreeNode<i32>>>>) -> Option<Rc<RefCell<TreeNode<i32>>>> {
    let mut cur = root.clone();
    while let Some(n) = cur.clone() {
        match num.cmp(&n.borrow().val) {
            Ordering::Greater => cur = n.borrow().right.clone(),
            Ordering::Less => cur = n.borrow().left.clone(),
            Ordering::Equal => break,
        }
    }
    cur
}

pub fn insert_in_binary_search_tree(num: i32, root: Option<Rc<RefCell<TreeNode<i32>>>>) {
    let mut cur = root.clone();
    let mut pre = None;
    let mut less = false;
    while let Some(n) = cur.clone() {
        match num.cmp(&n.borrow().val) {
            Ordering::Less => {
                less = true;
                pre = cur;
                cur = n.borrow().left.clone();
            }
            Ordering::Equal => (),
            Ordering::Greater => {
                pre = cur;
                cur = n.borrow().right.clone();
            }
        }
    }

    if let Some(n) = pre {
        let new_node = TreeNode::new(num);
        match less {
            true => n.borrow_mut().left = Some(new_node),
            false => n.borrow_mut().right = Some(new_node),
        }
    }
}


pub fn binary_search_tree() -> Rc<RefCell<TreeNode<i32>>> {
    let root = TreeNode::new(8);

    let n1 = TreeNode::new(4);
    let n2 = TreeNode::new(12);
    let n3 = TreeNode::new(2);
    let n4 = TreeNode::new(6);
    let n5 = TreeNode::new(10);
    let n6 = TreeNode::new(14);

    let n7 = TreeNode::new(1);
    let n8 = TreeNode::new(3);
    let n9 = TreeNode::new(5);
    let n10 = TreeNode::new(7);
    let n11 = TreeNode::new(9);
    let n12 = TreeNode::new(11);
    let n13 = TreeNode::new(13);
    let n14 = TreeNode::new(15);


    root.borrow_mut().left = Some(n1.clone());
    root.borrow_mut().right = Some(n2.clone());

    n1.borrow_mut().left = Some(n3.clone());
    n1.borrow_mut().right = Some(n4.clone());

    n2.borrow_mut().left = Some(n5.clone());
    n2.borrow_mut().right = Some(n6.clone());

    n3.borrow_mut().left = Some(n7);
    n3.borrow_mut().right = Some(n8);

    n4.borrow_mut().left = Some(n9);
    n4.borrow_mut().right = Some(n10);

    n5.borrow_mut().left = Some(n11);
    n5.borrow_mut().right = Some(n12);

    n6.borrow_mut().left = Some(n13);
    n6.borrow_mut().right = Some(n14);


    root
}