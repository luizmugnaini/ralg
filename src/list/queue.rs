use std::ptr;

// Push to the end of the queue
// Pop from the head of the queue
struct Queue<T> {
    head: List<T>,
    tail: *mut Node<T>,
}

type List<T> = *mut Node<T>;

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, key: T) {
        unsafe {
            let new_tail =
                Box::into_raw(Box::new(Node::new(key, ptr::null_mut())));

            if self.tail.is_null() {
                self.head = new_tail;
            } else {
                (*(self.tail)).next = new_tail;
            }

            self.tail = new_tail;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                // We create a box in order to correctly deallocate `last_head`
                let last_head = Box::from_raw(self.head);
                self.head = (*last_head).next;

                // The head was the only node
                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }

                Some(last_head.key)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|head| &head.key) }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|head| &mut head.key) }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter {
                next: self.head.as_ref(),
            }
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        unsafe {
            IterMut {
                next: self.head.as_mut(),
            }
        }
    }
}

pub struct IntoIter<T>(Queue<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_ref();
                &node.key
            })
        }
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                &mut node.key
            })
        }
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
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
    fn push_n_pop() {
        let mut queue = Queue::new();
        assert!(queue.pop().is_none());

        for x in 0..3 {
            queue.push(x);
        }
        for x in 0..3 {
            assert_eq!(queue.pop(), Some(x));
        }

        assert!(queue.pop().is_none());
    }

    #[test]
    fn into_iter() {
        let mut queue = Queue::new();
        for x in 0..3 {
            queue.push(x);
        }

        // Check iterator
        let mut iter = queue.into_iter();
        for x in 0..3 {
            assert_eq!(iter.next(), Some(x));
        }
        assert!(iter.next().is_none());
    }

    #[test]
    fn iter() {
        let mut queue = Queue::new();
        for x in 0..3 {
            queue.push(x);
        }

        // Check iterator
        let mut iter = queue.iter();
        for x in 0..3 {
            assert_eq!(iter.next(), Some(&x));
        }
        assert_eq!(iter.next(), None);

        // Check for integrity of the original `queue`
        for x in 0..3 {
            assert_eq!(queue.pop(), Some(x));
        }
        assert!(queue.pop().is_none());
    }

    #[test]
    fn iter_mut() {
        let mut queue = Queue::new();
        for x in 0..4 {
            queue.push(x);
        }

        let mut iter = queue.iter_mut();
        for mut x in 0..3 {
            assert_eq!(iter.next(), Some(&mut x));
        }

        // assert mutability
        let key = iter.next().unwrap();
        assert_eq!(key, &mut 3);
        *key += 3;

        // Check for integrity of the original `queue`
        for x in 0..3 {
            assert_eq!(queue.pop(), Some(x));
        }

        // Mutated value
        assert_eq!(queue.pop(), Some(6));

        assert!(queue.pop().is_none());
    }

    #[test]
    fn peek() {
        let mut queue = Queue::new();
        for x in 0..3 {
            queue.push(x);
        }

        for x in 0..3 {
            assert_eq!(queue.peek(), Some(&x));
            queue.pop();
        }
        assert_eq!(queue.peek(), None);
    }

    #[test]
    fn peek_mut() {
        let mut queue = Queue::new();
        for x in 0..4 {
            queue.push(x);
        }

        for mut x in 0..3 {
            assert_eq!(queue.peek_mut(), Some(&mut x));
            queue.pop();
        }

        // Assert mutability
        let key = queue.peek_mut().unwrap();
        assert_eq!(key, &mut 3);
        *key += 3;
        assert_eq!(queue.peek_mut(), Some(&mut 6));

        assert_eq!(queue.pop(), Some(6));
        assert!(queue.peek().is_none());
    }
}
