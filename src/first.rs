#[derive(Debug)]
pub struct BST {
    root: Link
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    right: Link,
    left: Link,
}

impl BST {
    fn new()->Self
    {
        BST {root: Link::Empty}
    }
}

