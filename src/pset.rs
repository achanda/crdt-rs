use std::iter::{FromIterator, Extend};

use gset::GSet;

#[derive(Clone, Hash, Debug)]
pub struct PSet<T> {
    add_set:    GSet<T>,
    remove_set: GSet<T>,
}

impl<T: Ord + Clone> PSet<T> {
    pub fn new() -> PSet<T> {
        PSet {
            add_set:    GSet::new(),
            remove_set: GSet::new()
        }
    }

    pub fn insert(&mut self, value: T) -> bool {
        self.add_set.insert(value)
    }

    pub fn remove(&mut self, value: T) -> bool {
        if self.add_set.contains(&value) {
            self.remove_set.insert(value);
        }
        return false
    }

    pub fn contains(&mut self, value: &T) -> bool {
        self.add_set.contains(&value) & !self.remove_set.contains(&value)
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
