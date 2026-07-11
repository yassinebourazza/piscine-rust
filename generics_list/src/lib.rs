#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Box<Node<T>>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None
         }
    }

    pub fn push(&mut self, value: T) {
        self.head = Some(Box::new(Node {
            value,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) {
        if let Some(last) = self.head.take() {
            self.head = last.next;
        }
    }

    pub fn len(&self) -> usize {
        let mut cur = self.head.as_ref();
        let mut count = 0;
        while let Some(node) = cur {
            count += 1;
            cur = node.next.as_ref();
        }
        count
    }
}