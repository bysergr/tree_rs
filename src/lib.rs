use std::cmp::PartialOrd;
use std::fmt::{Debug, Display};

pub struct Node<'a, T: Debug + Display + PartialOrd + ?Sized> {
    value: &'a T,
    left: Option<Box<Node<'a, T>>>,
    right: Option<Box<Node<'a, T>>>,
}

impl<'a, T: Debug + Display + PartialOrd + ?Sized> Node<'a, T> {
    pub fn new(value: &'a T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, new_value: &'a T) {
        if self.value == new_value {
            return;
        }
        let target_node = if new_value < self.value {
            &mut self.left
        } else {
            &mut self.right
        };
        match target_node {
            &mut Some(ref mut subnode) => subnode.insert(new_value),
            &mut None => {
                let new_node = Node {
                    value: new_value,
                    left: None,
                    right: None,
                };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }

    pub fn show(&self) {
        match self.left {
            Some(ref left) => left.show(),
            None => (),
        }

        println!("{}", self.value);

        match self.right {
            Some(ref right) => right.show(),
            None => (),
        }
    }
}
