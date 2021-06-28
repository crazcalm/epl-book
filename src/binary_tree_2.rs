use std::{
    borrow::BorrowMut,
    rc::{Rc, Weak},
};

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
    left: Option<Rc<BTree<T>>>,
    right: Option<Rc<BTree<T>>>,
}

type BinaryTree<T> = Option<Rc<BTree<T>>>;

#[derive(Debug)]
pub struct MyBinaryTree<T> {
    tree: BinaryTree<T>,
}

impl<T: Clone> MyBinaryTree<T> {
    pub fn new(value: T) -> Self {
        let tree = Some(Rc::new(BTree {
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

    pub fn insert_to_left(&mut self, value: T) -> Self {
        let mut tree = self.tree.clone().unwrap();
        let new_left = BTree {
            node: Node::new(value),
            left: tree.left.clone(),
            right: None,
        };

        let new_tree = Some(Rc::new(BTree {
            node: tree.clone().node.clone(),
            right: tree.clone().right.clone(),
            left: Some(Rc::new(new_left)),
        }));

        MyBinaryTree { tree: new_tree }
    }

    pub fn insert_to_right(&mut self, value: T) -> Self {
        let mut tree = self.tree.clone().unwrap();
        let new_right = BTree {
            node: Node::new(value),
            right: tree.right.clone(),
            left: None,
        };

        let new_tree = Some(Rc::new(BTree {
            node: tree.clone().node.clone(),
            left: tree.clone().left.clone(),
            right: Some(Rc::new(new_right)),
        }));

        MyBinaryTree { tree: new_tree }
    }

    pub fn move_to_left(&mut self) -> Self {
        MyBinaryTree {
            tree: self.tree.clone().unwrap().left.clone(),
        }
    }

    pub fn move_to_right(&mut self) -> Self {
        MyBinaryTree {
            tree: self.tree.clone().unwrap().right.clone(),
        }
    }
}

pub trait NodeBasicMovement2<T> {
    fn move_to_left(&mut self) -> Self;
    fn move_to_right(&mut self) -> Self;
    fn insert_to_left(&mut self, item: T) -> Self;
    fn insert_to_right(&mut self, item: T) -> Self;
}

impl<T: Clone> NodeBasicMovement2<T> for MyBinaryTree<T> {
    fn move_to_left(&mut self) -> Self {
        self.move_to_left()
    }

    fn move_to_right(&mut self) -> Self {
        self.move_to_right()
    }

    fn insert_to_left(&mut self, item: T) -> Self {
        self.insert_to_left(item)
    }

    fn insert_to_right(&mut self, item: T) -> Self {
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

        let mut tree_left = new_btree.insert_to_left("left".to_string());
        let mut tree_left_move = tree_left.move_to_left();

        assert_eq!(tree_left_move.value(), Some("left".to_string()));

        let mut tree_right = tree_left_move.insert_to_right("right".to_string());
        let mut tree_right_move = tree_right.move_to_right();

        assert_eq!(tree_right_move.value(), Some("right".to_string()));
    }
}
