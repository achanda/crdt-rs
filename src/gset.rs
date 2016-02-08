use std::collections::BTreeSet;
use std::iter::{FromIterator, Extend};

#[derive(Clone, Hash, Debug)]
pub struct GSet<T> {
    set: BTreeSet<T>,
}

pub struct IntoIter<T> {
    iter: <BTreeSet<T> as IntoIterator>::IntoIter,
}

impl<T: Ord + Clone> GSet<T> {
    pub fn new() -> GSet<T> {
        GSet { set: BTreeSet::new() }
    }

    pub fn insert(&mut self, value: T) -> bool {
        self.set.insert(value)
    }

    pub fn contains(&mut self, value: &T) -> bool {
        self.set.contains(&value)
    }

    pub fn len(&self) -> usize {
        self.set.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn difference(&mut self, other: &GSet<T>) -> GSet<T> {
        let difference: BTreeSet<_> = self.set.difference(&other.set).cloned().collect();
        GSet { set: difference }
    }

    pub fn intersection(&mut self, other: &GSet<T>) -> GSet<T> {
        let intersection: BTreeSet<_> = self.set.intersection(&other.set).cloned().collect();
        GSet { set: intersection }
    }

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
