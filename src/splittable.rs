use crate::Key;

pub trait Splittable {
    type Key: Key;
    fn key(&self) -> Self::Key;
}
