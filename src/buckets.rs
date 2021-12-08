use crate::{Split, BUCKETS};

use std::iter;

pub struct Buckets<Inner> {
    buckets: Vec<Inner>,
}

impl<Inner> Buckets<Inner> {
    pub fn new() -> Self
    where
        Inner: Default,
    {
        let buckets = iter::repeat_with(|| Default::default())
            .take(BUCKETS)
            .collect();

        Buckets { buckets }
    }

    pub fn apply<I, K, R>(&mut self, split: Split<I>, kernel: K) -> Split<R>
    where
        Inner: 'static + Send,
        I: 'static + Send,
        K: 'static + Send + Sync + Copy + Fn(&mut Inner, I) -> R,
        R: 'static + Send,
    {
        crate::apply(self, split, kernel)
    }

    pub fn apply_sparse<I, K, R>(&mut self, split: Split<I>, kernel: K) -> Vec<R>
    where
        Inner: 'static + Send,
        I: 'static + Send,
        K: 'static + Send + Sync + Copy + Fn(&mut Inner, I) -> Option<R>,
        R: 'static + Send,
    {
        crate::apply_sparse(self, split, kernel)
    }

    pub(crate) fn take_buckets(&mut self) -> Vec<Inner> {
        self.buckets.drain(..).collect()
    }

    pub(crate) fn restore_buckets(&mut self, buckets: Vec<Inner>) {
        self.buckets = buckets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Buckets;

    use std::collections::HashMap;

    #[test]
    fn apply() {
        let mut map = Buckets::<HashMap<u64, usize>>::new();
        let keys = Split::<u64>::with_key(0..50000, |key| *key);

        let keys = map.apply(keys, |map, key| {
            map.insert(key, 42);
            key
        });

        map.apply(keys, |map, key| assert_eq!(*map.get(&key).unwrap(), 42));
    }

    #[test]
    fn apply_sparse() {
        let mut map = Buckets::<HashMap<u64, u64>>::new();
        let keys = Split::<u64>::with_key(0..50000, |key| *key);

        let keys = map.apply(keys, |map, key| {
            map.insert(key, key * key);
            key
        });

        let mut powers = map.apply_sparse(keys, |map, key| {
            let value = *map.get(&key).unwrap();

            if value.count_ones() == 1 {
                Some(value)
            } else {
                None
            }
        });

        powers.sort();

        assert_eq!(
            powers,
            vec![
                1, 4, 16, 64, 256, 1024, 4096, 16384, 65536, 262144, 1048576, 4194304, 16777216,
                67108864, 268435456, 1073741824
            ]
        );
    }
}
