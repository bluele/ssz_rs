#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
extern crate core;

mod array;
mod bitlist;
mod bitvector;
mod boolean;
mod container;
mod de;
mod list;
mod merkleization;
mod ser;
#[cfg(feature = "serde")]
mod serde_test;
mod uint;
mod union;
mod vector;
pub mod std;

pub use crate::std::{Vec, vec};
pub use bitlist::Bitlist;
pub use bitvector::Bitvector;
pub use de::{Deserialize, DeserializeError};
pub use list::List;
pub use merkleization::{Context as MerkleizationContext, MerkleizationError, Merkleized, Node};
pub use ser::{Serialize, SerializeError};
pub use uint::U256;

/// `Sized` is a trait for types that can
/// provide sizing information relevant for the SSZ spec.
pub trait Sized {
    // is this type variable or fixed size?
    fn is_variable_size() -> bool;

    fn size_hint() -> usize;
}

/// `SimpleSerialize` is a trait for types
/// conforming to the SSZ spec.
pub trait SimpleSerialize: Serialize + Deserialize + Sized + Merkleized + Default {
    fn is_composite_type() -> bool {
        true
    }
}

/// `serialize` is a convenience function for taking a value that
/// implements `SimpleSerialize` and attempting to encode it to
/// a `Vec<u8>` according to the SSZ spec.
pub fn serialize<T>(value: &T) -> Result<Vec<u8>, SerializeError>
where
    T: SimpleSerialize,
{
    let mut result = vec![];
    value.serialize(&mut result)?;
    Ok(result)
}

/// `deserialize` is a convenience function for taking an encoding
/// for some value that implements `SimpleSerialize` in a `&[u8]`
/// and attempting to deserialize that value from the byte representation.
pub fn deserialize<T>(encoding: &[u8]) -> Result<T, DeserializeError>
where
    T: SimpleSerialize,
{
    T::deserialize(encoding)
}

#[derive(Debug)]
pub enum SimpleSerializeError {
    Serialize,
    Deserialize,
    Merkleization,
    List,
    Vector,
}

/// The `prelude` contains common traits and types a user of this library
/// would want to have handy with a simple (single) import.
pub mod prelude {
    pub use crate as ssz_rs;
    pub use crate::bitlist::Bitlist;
    pub use crate::bitvector::Bitvector;
    pub use crate::de::Deserialize;
    pub use crate::de::DeserializeError;
    pub use crate::list::List;
    pub use crate::merkleization::{
        is_valid_merkle_branch, merkleize, mix_in_selector, pack, pack_bytes, MerkleizationError,
        Merkleized, Node,
    };
    pub use crate::ser::{Serialize, SerializeError};
    pub use crate::uint::U256;
    pub use crate::vector::Vector;
    pub use crate::MerkleizationContext;
    pub use crate::SimpleSerialize;
    pub use crate::SimpleSerializeError;
    pub use crate::Sized;
    pub use crate::{deserialize, serialize};
    pub use ssz_rs_derive::SimpleSerialize;
}

/// `internal` contains functionality that is exposed purely for the derive proc macro crate
pub mod internal {
    // exported for derive macro to avoid code duplication...
    pub use crate::merkleization::{merkleize, mix_in_selector};
    pub use crate::ser::serialize_composite_from_components;
}
