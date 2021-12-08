use crate::{BucketZip, Split};

use rayon::prelude::*;

pub fn apply_sparse_attached<Z, A, I, K, R>(
    mut buckets: Z,
    attachment: &A,
    split: Split<I>,
    kernel: K,
) -> Vec<R>
where
    Z: BucketZip,
    A: Sync,
    I: 'static + Send,
    K: 'static + Send + Sync + Copy + Fn(&mut Z::Zip, &A, I) -> Option<R>,
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
                .filter_map(|input| kernel(&mut inners, &attachment, input))
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

        let mut powers = apply_sparse_attached(&mut map, &33, keys, |map, attachment, key| {
            let value = *map.get(&key).unwrap();

            if value.count_ones() == 1 {
                Some(value + *attachment)
            } else {
                None
            }
        });

        powers.sort();

        assert_eq!(
            powers,
            vec![
                34, 37, 49, 97, 289, 1057, 4129, 16417, 65569, 262177, 1048609, 4194337, 16777249,
                67108897, 268435489, 1073741857
            ]
        );
    }
}
