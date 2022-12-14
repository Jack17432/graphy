use std::rc::Rc;

pub trait Children {
    fn children<T: std::cmp::PartialEq, C>(&self, node: T) -> Option<&Vec<C>>;
}

pub struct Node<T> {
    pub data: T,
    pub children: Vec<Rc<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T, children: Vec<Rc<Node<T>>>) -> Rc<Self> {
        Rc::new(Self {
            data,
            children
        })
    }

    pub fn from(data: T) -> Rc<Self> {
        Rc::new(Self {
            data,
            children: vec![]
        })
    }
}