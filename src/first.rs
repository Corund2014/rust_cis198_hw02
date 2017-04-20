#[derive(Debug)]
pub struct List {
    head: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List{head:Link::Empty}
        
    }
    pub fn push(&mut self, e:i32)
    {
        let n=Node{elem:e,next:Link::Empty};
        self.head=Link::More(Box::new(n));
    }
}