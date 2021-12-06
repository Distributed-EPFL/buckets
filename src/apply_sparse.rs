use crate::{BucketZip, Split};

use rayon::prelude::*;

pub fn apply_sparse<Z, I, K, R>(mut buckets: Z, split: Split<I>, kernel: K) -> Vec<R>
where
    Z: BucketZip,
    I: 'static + Send,
    K: 'static + Send + Sync + Copy + Fn(&mut Z::Zip, I) -> Option<R>,
    R: 'static + Send,
{
    let zip = buckets.zip().into_par_iter();

    let (splits, _) = split.explode();
    let splits = splits.into_par_iter();

    let (zip, filtered): (Vec<_>, Vec<_>) = zip
        .zip(splits)
        .map(|(mut inners, inputs)| {
            let outputs = inputs
                .into_iter()
                .filter_map(|input| kernel(&mut inners, input))
                .collect::<Vec<_>>();

            (inners, outputs)
        })
        .unzip();

    buckets.unzip(zip);

    let mut sparse = Vec::new();

    for mut outputs in filtered {
        sparse.append(&mut outputs);
    }

    sparse
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{apply, Buckets};

    use std::collections::HashMap;

    #[test]
    fn filter_powers() {
        let mut map = Buckets::<HashMap<u64, u64>>::new();
        let keys = Split::<u64>::with_key(0..50000, |key| *key);

        let keys = apply(&mut map, keys, |map, key| {
            map.insert(key, key * key);
            key
        });

        let mut powers = apply_sparse(&mut map, keys, |map, key| {
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
