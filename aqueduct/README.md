# aqueduct

[![Clippy](https://github.com/Scattered-Systems/aqueduct/actions/workflows/clippy.yml/badge.svg)](https://github.com/Scattered-Systems/aqueduct/actions/workflows/clippy.yml)
[![Rust](https://github.com/Scattered-Systems/aqueduct/actions/workflows/rust.yml/badge.svg)](https://github.com/Scattered-Systems/aqueduct/actions/workflows/rust.yml)

***

Aqueduct is a hybrid cybersecurity protocol blending elements of homorphic encryption with elliptic-curve cryptography leveraging the Modularity Theorem to naturally bridge the two distinctly different worlds. Aqueduct is set to implement fully-homorphic encryption measures in addition to the recently formalized tweakable sleeve, a novel sleeve achieving modularity by implementing a composition mechanism based on tweakble hash functions.

## Getting Started

Use Rust's built-in package manager [crates](https://crates.io/crates/aqueduct) to install *aqueduct*.

```bash
cargo install package
```

## Usage

- [crates.io](https://crates.io/crates/aqueduct)
- [docs.rs](https://docs.rs/aqueduct)

```rust
use scsys::prelude::*;

fn main() {
  println!("{:?}", Message::<String>::default());
}
```

## Research

- [The Modularity Theorem](https://www.universiteitleiden.nl/binaries/content/assets/science/mi/scripties/dobbendebruynbach.pdf)
- [Tweakable Sleeve](https://iohk.io/en/research/library/papers/tweakable-sleeve-a-novel-sleeve-construction-based-on-tweakable-hash-functions/)

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

- [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
- [MIT](https://choosealicense.com/licenses/mit/)
