use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T: Eq + Hash> {
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    internal: HashMap<T, ()>,
}

impl<T: Eq + Hash + Clone> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        let mut result: HashMap<T, ()> = HashMap::new();
        for item in _input {
            result.insert(item.clone(), ());
        }
        Self {
            internal: result,
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.internal.contains_key(element)
    }

    pub fn add(&mut self, element: T) {
        self.internal.insert(element, ());
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        self.internal.keys().all(|k| _other.internal.contains_key(k))
    }

    pub fn is_empty(&self) -> bool {
        self.internal.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        self.internal.keys().all(|k| !_other.internal.contains_key(k))
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        let mut result = Self::new(&[]);
        for key in self.internal.keys() {
            if _other.internal.contains_key(key) {
                result.add(key.clone());
            }
        }
        result
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        let mut result = Self::new(&[]);
        for key in self.internal.keys() {
            if !_other.internal.contains_key(key) {
                result.add(key.clone());
            }
        }
        result
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        let mut result = Self::new(&[]);
        for key in self.internal.keys() {
            result.add(key.clone());
        }
        for key in _other.internal.keys() {
            result.add(key.clone());
        }
        result
    }
}
