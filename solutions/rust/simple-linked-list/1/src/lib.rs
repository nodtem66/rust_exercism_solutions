#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SimpleLinkedList<T: PartialEq + Eq + Clone> {
    Node(T, Box<SimpleLinkedList<T>>),
    HEAD,
}

impl<T: PartialEq + Eq + Clone> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList::HEAD
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self == &SimpleLinkedList::HEAD
    }

    pub fn len(&self) -> usize {
        match *self {
            SimpleLinkedList::HEAD => 0,
            SimpleLinkedList::Node(_, ref tail) => 1 + tail.len(),
        }
    }

    pub fn push(&mut self, _element: T) {
        let node = Box::new(self.clone());
        *self = SimpleLinkedList::Node(_element, node);

    }

    pub fn pop(&mut self) -> Option<T> {
        match self {
            SimpleLinkedList::HEAD => None,
            SimpleLinkedList::Node(value, next) => {
                let val = value.clone();
                *self = *next.to_owned();
                Some(val)
            }
        }
    }
 
    pub fn peek(&self) -> Option<&T> {
        match self {
            SimpleLinkedList::HEAD => None,
            SimpleLinkedList::Node(value, _) => Some(value),
        }
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        let mut current = self;
        while let SimpleLinkedList::Node(value, next) = current {
            list.push(value);
            current = *next;
        }
        list
    }
}

impl<T: PartialEq + Eq + Clone> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::<T>::new();
        for i in _iter {
            list.push(i);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T: PartialEq + Eq + Clone> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = Vec::new();
        while let Some(value) = _linked_list.pop() {
            vec.push(value);
        }
        vec.reverse();
        vec
    }
}
