use std::fmt::Debug;
use crate::Location;

/// Wraps the value of an ast node and gives us access to the nodes location in the source file(s).
///
/// Equality comparisons on nodes (where applicable) are performed by comparing their inner values,
/// and the start/end locations are ignored.
#[derive(Debug, Clone)]
pub struct Node<T: Sized + Debug + Clone> {
    pub value: T,
    pub start: Location,
    pub end: Location
}

impl<T: Sized + Debug + Clone> Node<T> {
    pub fn new(value: T, start: Location, end: Location) -> Node<T> {
        Node {
            value,
            start,
            end
        }
    }
}

/// Checks the inner value of two nodes for equality
impl<T: Sized + Debug + Clone + PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.value.eq(&other.value)
    }
}

impl<T: Sized + Debug + Clone + PartialEq> Eq for Node<T> {}