use crate::{BucketZip, Split};

pub fn apply<Z, I, K, R>(buckets: Z, split: Split<I>, kernel: K) -> Split<R>
where
    Z: Copy + BucketZip,
    I: 'static + Send,
    K: 'static + Send + Copy + Fn(&mut Z::Zip, I) -> R,
    R: 'static + Send,
{
    let zip = buckets.zip().into_iter();

    let (splits, indexes) = split.take();
    let splits = splits.into_iter();

    let handles = zip
        .zip(splits)
        .map(|(mut inners, inputs)| {
            std::thread::spawn(move || {
                let outputs = inputs
                    .into_iter()
                    .map(|input| kernel(&mut inners, input))
                    .collect::<Vec<_>>();

                (inners, outputs)
            })
        })
        .collect::<Vec<_>>();

    let (zip, splits): (Vec<_>, Vec<_>) = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .unzip();

    buckets.unzip(zip);

    Split::raw(splits, indexes)
}
