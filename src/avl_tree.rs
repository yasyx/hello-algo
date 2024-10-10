use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
#[derive(Debug, Default)]
struct AvlTreeNode {
    val: i32,
    left: Option<Rc<RefCell<AvlTreeNode>>>,
    right: Option<Rc<RefCell<AvlTreeNode>>>,
    height: i32,
}

type OptionTreeNodeRc = Option<Rc<RefCell<AvlTreeNode>>>;


impl AvlTreeNode {
    fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            Self {
                val,
                height: 0,
                left: None,
                right: None,
            }
        ))
    }

    fn height(node: OptionTreeNodeRc) -> i32 {
        match node {
            Some(node) => node.borrow().height,
            None => -1
        }
    }

    fn update_height(node: OptionTreeNodeRc) {
        if let Some(node) = node {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            node.borrow_mut().height = max(Self::height(left), Self::height(right)) + 1;
        }
    }

    fn balance_factor(node: OptionTreeNodeRc) -> i32 {
        match node {
            None => 0,
            Some(n) => Self::height(n.borrow().left.clone()) - Self::height(n.borrow().right.clone())
        }
    }

    fn right_rotate(node: OptionTreeNodeRc) -> OptionTreeNodeRc {
        match node {
            None => None,
            Some(n) => {
                let child = n.borrow().left.clone().unwrap_or_default();
                let grand_child = child.borrow().right.clone();
                child.borrow_mut().right = Some(n.clone());
                n.borrow_mut().right = grand_child;

                Self::update_height(Some(n));
                Self::update_height(Some(child.clone()));

                Some(child)
            }
        }
    }

    fn left_rotate(node: OptionTreeNodeRc) -> OptionTreeNodeRc {
        match node {
            None => None,
            Some(n) => {
                let child = n.borrow().right.clone().unwrap_or_default();
                let grand_child = child.borrow().left.clone();

                child.borrow_mut().left = Some(n.clone());
                n.borrow_mut().right = grand_child;

                Self::update_height(Some(n));
                Self::update_height(Some(child.clone()));

                Some(child)
            }
        }
    }
    fn rotate(node: OptionTreeNodeRc) -> OptionTreeNodeRc {
        // 获取当前节点的平衡因子
        let balance_factor = Self::balance_factor(node.clone());
        // 左偏树
        if balance_factor > 1 {
            // 根据child节点的平衡因子判断是右旋还是先左旋再右旋
            let node = node.unwrap_or_default();
            // child子树再左侧 -> 右旋
            if Self::balance_factor(node.borrow().left.clone()) >= 0 {
                Self::right_rotate(Some(node))
            } else {
                let left = node.borrow().left.clone();
                node.borrow_mut().left = Self::left_rotate(left);
                Self::right_rotate(Some(node))
            }
            // 右偏树
        } else if balance_factor < -1 {
            // 根据child节点的平衡因子判断是左旋还是先右旋再左旋
            let node = node.unwrap_or_default();
            // 子树在右侧
            if Self::balance_factor(node.borrow().right.clone()) <= 0 {
                Self::left_rotate(Some(node))
            } else {
                let right = node.borrow().right.clone();
                node.borrow_mut().right = Self::right_rotate(right);
                Self::left_rotate(Some(node))
            }
        } else {
            node
        }
    }


}