use std::fmt::Debug;
use std::marker::Copy;

#[derive(Debug, Clone)]
pub struct Stack<T> {
    data: Option<T>,
    next: Option<Box<Stack<T>>>,
}

impl<T: Debug> Stack<T> {
    pub fn new(data: Option<T>) -> Self {
        let mut stack: Stack<T> = Stack {
            data: None,
            next: None,
        };
        match data {
            Some(_n) => {
                stack.data = Some(_n);
            },
            _ => {},
        }
        stack
    }

    pub fn push(
        &mut self,
        item: T,
    ) {
        match &self.data {
            None => {
                println!("First Node with value of '{:?}'", &item);
                self.data = Some(item);
                return;
            },
            _ => {},
        }
        match &mut self.next {
            Some(last_item) => {
                last_item.push(item);
            },
            _ => {
                let new_node = Stack::new(Some(item));
                self.next = Some(Box::new(new_node));
            },
        }
    }

    pub fn print_me(&self) {
        if let Some(val) = &self.data {
            println!("{:?}", val);
        }
        match &self.next {
            Some(next_) => {
                next_.print_me();
            },
            _ => {},
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        None
    }
}
