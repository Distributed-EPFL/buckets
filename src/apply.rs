use crate::{BucketZip, Split};

use rayon::prelude::*;

pub fn apply<Z, I, K, R>(mut buckets: Z, split: Split<I>, kernel: K) -> Split<R>
where
    Z: BucketZip,
    I: 'static + Send,
    K: 'static + Send + Sync + Copy + Fn(&mut Z::Zip, I) -> R,
    R: 'static + Send,
{
    let zip = buckets.zip().into_par_iter();

    let (splits, indexes) = split.explode();
    let splits = splits.into_par_iter();

    let (zip, splits): (Vec<_>, Vec<_>) = zip
        .zip(splits)
        .map(|(mut inners, inputs)| {
            let outputs = inputs
                .into_iter()
                .map(|input| kernel(&mut inners, input))
                .collect::<Vec<_>>();

            (inners, outputs)
        })
        .unzip();

    buckets.unzip(zip);
    Split::raw(splits, indexes)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Buckets;

    use std::collections::{HashMap, HashSet};

    #[test]
    fn single_set_then_check() {
        let mut map = Buckets::<HashMap<u64, usize>>::new();
        let keys = Split::<u64>::with_key(0..50000, |key| *key);

        let keys = apply(&mut map, keys, |map, key| {
            map.insert(key, 42);
            key
        });

        apply(&mut map, keys, |map, key| {
            assert_eq!(*map.get(&key).unwrap(), 42)
        });
    }

    #[test]
    fn double_set_then_check() {
        let mut map = Buckets::<HashMap<u64, usize>>::new();
        let mut set = Buckets::<HashSet<u64>>::new();

        let keys = Split::<u64>::with_key(0..50000, |key| *key);

        let keys = apply((&mut map, &mut set), keys, |(map, set), key| {
            map.insert(key, 42);
            set.insert(key);

            key
        });

        apply((&mut map, &mut set), keys, |(map, set), key| {
            assert_eq!(*map.get(&key).unwrap(), 42);
            assert!(set.contains(&key));
        });
    }
}
