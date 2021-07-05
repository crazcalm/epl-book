pub trait NodeBasicMovement<T> {
    fn move_to_left(&mut self);
    fn move_to_right(&mut self);
    fn insert_to_left(&mut self, item: T);
    fn insert_to_right(&mut self, item: T);
}

#[derive(Debug)]
pub struct NodeInSequence<T> {
    pub current_node: T,
    pub left_list: Vec<T>,
    pub right_list: Vec<T>,
}

impl<T> NodeInSequence<T> {
    #[allow(dead_code)]
    pub fn sequence(item: T) -> Self {
        NodeInSequence {
            current_node: item,
            left_list: vec![],
            right_list: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn current_element(&self) -> Option<&T> {
        Some(&self.current_node)
    }
}

impl<T: Clone> NodeBasicMovement<T> for NodeInSequence<T> {
    fn move_to_left(&mut self) {
        let temp = self.current_node.clone();
        self.right_list.insert(0, temp);

        self.current_node = self.left_list.remove(0);
    }

    fn move_to_right(&mut self) {
        let temp = self.current_node.clone();
        self.left_list.insert(0, temp);

        self.current_node = self.right_list.remove(0);
    }

    fn insert_to_left(&mut self, item: T) {
        self.left_list.insert(0, item)
    }

    fn insert_to_right(&mut self, item: T) {
        self.right_list.insert(0, item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_basic_movements() {
        let mut node = NodeInSequence {
            current_node: 6,
            left_list: vec![5, 4, 3, 2, 1],
            right_list: vec![7, 8, 9],
        };

        assert_eq!(node.current_element(), Some(&6));

        node.move_to_left();
        assert_eq!(node.current_element(), Some(&5));
        assert_eq!(node.left_list, vec![4, 3, 2, 1]);
        assert_eq!(node.right_list, vec![6, 7, 8, 9]);

        node.move_to_right(); // moving back to original formation
        node.move_to_right();
        assert_eq!(node.current_element(), Some(&7));
        assert_eq!(node.left_list, vec![6, 5, 4, 3, 2, 1]);
        assert_eq!(node.right_list, vec![8, 9]);

        node.move_to_left(); //moving back to original formation
        node.insert_to_left(13);
        assert_eq!(node.current_element(), Some(&6));
        assert_eq!(node.right_list, vec![7, 8, 9]);
        assert_eq!(node.left_list, vec![13, 5, 4, 3, 2, 1]);

        node.insert_to_right(13);
        assert_eq!(node.current_element(), Some(&6));
        assert_eq!(node.right_list, vec![13, 7, 8, 9]);
        assert_eq!(node.left_list, vec![13, 5, 4, 3, 2, 1]);
    }
}
