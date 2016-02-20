use std::iter::{FromIterator, Extend};

use gset::GSet;

/// Implements a state based two-phase set
/// using two `GSet`s
#[derive(Clone, Hash, Debug, Serialize, Deserialize)]
pub struct PSet<T> {
    add_set:    GSet<T>,
    remove_set: GSet<T>,
}

impl<T: Ord + Clone> PSet<T> {
    /// Creates a new `PSet`
    pub fn new() -> PSet<T> {
        PSet {
            add_set:    GSet::new(),
            remove_set: GSet::new()
        }
    }

    /// Inserts an element of type `T` into the
    /// given `PSet`. Returns `true` if the element
    /// was already in the set. Otherwise, it inserts
    /// the element and returns `false`
    pub fn insert(&mut self, value: T) -> bool {
        self.add_set.insert(value)
    }

    /// Removes the given element from the `PSet`
    pub fn remove(&mut self, value: T) -> bool {
        if self.add_set.contains(&value) {
            self.remove_set.insert(value);
        }
        return false
    }

    /// Checks if the given value of type `T` is in
    /// the set. If the check is successful, returns
    /// `true`. Otherwise returns `false`
    pub fn contains(&mut self, value: &T) -> bool {
        self.add_set.contains(&value) & !self.remove_set.contains(&value)
    }

    /// Returns the contents of the given set. This is
    /// equivalent to the set difference between the add set and the
    /// remove set
    pub fn contents(&mut self) -> Vec<T> {
        self.add_set.difference(&self.remove_set).into_iter().collect()
    }

    /// Checks if the given `PSet` is empty
    pub fn is_empty(&mut self) -> bool {
        self.contents().is_empty()
    }

    /// Returns the length of the given `PSet`
    pub fn len(&mut self) -> usize {
        self.contents().len()
    }

    /// Returns the set union between the given `PSet` and
    /// another `PSet` as a `PSet`
    pub fn union(&mut self, other: &PSet<T>) -> PSet<T> {
        PSet {
            add_set:    self.add_set.union(&other.add_set),
            remove_set: self.remove_set.union(&other.remove_set)
        }
    }

    /// Returns the set intersection between the given `PSet` and
    /// another `PSet` as a `PSet`
    pub fn intersection(&mut self, other: &PSet<T>) -> PSet<T> {
        PSet {
            add_set:    self.add_set.intersection(&other.add_set),
            remove_set: self.remove_set.union(&other.remove_set)
        }
    }

    /// Returns the set difference between the given `PSet` and
    /// another `PSet` as a `PSet`
    pub fn difference(&mut self, other: &PSet<T>) -> PSet<T> {
        PSet {
            add_set:    self.add_set.difference(&other.add_set),
            remove_set: GSet::new()
        }
    }
}

impl<T: Ord + Clone> FromIterator<T> for PSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> PSet<T> {
        let mut set = PSet::new();
        set.extend(iter);
        set
    }
}

impl<T: Ord + Clone> Extend<T> for PSet<T> {
    #[inline]
    fn extend<Iter: IntoIterator<Item = T>>(&mut self, iter: Iter) {
        for elem in iter {
            self.insert(elem);
        }
    }
}
