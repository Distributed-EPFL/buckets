use crate::BUCKETS;

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

    pub(crate) fn take_buckets(&mut self) -> Vec<Inner> {
        self.buckets.drain(..).collect()
    }

    pub(crate) fn restore_buckets(&mut self, buckets: Vec<Inner>) {
        self.buckets = buckets
    }
}
