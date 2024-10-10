mod binary_tree;
mod r#loop;
mod recur;
mod avl_tree;
mod sort;

use crate::binary_tree::{ArrayBinaryTree, binary_search_tree, build_perfect_tree, find_in_binary_search_tree, in_order, insert_in_binary_search_tree, insert_node, level_order, post_order, pre_order, remove_node, test_binary_tree};
use crate::r#loop::{for_loop, while_loop};
use crate::recur::{fib, for_loop_recur, recur, tail_recur};
use crate::sort::{bubble_sort, selection_sort};

fn main() {
    let n = 100;
    loop_n(n);
    recur_n(n);
    binary_tree();

    sorts();
}


fn sorts(){
    selection_sort(&mut [4, 5, 1, 2, 9]);
    bubble_sort(&mut [4, 5, 1, 2, 9])
}

fn binary_tree(){
    println!("test tree print: {:?}", test_binary_tree());
    println!("perfect tree print: {:?}", build_perfect_tree(5));
    println!("insert node: {:?} ",insert_node());
    println!("remove node: {:?} ",remove_node());
    let test_tree = test_binary_tree();
    println!("level order: {:?}", level_order(&test_tree));
    println!("pre order: {:?}", pre_order(Some(&test_tree)));
    println!("in order: {:?}", in_order(Some(&test_tree)));
    println!("post order: {:?}", post_order(Some(&test_tree)));
    let vec = vec![Some(1),Some(2),Some(3),Some(4),Some(5),Some(6),Some(7)];
    let array_binary_tree = ArrayBinaryTree::new(vec);
    println!("array binary tree level order: {:?}", array_binary_tree.level_order());
    println!("array binary tree pre order: {:?}", array_binary_tree.pre_order());
    println!("array binary tree in order: {:?}", array_binary_tree.in_order());
    println!("array binary tree post order: {:?}", array_binary_tree.post_order());
    let search_tree = binary_search_tree();
    println!("binary search tree find a node: {:?}", find_in_binary_search_tree(11,Some(search_tree.clone())));
    insert_in_binary_search_tree(-2,Some(search_tree.clone()));
    println!("binary search tree insert {:?}",search_tree)
}

fn loop_n( num : i32) {
    println!("sum of for_loop 100: {}", for_loop(num));
    println!("sum of while_loop 100: {}", while_loop(num));
}


fn recur_n(num : i32 ){
    println!("sum of recur 100: {}", recur(num));
    println!("sum of tail_recur 100 : {}", tail_recur(num, 0));
    println!("sum of fib 8 : {}", fib(8));
    println!("sum of for_loop_recur 100: {}", for_loop_recur(num));
}
