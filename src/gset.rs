use std::collections::BTreeSet;
use std::iter::{FromIterator, Extend};

/// A `GSet` is an implementation of a grow-only set.
/// The underlying data-structure is a `BTreeSet`
#[derive(Clone, Hash, Debug)]
pub struct GSet<T> {
    set: BTreeSet<T>,
}

/// This is used to iterate over the elements of a GSet
pub struct IntoIter<T> {
    iter: <BTreeSet<T> as IntoIterator>::IntoIter,
}

impl<T: Ord + Clone> GSet<T> {
    /// Creates a new, empty `GSet`
    pub fn new() -> GSet<T> {
        GSet { set: BTreeSet::new() }
    }

    /// Inserts an element of type `T` into the
    /// given `GSet`. Returns `true` if the element
    /// was already in the set. Otherwise, it inserts
    /// the element and returns `false`
    pub fn insert(&mut self, value: T) -> bool {
        self.set.insert(value)
    }

    /// Checks if the given value of type `T` is in
    /// the set. If the check is successful, returns
    /// `true`. Otherwise returns `false`.
    pub fn contains(&mut self, value: &T) -> bool {
        self.set.contains(&value)
    }

    /// Returns the number of elements in the given `GSet`
    pub fn len(&self) -> usize {
        self.set.len()
    }

    /// Checks if the given `GSet` is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the set difference between two `GSet`s as another `GSet`
    pub fn difference(&mut self, other: &GSet<T>) -> GSet<T> {
        let difference: BTreeSet<_> = self.set.difference(&other.set).cloned().collect();
        GSet { set: difference }
    }

    /// Returns the intersection between two `GSet`s as another `GSet`
    pub fn intersection(&mut self, other: &GSet<T>) -> GSet<T> {
        let intersection: BTreeSet<_> = self.set.intersection(&other.set).cloned().collect();
        GSet { set: intersection }
    }

    /// Returns the union between two `GSet`s as another `GSet`
    pub fn union(&mut self, other: &GSet<T>) -> GSet<T> {
        let union: BTreeSet<_> = self.set.union(&other.set).cloned().collect();
        GSet { set: union }
    }
}

impl<T: Ord + Clone> FromIterator<T> for GSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> GSet<T> {
        let mut set = GSet::new();
        set.extend(iter);
        set
    }
}

impl<T: Ord + Clone> Extend<T> for GSet<T> {
    #[inline]
    fn extend<Iter: IntoIterator<Item = T>>(&mut self, iter: Iter) {
        for elem in iter {
        self.insert(elem);
        }
    }
}

impl<T> IntoIterator for GSet<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter { iter: self.set.into_iter() }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gset_insert() {
        let mut a: GSet<i32> = GSet::new();
        a.insert(1);
        a.insert(2);

        let elements = a.into_iter().collect::<Vec<_>>();
        assert_eq!(elements, [1,2]);
    }
}
