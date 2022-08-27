use std::hash::Hasher;

#[derive(Default)]
pub(super) struct Noop {
    value: u64,
}

// The only values this "hasher" ever receives are unique u64 `TypeId`s,
// so we skip any kind of hashing and simply store them as is.
impl Hasher for Noop {
    fn finish(&self) -> u64 {
        self.value
    }

    fn write(&mut self, bytes: &[u8]) {
        self.value = u64::from_ne_bytes(bytes.try_into().unwrap());
    }
}
