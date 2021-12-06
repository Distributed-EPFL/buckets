use crate::{BucketZip, Split};

use rayon::prelude::*;

pub fn apply<Z, I, K, R>(buckets: Z, split: Split<I>, kernel: K) -> Split<R>
where
    Z: Copy + BucketZip,
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
