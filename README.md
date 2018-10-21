# Quantized Density Fields
Rust implementation of Quantized Density Fields data structure.

![Docs.rs](https://docs.rs/quantized-density-fields/badge.svg)
![Crates.io](https://img.shields.io/crates/v/quantized-density-fields.svg)

# Usage
Record in `Cargo.toml`:
```toml
[dependencies]
quantized-density-fields = "0.1.4"
```

Your crate module:
```rust
// declare import of external QDF crate.
extern crate quantized_density_fields;

// use QDF struct.
use quantized_density_fields::QDF;

// create 2D space with `9` as state of whole universe.
let mut qdf = QDF::new(2, 9);
let id = qdf.root();
// increase root space density (2D space is subdivided into 3 children chunks).
qdf.increase_space_density(id);
let subs = qdf.space(qdf.root()).subspace().to_vec();
qdf.increase_space_density(subs[0]);
let subs2 = qdf.space(subs[0]).subspace();
// find shortest path between two platonic spaces.
assert_eq!(qdf.find_path(subs2[0], subs[2]).unwrap(), vec![subs2[0], subs2[1], subs[2]]);
```

# Concept
QDF does not exists in any space - it IS the space, it defines it,
it describes it so there are no space coordinates and it is your responsibility to deliver it.
In future releases this crate will have module for projecting QDF onto Euclidean space
and will have a satelite crate to easly traverse and visualize space.

To sample specified region you have to know some space ID and gather the rest of information
based on it neighbors spaces.
It gives the ability to cotrol space density at specified locations, which can be used
for example to simulate space curvature based on gravity.

# TODO
- Illustrations showing idea of how QDF container works.
