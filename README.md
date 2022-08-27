# TypeMap

`TypeMap` is a wrapper around a `HashMap<TypeId, Box<dyn Any>>`, guaranteeing it only ever holds at most 1 value of a given type.

## Usage

`TypeMap` is kept incredibly lean and isn't intended for production use. If you're looking for a more generally useful alternative, supporting the full range of features you'd expect from a Map structure, and supporting cloneable types, see [`anymap`](https://lib.rs/crates/anymap).

## Example

```rs
use type_map::TypeMap;

let mut map = TypeMap::new();

// Insert a value of type i32. The key will be the type_id of the value provided,
// so it does not need to be specified.
map.insert(7);

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
```
