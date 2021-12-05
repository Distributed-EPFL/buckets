use crate::{Key, Splittable, BUCKETS};

use fasthash::xx;

use std::{iter, ops::Fn};

pub struct Split<Item> {
    splits: Vec<Vec<Item>>,
    buckets: Vec<usize>,
}

impl<Item> Split<Item> {
    pub fn with_key<I, F, K>(items: I, key: F) -> Self
    where
        I: IntoIterator<Item = Item>,
        F: Fn(&Item) -> K,
        K: Key,
    {
        let items = items.into_iter();
        let (size_hint, _) = items.size_hint();

        let mut splits = iter::repeat_with(|| Vec::with_capacity(size_hint / BUCKETS))
            .take(BUCKETS)
            .collect::<Vec<_>>();

        let mut buckets = Vec::new();

        for item in items {
            let bucket = xx::hash32(key(&item).represent()) as usize % BUCKETS;

            splits[bucket].push(item);
            buckets.push(bucket);
        }

        Split { splits, buckets }
    }

    pub(crate) fn raw(splits: Vec<Vec<Item>>, buckets: Vec<usize>) -> Self {
        Split { splits, buckets }
    }

    pub fn join(self) -> Vec<Item> {
        let mut splits = self
            .splits
            .into_iter()
            .map(|split| split.into_iter())
            .collect::<Vec<_>>();

        self.buckets
            .into_iter()
            .map(|bucket| splits[bucket].next().unwrap())
            .collect()
    }

    pub(crate) fn take(self) -> (Vec<Vec<Item>>, Vec<usize>) {
        (self.splits, self.buckets)
    }
}

impl<Item> FromIterator<Item> for Split<Item>
where
    Item: Splittable,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Item>,
    {
        Split::with_key(iter, |item| item.key())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_then_join() {
        let split = Split::<u64>::with_key(0..16384, |item| *item);
        let join = split.join();

        assert_eq!(join, (0..16384).collect::<Vec<_>>());
    }
}
