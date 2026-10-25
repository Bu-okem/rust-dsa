struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

struct BST {
    root: Option<Box<Node>>,
}

impl BST {
    fn insert_recursive(node: &mut Option<Box<Node>>, value: i32) {
        match node {
            Some(n) => {
                if n.value > value {
                    BST::insert_recursive(&mut n.left, value);
                } else if n.value < value {
                    BST::insert_recursive(&mut n.right, value);
                }
            }
            None => {
                *node = Some(Box::new(Node {
                    value,
                    left: None,
                    right: None,
                }));
            }
        }
    }

    fn insert(&mut self, value: i32) {
        BST::insert_recursive(&mut self.root, value);
    }

    fn inorder_traversal_recursive(node: &Option<Box<Node>>, arr: &mut Vec<i32>) {
        if let Some(n) = node {
            BST::inorder_traversal_recursive(&n.left, arr);
            arr.push(n.value);
            BST::inorder_traversal_recursive(&n.right, arr);
        } else {
            return;
        }
    }

    fn inorder_traversal(&self) -> Vec<i32> {
        let mut order = Vec::new();

        BST::inorder_traversal_recursive(&self.root, &mut order);

        order
    }
}
