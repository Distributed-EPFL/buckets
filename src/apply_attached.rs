use crate::{BucketZip, Split};

use rayon::prelude::*;

pub fn apply_attached<Z, A, I, K, R>(
    mut buckets: Z,
    attachment: &A,
    split: Split<I>,
    kernel: K,
) -> Split<R>
where
    Z: BucketZip,
    A: Sync,
    I: 'static + Send,
    K: 'static + Send + Sync + Copy + Fn(&mut Z::Zip, &A, I) -> R,
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
                .map(|input| kernel(&mut inners, &attachment, input))
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

    use crate::{apply, Buckets};

    use rand::prelude::*;

    use std::{
        collections::{HashMap, HashSet},
        iter,
    };

    #[test]
    fn single_set_then_check() {
        let mut map = Buckets::<HashMap<u64, usize>>::new();
        let keys = Split::<u64>::with_key(0..50000, |key| *key);

        let keys = apply_attached(&mut map, &42, keys, |map, attachment, key| {
            map.insert(key, *attachment);
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

        let keys = apply_attached(
            (&mut map, &mut set),
            &42,
            keys,
            |(map, set), attachment, key| {
                map.insert(key, *attachment);
                set.insert(key);

                key
            },
        );

        apply((&mut map, &mut set), keys, |(map, set), key| {
            assert_eq!(*map.get(&key).unwrap(), 42);
            assert!(set.contains(&key));
        });
    }

    #[test]
    fn shift() {
        let mut source = Buckets::<HashMap<u64, u64>>::new();
        let mut sink = Buckets::<HashMap<u64, u64>>::new();

        let keys = Split::<u64>::with_key(0..50000, |key| *key);

        apply((&mut source, &mut sink), keys, |(source, sink), key| {
            source.insert(key, 1000);
            sink.insert(key, 0);
        });

        for _ in 0..16 {
            let keys = Split::<u64>::with_key(
                iter::repeat_with(|| random::<u64>() % 50000).take(50000),
                |key| *key,
            );

            let shift = random::<u64>() % 3;

            apply_attached(
                (&mut source, &mut sink),
                &shift,
                keys,
                |(source, sink), shift, key| {
                    *source.get_mut(&key).unwrap() -= *shift;
                    *sink.get_mut(&key).unwrap() += *shift;
                },
            );
        }

        let keys = Split::<u64>::with_key(0..50000, |key| *key);

        apply((&mut source, &mut sink), keys, |(source, sink), key| {
            assert_eq!(source.get(&key).unwrap() + sink.get(&key).unwrap(), 1000);
        });
    }
}
