#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod hasher;

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    hash::BuildHasherDefault,
};

/// A map structure containing either zero or one value(s) for any given type.
#[derive(Default)]
pub struct TypeMap {
    raw: HashMap<TypeId, Box<dyn Any>, BuildHasherDefault<hasher::Noop>>,
}

impl TypeMap {
    /// Creates an empty `TypeMap`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `TypeMap` with at least the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            raw: HashMap::with_capacity_and_hasher(capacity, BuildHasherDefault::default()),
        }
    }

    /// Clears the map, removing all key-value pairs. Keeps the allocated memory for reuse.
    pub fn clear(&mut self) {
        self.raw.clear();
    }

    /// Returns `true` if the map contains a value of type `T`.
    pub fn contains<T: Any>(&self) -> bool {
        self.raw.contains_key(&TypeId::of::<T>())
    }

    /// Returns a reference to the value of type `T`.
    pub fn get<T: Any>(&self) -> Option<&T> {
        self.raw
            .get(&TypeId::of::<T>())
            .map(|value| value.downcast_ref().unwrap())
    }

    /// Returns a mutable reference to the value of type `T`.
    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.raw
            .get_mut(&TypeId::of::<T>())
            .map(|value| value.downcast_mut().unwrap())
    }

    /// Inserts a value of type `T` into the map.
    ///
    /// Returns the previous value of type `T`.
    pub fn insert<T: Any>(&mut self, value: T) -> Option<T> {
        self.raw
            .insert(TypeId::of::<T>(), Box::new(value))
            .map(|previous_value| *previous_value.downcast().unwrap())
    }

    /// Returns true if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.raw.len()
    }

    /// Removes and returns a value of type `T` from the map.
    pub fn remove<T: Any>(&mut self) -> Option<T> {
        self.raw
            .remove(&TypeId::of::<T>())
            .map(|value| *value.downcast().unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut map = TypeMap::new();

        // Insert a value of type i32. The key will be the type_id of the value provided,
        // so it does not need to be specified.
        map.insert(7);

        // Items are retrieved via their type, so there is no need to specify a key
        assert_eq!(map.get::<i32>(), Some(&7));

        // `insert` also returns the previous value
        let previous_value = map.insert(42);

        assert_eq!(previous_value, Some(7)); // Note no reference, this is the owned value
        assert_eq!(map.get::<i32>(), Some(&42));

        // Values of different types can co-exist
        map.insert(64_u32);

        assert_eq!(map.get::<i32>(), Some(&42));
        assert_eq!(map.get::<u32>(), Some(&64));

        // Values can also be retrived mutably
        let map_u32 = map.get_mut::<u32>().unwrap();
        *map_u32 *= 10;

        assert_eq!(map.get::<u32>(), Some(&640));

        // Removing values updates the map and returns the value
        let map_i32 = map.remove::<i32>();

        assert_eq!(map_i32, Some(42));
        assert_eq!(map.get::<i32>(), None);
    }
}
