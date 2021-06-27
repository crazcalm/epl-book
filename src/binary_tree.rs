use std::ops::DerefMut;

use crate::chapter_2_3::NodeBasicMovement;

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node { value }
    }
}

#[derive(Debug, Clone)]
struct BTree<T> {
    node: Node<T>,
    left: Option<Box<BTree<T>>>,
    right: Option<Box<BTree<T>>>,
}

type BinaryTree<T> = Option<Box<BTree<T>>>;

#[derive(Debug)]
pub struct MyBinaryTree<T> {
    tree: BinaryTree<T>,
}

impl<T: Clone> MyBinaryTree<T> {
    pub fn new(value: T) -> Self {
        let tree = Some(Box::new(BTree {
            node: Node::new(value),
            left: None,
            right: None,
        }));

        MyBinaryTree { tree }
    }

    pub fn value(&self) -> Option<T> {
        match self.tree.clone() {
            None => None,
            Some(tree) => Some(tree.node.value.clone()),
        }
    }

    pub fn insert_to_left(&mut self, value: T) {
        let mut tree = self.tree.clone().unwrap();
        let new_left = BTree {
            node: Node::new(value),
            left: tree.left.take(),
            right: None,
        };

        tree.left = Some(Box::new(new_left));

        self.tree = Some(tree);
    }

    pub fn insert_to_right(&mut self, value: T) {
        let mut tree = self.tree.clone().unwrap();
        let new_right = BTree {
            node: Node::new(value),
            right: tree.right.take(),
            left: None,
        };

        tree.right = Some(Box::new(new_right));

        self.tree = Some(tree);
    }

    pub fn move_to_left(&mut self) {
        self.tree = self.tree.clone().unwrap().left;
    }

    pub fn move_to_right(&mut self) {
        self.tree = self.tree.clone().unwrap().right;
    }
}

impl<T: Clone> NodeBasicMovement<T> for MyBinaryTree<T> {
    fn move_to_left(&mut self) {
        self.move_to_left()
    }

    fn move_to_right(&mut self) {
        self.move_to_right()
    }

    fn insert_to_left(&mut self, item: T) {
        self.insert_to_left(item)
    }

    fn insert_to_right(&mut self, item: T) {
        self.insert_to_right(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_basic_movements() {
        let mut new_btree = MyBinaryTree::new("head".to_string());

        assert_eq!(new_btree.value(), Some("head".to_string()));

        new_btree.insert_to_left("left".to_string());
        new_btree.move_to_left();

        assert_eq!(new_btree.value(), Some("left".to_string()));

        new_btree.insert_to_right("right".to_string());
        new_btree.move_to_right();

        assert_eq!(new_btree.value(), Some("right".to_string()));
    }
}
