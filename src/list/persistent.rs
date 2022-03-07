use std::sync::Arc;

pub struct Persistant<T> {
    head: List<T>,
}

type List<T> = Option<Arc<Node<T>>>;

impl<T> Persistant<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn prepend(&self, key: T) -> Self {
        Self {
            head: Some(Arc::new(Node::new(key, self.head.clone()))),
        }
    }

    pub fn tail(&self) -> Self {
        Self {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.key)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Default for Persistant<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.key
        })
    }
}

impl<T> Drop for Persistant<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            match Arc::try_unwrap(node) {
                Ok(mut node) => head = node.next.take(),
                _ => break,
            }
        }
    }
}

struct Node<T> {
    key: T,
    next: List<T>,
}

impl<T> Node<T> {
    fn new(key: T, next: List<T>) -> Self {
        Self { key, next }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let list = Persistant::new();
        // Check head and tail in empty case
        assert_eq!(list.head(), None);
        assert_eq!(list.tail().head(), None);

        let list = list.prepend(0).prepend(1).prepend(2);
        assert_eq!(list.head(), Some(&2));

        // Check tail
        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), Some(&0));

        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = Persistant::new().prepend(0).prepend(1).prepend(2);

        let mut iter = list.iter();
        for x in (0..3).rev() {
            assert_eq!(iter.next(), Some(&x));
        }
        assert_eq!(iter.next(), None);
    }
}
