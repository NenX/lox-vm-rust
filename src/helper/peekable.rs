#[derive(Clone, Debug)]
pub struct MyPeekable<I: Iterator> {
    iter: I,
    /// Remember a peeked value, even if it was None.
    peeked: Vec<I::Item>,
}

impl<T: Iterator> MyPeekable<T> {
    pub fn new(iter: T) -> Self {
        Self {
            iter,
            peeked: vec![],
        }
    }
    /// Returns a reference to the next() value without advancing the iterator.
    ///
    /// Like [`next`], if there is a value, it is wrapped in a `Some(T)`.
    /// But if the iteration is over, `None` is returned.
    ///
    /// [`next`]: Iterator::next
    ///
    /// Because `peek()` returns a reference, and many iterators iterate over
    /// references, there can be a possibly confusing situation where the
    /// return value is a double reference. You can see this effect in the
    /// examples below.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use lox_vm_rust::MyPeekable;
    /// let xs = [1, 2, 3];
    ///
    /// let mut iter = MyPeekable::new(xs.iter());
    ///
    /// // peek() lets us see into the future
    /// assert_eq!(iter.peek(0), Some(&&1));
    /// assert_eq!(iter.next(), Some(&1));
    ///
    ///
    /// // The iterator does not advance even if we `peek` multiple times
    /// assert_eq!(iter.peek(0), Some(&&2));
    /// assert_eq!(iter.peek(1), Some(&&3));
    ///
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    ///
    /// // After the iterator is finished, so is `peek()`
    /// assert_eq!(iter.peek(0), None);
    /// assert_eq!(iter.next(), None);
    /// ```
    #[inline]
    pub fn peek(&mut self, idx: usize) -> Option<&T::Item> {
        while self.peeked.len() <= idx {
            match self.iter.next() {
                Some(item) => self.peeked.push(item),
                None => break,
            }
        }
        self.peeked.get(idx)
    }

    /// Returns a mutable reference to the next() value without advancing the iterator.
    ///
    /// Like [`next`], if there is a value, it is wrapped in a `Some(T)`.
    /// But if the iteration is over, `None` is returned.
    ///
    /// Because `peek_mut()` returns a reference, and many iterators iterate over
    /// references, there can be a possibly confusing situation where the
    /// return value is a double reference. You can see this effect in the examples
    /// below.
    ///
    /// [`next`]: Iterator::next
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use lox_vm_rust::MyPeekable;
    /// let xs = [1, 2, 3];
    ///
    /// let mut iter = MyPeekable::new(xs.iter());
    ///
    /// // Like with `peek()`, we can see into the future without advancing the iterator.
    /// assert_eq!(iter.peek_mut(0), Some(&mut &1));
    /// assert_eq!(iter.peek_mut(2), Some(&mut &3));
    /// assert_eq!(iter.next(), Some(&1));
    ///
    /// // Peek into the iterator and set the value behind the mutable reference.
    /// if let Some(p) = iter.peek_mut(1) {
    ///     assert_eq!(*p, &3);
    ///     *p = &5;
    /// }
    ///
    /// // The value we put in reappears as the iterator continues.
    /// assert_eq!(iter.collect::<Vec<_>>(), vec![&2, &5]);
    /// ```
    #[inline]
    pub fn peek_mut(&mut self, idx: usize) -> Option<&mut T::Item> {
        while self.peeked.len() <= idx {
            match self.iter.next() {
                Some(item) => self.peeked.push(item),
                None => break,
            }
        }
        self.peeked.get_mut(idx)
    }

    /// Consume and return the next value of this iterator if a condition is true.
    ///
    /// If `func` returns `true` for the next value of this iterator, consume and return it.
    /// Otherwise, return `None`.
    ///
    /// # Examples
    /// Consume a number if it's equal to 0.
    /// ```
    /// use lox_vm_rust::MyPeekable;
    /// let xs = [1, 2, 3];
    ///
    /// let mut iter = MyPeekable::new(xs.iter());
    /// // The first item of the iterator is 0; consume it.
    /// assert_eq!(iter.next_if(|&&x| x == 1), Some(&1));
    /// // The next item returned is now 1, so `next_if` will return `None`.
    /// assert_eq!(iter.next_if(|&&x| x == 0), None);
    /// // `next_if` saves the value of the next item if it was not equal to `expected`.
    /// assert_eq!(iter.next(), Some(&2));
    /// ```
    ///
    /// Consume any number less than 10.
    /// ```
    /// use lox_vm_rust::MyPeekable;
    ///
    /// let mut iter = MyPeekable::new(1..20);
    /// // Consume all numbers less than 10
    /// while iter.next_if(|&x| x < 10).is_some() {}
    /// // The next value returned will be 10
    /// assert_eq!(iter.next(), Some(10));
    /// ```
    pub fn next_if(&mut self, func: impl FnOnce(&T::Item) -> bool) -> Option<T::Item> {
        match self.next() {
            Some(matched) if func(&matched) => Some(matched),
            Some(matched) => {
                self.peeked.push(matched);
                None
            }
            _ => None,
        }
    }

    /// Consume and return the next item if it is equal to `expected`.
    ///
    /// # Example
    /// Consume a number if it's equal to 0.
    /// ```
    /// use lox_vm_rust::MyPeekable;
    ///
    /// let mut iter = MyPeekable::new(1..5);
    ///
    /// // The first item of the iterator is 0; consume it.
    /// assert_eq!(iter.next_if_eq(&1), Some(1));
    /// // The next item returned is now 1, so `next_if` will return `None`.
    /// assert_eq!(iter.next_if_eq(&0), None);
    /// // `next_if_eq` saves the value of the next item if it was not equal to `expected`.
    /// assert_eq!(iter.next(), Some(2));
    /// ```
    ///
    pub fn next_if_eq<I>(&mut self, expected: &I) -> Option<T::Item>
    where
        I: ?Sized,
        T::Item: PartialEq<I>,
    {
        self.next_if(|next| next == expected)
    }
}
impl<T: Iterator> Iterator for MyPeekable<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.peeked.is_empty() {
            self.iter.next()
        } else {
            let x = self.peeked.remove(0);
            Some(x)
        }
    }
}
