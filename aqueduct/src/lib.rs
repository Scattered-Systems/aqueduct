/*
    Appellation: aqueduct <library>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description:
        Aqueduct is a hybrid encryption protocol combining aspects of homomorphic encryption with elliptic-curve cryptography leveraging the modularity theorem to
        naturally bridge the two distinctly different approaches.
*/
#[cfg(feature = "sleeves")]
pub use aqueduct_sleeves as sleeves;

pub mod prelude {
    #[cfg(feature = "sleeves")]
    pub use crate::sleeves::*;
}
