struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut length = 0;
        let mut current_link = &self.head;
        // while let Some(node) = current_link {
        //     length +=1;
        //     current_link = &node.next;
        // }

        while (*current_link).is_some() {
            length += 1;
            current_link = &(current_link.as_ref().unwrap().next);
        }

        length
    }

    pub fn push(&mut self, _element: T) {
        let head_node = Node::new(_element, self.head.take());
        self.head = Some(Box::new(head_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                let node = *node;
                self.head = node.next;
                Some(node.data)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.data)
    }
}

impl<T> Node<T> {
    fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Node { data, next }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut new_list = SimpleLinkedList::new();

        let mut current_node = &self.head;
        while let Some(node) = current_node {
            new_list.push(node.data.clone());
            current_node = &node.next;
        }
        new_list
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_item: &[T]) -> Self {
        let mut list = Self::new();

        for item in _item {
            list.push(item.clone());
        }

        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut v = Vec::new();

        while let Some(data) = self.pop() {
            v.push(data);
        }

        v.reverse();
        v
    }
}
