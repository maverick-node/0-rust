#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            head: None,
        }
    }

    pub fn push(&mut self, value: T) {
        let mut nn = Node {
            value: value,
            next: self.head.take().map(Box::new),
        };
        self.head = Some(nn);
    }

    pub fn pop(&mut self) -> Option<T> {
        let old_head = self.head.take();

        if let Some(node) = old_head {
            if let Some(boxed_next) = node.next {
                self.head = Some(*boxed_next);
            } else {
                self.head = None;
            }
            Some(node.value)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            count += 1;
            current = node.next.as_deref();
        }
        count
    }
}
