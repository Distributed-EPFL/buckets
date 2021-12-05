use crate::{Buckets, BUCKETS};

pub(crate) trait BucketZip {
    type Zip;

    fn zip(self) -> Vec<Self::Zip>;
    fn unzip(self, zip: Vec<Self::Zip>);
}

impl<I> BucketZip for &mut Buckets<I> {
    type Zip = I;

    fn zip(self) -> Vec<Self::Zip> {
        self.take_buckets()
    }

    fn unzip(self, zip: Vec<Self::Zip>) {
        self.restore_buckets(zip);
    }
}

impl<I0, I1> BucketZip for (&mut Buckets<I0>, &mut Buckets<I1>) {
    type Zip = (I0, I1);

    fn zip(self) -> Vec<Self::Zip> {
        let mut b0 = self.0.take_buckets().into_iter();
        let mut b1 = self.1.take_buckets().into_iter();

        let mut zip = Vec::with_capacity(BUCKETS);

        for _ in 0..BUCKETS {
            zip.push((b0.next().unwrap(), b1.next().unwrap()));
        }

        zip
    }

    fn unzip(self, zip: Vec<Self::Zip>) {
        let mut b0 = Vec::with_capacity(BUCKETS);
        let mut b1 = Vec::with_capacity(BUCKETS);

        for (i0, i1) in zip {
            b0.push(i0);
            b1.push(i1);
        }

        self.0.restore_buckets(b0);
        self.1.restore_buckets(b1);
    }
}

impl<I0, I1, I2> BucketZip for (&mut Buckets<I0>, &mut Buckets<I1>, &mut Buckets<I2>) {
    type Zip = (I0, I1, I2);

    fn zip(self) -> Vec<Self::Zip> {
        let mut b0 = self.0.take_buckets().into_iter();
        let mut b1 = self.1.take_buckets().into_iter();
        let mut b2 = self.2.take_buckets().into_iter();

        let mut zip = Vec::with_capacity(BUCKETS);

        for _ in 0..BUCKETS {
            zip.push((b0.next().unwrap(), b1.next().unwrap(), b2.next().unwrap()));
        }

        zip
    }

    fn unzip(self, zip: Vec<Self::Zip>) {
        let mut b0 = Vec::with_capacity(BUCKETS);
        let mut b1 = Vec::with_capacity(BUCKETS);
        let mut b2 = Vec::with_capacity(BUCKETS);

        for (i0, i1, i2) in zip {
            b0.push(i0);
            b1.push(i1);
            b2.push(i2);
        }

        self.0.restore_buckets(b0);
        self.1.restore_buckets(b1);
        self.2.restore_buckets(b2);
    }
}

impl<I0, I1, I2, I3> BucketZip
    for (
        &mut Buckets<I0>,
        &mut Buckets<I1>,
        &mut Buckets<I2>,
        &mut Buckets<I3>,
    )
{
    type Zip = (I0, I1, I2, I3);

    fn zip(self) -> Vec<Self::Zip> {
        let mut b0 = self.0.take_buckets().into_iter();
        let mut b1 = self.1.take_buckets().into_iter();
        let mut b2 = self.2.take_buckets().into_iter();
        let mut b3 = self.3.take_buckets().into_iter();

        let mut zip = Vec::with_capacity(BUCKETS);

        for _ in 0..BUCKETS {
            zip.push((
                b0.next().unwrap(),
                b1.next().unwrap(),
                b2.next().unwrap(),
                b3.next().unwrap(),
            ));
        }

        zip
    }

    fn unzip(self, zip: Vec<Self::Zip>) {
        let mut b0 = Vec::with_capacity(BUCKETS);
        let mut b1 = Vec::with_capacity(BUCKETS);
        let mut b2 = Vec::with_capacity(BUCKETS);
        let mut b3 = Vec::with_capacity(BUCKETS);

        for (i0, i1, i2, i3) in zip {
            b0.push(i0);
            b1.push(i1);
            b2.push(i2);
            b3.push(i3);
        }

        self.0.restore_buckets(b0);
        self.1.restore_buckets(b1);
        self.2.restore_buckets(b2);
        self.3.restore_buckets(b3);
    }
}
