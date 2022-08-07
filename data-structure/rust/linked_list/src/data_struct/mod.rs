
#[derive(Debug)]
pub struct Node {
    data: Option<u32>,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(data: u32) -> Self {
        Node {
            data: Some(data),
            next: None,
        }
    }

    pub fn insert(
        &mut self,
        data: u32,
    ) {
        match &mut self.next {
            Some(n) => n.insert(data),
            _ => {
                let new_node = Node::new(data);
                self.next = Some(Box::new(new_node));
            },
        }
    }

    pub fn print_whole_list(&self) {
        match self.data {
            Some(ref n) => {
                print!("{} ", n);
            },
            _ => {},
        }
        match &self.next {
            Some(next_node) => {
                next_node.print_whole_list();
            },
            _ => {
                println!("\nhit the end of the linked list");
            },
        }
    }
}
    
