#[cfg(feature = "with-serde")]
use crate::chain::{Base16DecodedBytes, Base16EncodedBytes};
use crate::{chain::ErgoBox, sigma_protocol::SigmaProp, types::SType};
#[cfg(feature = "with-serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "with-serde")]
use sigma_ser::serializer::{SerializationError, SigmaSerializable};
#[cfg(feature = "with-serde")]
use std::convert::TryFrom;

#[derive(PartialEq, Eq, Debug, Clone)]
/// Collection for primitive values (i.e byte array)
pub enum CollPrim {
    /// Collection of bools
    CollBoolean(Vec<bool>),
    /// Collection of bytes
    CollByte(Vec<i8>),
    /// Collection of shorts
    CollShort(Vec<i16>),
    /// Collection of ints
    CollInt(Vec<i32>),
    /// Collection of longs
    CollLong(Vec<i64>),
}

#[derive(PartialEq, Eq, Debug, Clone)]
/// Constant value
pub enum ConstantVal {
    /// Boolean
    Boolean(bool),
    /// Byte
    Byte(i8),
    /// Short
    Short(i16),
    /// Int
    Int(i32),
    /// Long
    Long(i64),
    /// Big integer
    BigInt,
    /// GroupElement
    GroupElement,
    /// Sigma property
    SigmaProp(Box<SigmaProp>),
    /// Box
    CBox(Box<ErgoBox>),
    /// AVL tree
    AvlTree,
    /// Collection of primitive values
    CollPrim(CollPrim),
    /// Collection of same type constant value
    Coll(Vec<ConstantVal>),
    /// Tuple (arbitrary type values)
    Tup(Vec<ConstantVal>),
}

impl ConstantVal {
    /// Create Sigma property constant
    pub fn sigma_prop(prop: SigmaProp) -> ConstantVal {
        ConstantVal::SigmaProp(Box::new(prop))
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "with-serde",
    serde(into = "Base16EncodedBytes", try_from = "Base16DecodedBytes")
)]
/// Constant
pub struct Constant {
    /// Constant type
    pub tpe: SType,
    /// Constant value
    pub v: ConstantVal,
}

#[cfg(feature = "with-serde")]
impl Into<Base16EncodedBytes> for Constant {
    fn into(self) -> Base16EncodedBytes {
        Base16EncodedBytes::new(&self.sigma_serialise_bytes())
    }
}

#[cfg(feature = "with-serde")]
impl TryFrom<Base16DecodedBytes> for Constant {
    type Error = SerializationError;
    fn try_from(bytes: Base16DecodedBytes) -> Result<Self, Self::Error> {
        Constant::sigma_parse_bytes(bytes.0)
    }
}

impl Constant {
    /// Create bool value constant
    pub fn bool(v: bool) -> Constant {
        Constant {
            tpe: SType::SBoolean,
            v: ConstantVal::Boolean(v),
        }
    }

    /// Create byte value constant
    pub fn byte(v: i8) -> Constant {
        Constant {
            tpe: SType::SByte,
            v: ConstantVal::Byte(v),
        }
    }

    /// Create short value constant
    pub fn short(v: i16) -> Constant {
        Constant {
            tpe: SType::SShort,
            v: ConstantVal::Short(v),
        }
    }

    /// Create int value constant
    pub fn int(v: i32) -> Constant {
        Constant {
            tpe: SType::SInt,
            v: ConstantVal::Int(v),
        }
    }

    /// Create long value constant
    pub fn long(v: i64) -> Constant {
        Constant {
            tpe: SType::SLong,
            v: ConstantVal::Long(v),
        }
    }

    /// Create byte array value constant
    pub fn byte_array(v: Vec<i8>) -> Constant {
        Constant {
            tpe: SType::SColl(Box::new(SType::SByte)),
            v: ConstantVal::CollPrim(CollPrim::CollByte(v)),
        }
    }

    /// Create Sigma property constant
    pub fn sigma_prop(prop: SigmaProp) -> Constant {
        Constant {
            tpe: SType::SSigmaProp,
            v: ConstantVal::sigma_prop(prop),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::collection::vec;
    use proptest::prelude::*;

    impl Arbitrary for Constant {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                any::<bool>().prop_map(Constant::bool),
                any::<i8>().prop_map(Constant::byte),
                any::<i16>().prop_map(Constant::short),
                any::<i32>().prop_map(Constant::int),
                any::<i64>().prop_map(Constant::long),
                (vec(any::<i8>(), 0..100)).prop_map(Constant::byte_array),
            ]
            .boxed()
        }
    }
}
