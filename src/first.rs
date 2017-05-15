#[allow(dead_code)]
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

#[allow(dead_code)]
impl BST {
    pub fn new()->Self
    {
        BST {root: Link::Empty}
    }
    pub fn insert(&mut self, elem: i32)->bool
    {
        let mut result=false;
        match (*self).root {
            Link::Empty => 
                {
                    let new_node=Node{
                                    elem: elem,
                                    left: Link::Empty,
                                    right: Link::Empty};
                    (*self).root=Link::More(Box::new(new_node));
                    result=true;
                }
            Link::More(ref boxed_node) => 
                {
                    if boxed_node.elem==elem {result=false}
                    else if boxed_node.elem>elem{}
                    else if boxed_node.elem<elem{}
                    result =true;
                }
        }
        result
    }

}

#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn insert_to_empty_tree()
    {
        let mut tree = BST::new();
        tree.insert(34);
    }

}