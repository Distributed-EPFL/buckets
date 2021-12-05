pub trait Key {
    type Representation: AsRef<[u8]>;
    fn represent(&self) -> Self::Representation;
}

macro_rules! primitives {
    ($(($primitive: ty, $size: expr)), +) => {
        $(impl Key for $primitive {
            type Representation = [u8; $size];
            fn represent(&self) -> Self::Representation {
                self.to_ne_bytes()
            }
        })*
    }
}

primitives! {
    (u8, 1),
    (i8, 1),
    (u16, 2),
    (i16, 2),
    (u32, 4),
    (i32, 4),
    (u64, 8),
    (i64, 8),
    (u128, 16),
    (i128, 16)
}
