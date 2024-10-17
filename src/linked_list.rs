struct Node {
    data: i32,
    next: Option<Box<Node>>
}

struct LinkedList {
    head: Option<Box<Node>>
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push(&mut self, data: i32) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take() // take ownership of the current head
        });

        self.head = Some(new_node);
    }
}