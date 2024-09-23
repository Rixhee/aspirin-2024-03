// Now try and implement a doubly linked version. Give an explanation
// for why this doesn't work.

struct Node {
    val: i32,
    previous: Link,
    next: Link,
}

type Link = Option<Box<Node>>;

pub struct LinkedStack {
    head: Link,
}

impl LinkedStack {
    fn new() -> Self {
        let head = None;
        return Self { head };
    }

    fn push(&mut self, val: i32) {
        self.head = Some(Box::new(Node {
            val: val,
            next: self.head.take(),
            previous: None,
        }));
        self.head.unwrap().next.unwrap().previous = self.head;
    }

    fn pop(&mut self) -> Option<i32> {
        let mut val = None;
        self.head.take().map(|node| {
            self.head = node.next;
            val = Some(node.val)
        });
        val
    }
}
