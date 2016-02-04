use std::collections::BTreeSet;
use std::iter::{FromIterator, Extend};

#[derive(Clone, Hash, Debug)]
pub struct GSet<T> {
    set: BTreeSet<T>,
}

impl<T: Ord> GSet<T> {
    pub fn new() -> GSet<T> {
        GSet { set: BTreeSet::new() }
    }

    pub fn insert(&mut self, value: T) -> bool {
        self.set.insert(value)
    }

    pub fn contains(&mut self, value: &T) -> bool {
        self.set.contains(&value)
    }
}

impl<T: Ord> FromIterator<T> for GSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> GSet<T> {
        let mut set = GSet::new();
        set.extend(iter);
        set
    }
}

impl<T: Ord> Extend<T> for GSet<T> {
    #[inline]
    fn extend<Iter: IntoIterator<Item = T>>(&mut self, iter: Iter) {
        for elem in iter {
        self.insert(elem);
        }
    }
}
