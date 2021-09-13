//! Token related types

use ergotree_ir::serialization::SigmaSerializeResult;
use ergotree_ir::serialization::{
    sigma_byte_reader::SigmaByteRead, sigma_byte_writer::SigmaByteWrite, SigmaParsingError,
    SigmaSerializable,
};
use std::convert::TryFrom;

use super::digest32::Digest32;
use super::ergo_box::BoxId;
use derive_more::From;
use derive_more::Into;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(feature = "arbitrary")]
use proptest_derive::Arbitrary;

/// newtype for token id
#[derive(PartialEq, Eq, Hash, Debug, Clone, From, Into)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct TokenId(Digest32);

impl TokenId {
    /// token id size in bytes
    pub const SIZE: usize = Digest32::SIZE;
}

impl From<BoxId> for TokenId {
    fn from(i: BoxId) -> Self {
        TokenId(i.into())
    }
}

impl From<TokenId> for Vec<i8> {
    fn from(v: TokenId) -> Self {
        v.0.into()
    }
}

impl From<TokenId> for String {
    fn from(v: TokenId) -> Self {
        v.0.into()
    }
}

impl SigmaSerializable for TokenId {
    fn sigma_serialize<W: SigmaByteWrite>(&self, w: &mut W) -> SigmaSerializeResult {
        self.0.sigma_serialize(w)
    }
    fn sigma_parse<R: SigmaByteRead>(r: &mut R) -> Result<Self, SigmaParsingError> {
        Ok(Self(Digest32::sigma_parse(r)?))
    }
}

/// Token amount with bound checks
#[cfg(not(feature = "json"))]
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, PartialOrd, Ord)]
pub struct TokenAmount(u64);

/// Token amount with bound checks
#[cfg(feature = "json")]
#[serde_with::serde_as]
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Copy, PartialOrd, Ord)]
pub struct TokenAmount(
    // Tries to decode as u64 first, then fallback to string. Encodes as u64 always
    // see details - https://docs.rs/serde_with/1.9.4/serde_with/struct.PickFirst.html
    #[serde_as(as = "serde_with::PickFirst<(_, serde_with::DisplayFromStr)>")] u64,
);

impl TokenAmount {
    /// minimal allowed value
    pub const MIN_RAW: u64 = 1;
    /// maximal allowed value
    pub const MAX_RAW: u64 = i64::MAX as u64;

    /// Addition with overflow check
    pub fn checked_add(&self, rhs: &Self) -> Result<Self, TokenAmountError> {
        let raw = self
            .0
            .checked_add(rhs.0)
            .ok_or(TokenAmountError::Overflow)?;
        if raw > Self::MAX_RAW {
            Err(TokenAmountError::OutOfBounds(raw))
        } else {
            Ok(Self(raw))
        }
    }

    /// Subtraction with overflow and bounds check
    pub fn checked_sub(&self, rhs: &Self) -> Result<Self, TokenAmountError> {
        let raw = self
            .0
            .checked_sub(rhs.0)
            .ok_or(TokenAmountError::Overflow)?;
        if raw < Self::MIN_RAW {
            Err(TokenAmountError::OutOfBounds(raw))
        } else {
            Ok(Self(raw))
        }
    }
}

/// BoxValue errors
#[derive(Error, Eq, PartialEq, Debug, Clone)]
pub enum TokenAmountError {
    /// Value is out of bounds
    #[error("Token amount is out of bounds: {0}")]
    OutOfBounds(u64),
    /// Overflow
    #[error("Overflow")]
    Overflow,
}

impl TryFrom<u64> for TokenAmount {
    type Error = TokenAmountError;

    fn try_from(v: u64) -> Result<Self, Self::Error> {
        if (TokenAmount::MIN_RAW..=TokenAmount::MAX_RAW).contains(&v) {
            Ok(TokenAmount(v))
        } else {
            Err(TokenAmountError::OutOfBounds(v))
        }
    }
}

impl From<TokenAmount> for u64 {
    fn from(ta: TokenAmount) -> Self {
        ta.0
    }
}

impl From<TokenAmount> for i64 {
    fn from(ta: TokenAmount) -> Self {
        ta.0 as i64
    }
}

/// Token represented with token id paired with it's amount
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct Token {
    /// token id
    #[cfg_attr(feature = "json", serde(rename = "tokenId"))]
    pub token_id: TokenId,
    /// token amount
    #[cfg_attr(feature = "json", serde(rename = "amount"))]
    pub amount: TokenAmount,
}

impl From<(TokenId, TokenAmount)> for Token {
    fn from(token_pair: (TokenId, TokenAmount)) -> Self {
        Token {
            token_id: token_pair.0,
            amount: token_pair.1,
        }
    }
}

pub(crate) mod arbitrary {
    use crate::chain::Base16DecodedBytes;

    use super::*;
    use proptest::prelude::*;

    pub enum ArbTokenIdParam {
        Predef,
        Arbitrary,
    }

    impl Default for ArbTokenIdParam {
        fn default() -> Self {
            ArbTokenIdParam::Predef
        }
    }

    impl Arbitrary for TokenId {
        type Parameters = ArbTokenIdParam;

        fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
            match args {
                ArbTokenIdParam::Predef => prop_oneof![
                    Just(TokenId::from(
                        Digest32::try_from(
                            Base16DecodedBytes::try_from(
                                "3130a82e45842aebb888742868e055e2f554ab7d92f233f2c828ed4a43793710"
                                    .to_string()
                            )
                            .unwrap()
                        )
                        .unwrap()
                    )),
                    Just(TokenId::from(
                        Digest32::try_from(
                            Base16DecodedBytes::try_from(
                                "e7321ffb4ec5d71deb3110eb1ac09612b9cf57445acab1e0e3b1222d5b5a6c60"
                                    .to_string()
                            )
                            .unwrap()
                        )
                        .unwrap()
                    )),
                    Just(TokenId::from(
                        Digest32::try_from(
                            Base16DecodedBytes::try_from(
                                "ad62f6dd92e7dc850bc406770dfac9a943dd221a7fb440b7b2bcc7d3149c1792"
                                    .to_string()
                            )
                            .unwrap()
                        )
                        .unwrap()
                    ))
                ]
                .boxed(),
                ArbTokenIdParam::Arbitrary => (any::<Digest32>()).prop_map_into().boxed(),
            }
        }

        type Strategy = BoxedStrategy<Self>;
    }

    impl Arbitrary for TokenAmount {
        type Parameters = ();

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            (TokenAmount::MIN_RAW..=TokenAmount::MAX_RAW / 100000)
                .prop_map(Self)
                .boxed()
        }
        type Strategy = BoxedStrategy<Self>;
    }
}

#[cfg(test)]
pub mod tests {
    use ergotree_ir::serialization::sigma_serialize_roundtrip;
    use proptest::prelude::*;

    use crate::chain::token::TokenId;

    proptest! {

        #[test]
        fn token_id_roundtrip(v in any::<TokenId>()) {
            prop_assert_eq![sigma_serialize_roundtrip(&v), v];
        }
    }
}
