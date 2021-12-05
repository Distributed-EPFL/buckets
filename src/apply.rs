use crate::{BucketZip, Split};

pub fn apply<Z, I, K, R>(buckets: Z, split: Split<I>, kernel: K) -> Split<R>
where
    Z: BucketZip,
    I: 'static + Send,
    K: 'static + Send + Copy + Fn(&mut Z::Zip, I) -> R,
    R: 'static + Send,
{
    let zip = buckets.zip().into_iter();

    let (splits, buckets) = split.take();
    let splits = splits.into_iter();

    let handles = zip
        .zip(splits)
        .map(|(mut inners, inputs)| {
            std::thread::spawn(move || {
                inputs
                    .into_iter()
                    .map(|input| kernel(&mut inners, input))
                    .collect::<Vec<_>>()
            })
        })
        .collect::<Vec<_>>();

    let splits = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect::<Vec<_>>();

    Split::raw(splits, buckets)
}
