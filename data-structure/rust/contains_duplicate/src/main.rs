#![allow(dead_code)]

enum TraversalOrder {
    InOrder,
    PreOrder,
    PostOrder,
}

struct Node<'a> {
    data: &'a i32,
    lnode: Option<Box<Node<'a>>>,
    rnode: Option<Box<Node<'a>>>,
}

struct BinaryTree<'a> {
    root_node: Option<Node<'a>>,
}

impl<'a> Node<'a> {
    fn new(val: &'a i32) -> Self {
        Node {
            data: val,
            lnode: None,
            rnode: None,
        }
    }
    fn insert(
        &mut self,
        val: &'a i32,
    ) {
        let target_node = if val > self.data {
            &mut self.rnode
        } else {
            &mut self.lnode
        };
        match target_node {
            &mut Some(ref mut next_node) => next_node.insert(val),
            &mut None => *target_node = Some(Box::new(Node::new(val))),
        }
    }

    fn to_print(&self) {
        print!("{} ", self.data);
    }

    fn in_order(&self) {
        match &self.lnode {
            Some(ref next_lnode) => {
                next_lnode.in_order();
            },
            _ => {},
        }
        self.to_print();
        match &self.rnode {
            Some(ref next_rnode) => {
                next_rnode.in_order();
            },
            _ => {},
        }
    }

    fn pre_order(&self) {
        self.to_print();
        match &self.lnode {
            Some(ref next_lnode) => {
                next_lnode.pre_order();
            },
            _ => {},
        }
        match &self.rnode {
            Some(ref next_rnode) => {
                next_rnode.pre_order();
            },
            _ => {},
        }
    }

    fn post_order(&self) {
        match &self.lnode {
            Some(ref next_lnode) => {
                next_lnode.post_order();
            },
            _ => {},
        }
        match &self.rnode {
            Some(ref next_rnode) => {
                next_rnode.post_order();
            },
            _ => {},
        }
        self.to_print();
    }
}

impl<'a> BinaryTree<'a> {
    pub fn new(val: &'a i32) -> Self {
        let root = Some(Node::new(val));
        return {
            BinaryTree {
                root_node: root,
            }
        };
    }

    fn insert(
        &mut self,
        val: &'a i32,
    ) {
        match &mut self.root_node {
            Some(ref mut node) => node.insert(val),
            _ => {},
        }
    }

    fn to_print(
        self,
        order: TraversalOrder,
    ) {
        match self.root_node {
            Some(ref node) => match order {
                TraversalOrder::InOrder => {
                    print!("in-order   : ");
                    node.in_order();
                    println!();
                },
                TraversalOrder::PreOrder => {
                    print!("pre-order  : ");
                    node.pre_order();
                    println!();
                },
                TraversalOrder::PostOrder => {
                    print!("post-order : ");
                    node.post_order();
                    println!();
                },
            },
            _ => println!("something went error"),
        }
    }
}

fn main() {
    let vec_arr = vec![15, 25, 30, 16, 10, 8, 12, 8, 20, 16, 25];
    let mut binary_tree = BinaryTree::new(&vec_arr[0]);
    for index in 1..vec_arr.len() {
        binary_tree.insert(&vec_arr[index]);
    }
    binary_tree.to_print(TraversalOrder::PostOrder);
}
