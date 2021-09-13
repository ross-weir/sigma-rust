//! JSON serialization using string for BoxValue and TokenAmount

use derive_more::FromStr;
use ergo_lib::chain::ergo_box::BoxId;
use ergo_lib::chain::ergo_box::BoxValue;
use ergo_lib::chain::ergo_box::ErgoBox;
use ergo_lib::chain::ergo_box::NonMandatoryRegisters;
use ergo_lib::chain::token::Token;
use ergo_lib::chain::transaction::DataInput;
use ergo_lib::chain::transaction::Input;
use ergo_lib::chain::transaction::Transaction;
use ergo_lib::chain::transaction::TxId;
use ergo_lib::ergotree_ir::ergo_tree::ErgoTree;
use serde::Serialize;

#[derive(Serialize, PartialEq, Debug, Clone)]
pub(crate) struct TransactionJsonDapp {
    #[cfg_attr(feature = "json", serde(rename = "id"))]
    pub tx_id: TxId,
    /// inputs, that will be spent by this transaction.
    #[cfg_attr(feature = "json", serde(rename = "inputs"))]
    pub inputs: Vec<Input>,
    /// inputs, that are not going to be spent by transaction, but will be reachable from inputs
    /// scripts. `dataInputs` scripts will not be executed, thus their scripts costs are not
    /// included in transaction cost and they do not contain spending proofs.
    #[cfg_attr(feature = "json", serde(rename = "dataInputs"))]
    pub data_inputs: Vec<DataInput>,
    #[cfg_attr(feature = "json", serde(rename = "outputs"))]
    pub outputs: Vec<ErgoBoxJsonDapp>,
}

impl From<Transaction> for TransactionJsonDapp {
    fn from(t: Transaction) -> Self {
        TransactionJsonDapp {
            tx_id: t.id(),
            inputs: t.inputs,
            data_inputs: t.data_inputs,
            outputs: t.outputs.into_iter().map(|b| b.into()).collect(),
        }
    }
}

#[derive(Serialize, PartialEq, Eq, Debug, Clone)]
pub(crate) struct ErgoBoxJsonDapp {
    #[serde(rename = "boxId", alias = "id")]
    pub box_id: Option<BoxId>,
    /// amount of money associated with the box
    #[serde(rename = "value")]
    pub value: BoxValueJsonDapp,
    /// guarding script, which should be evaluated to true in order to open this box
    #[serde(rename = "ergoTree", with = "ergo_lib::chain::json::ergo_tree")]
    pub ergo_tree: ErgoTree,
    /// secondary tokens the box contains
    #[serde(rename = "assets")]
    // TODO: JsonDapp equivalent
    pub tokens: Vec<Token>,
    ///  additional registers the box can carry over
    #[serde(rename = "additionalRegisters")]
    pub additional_registers: NonMandatoryRegisters,
    /// height when a transaction containing the box was created.
    /// This height is declared by user and should not exceed height of the block,
    /// containing the transaction with this box.
    #[serde(rename = "creationHeight")]
    pub creation_height: u32,
    /// id of transaction which created the box
    #[serde(rename = "transactionId", alias = "txId")]
    pub transaction_id: TxId,
    /// number of box (from 0 to total number of boxes the transaction with transactionId created - 1)
    #[serde(rename = "index")]
    pub index: u16,
}

impl From<ErgoBox> for ErgoBoxJsonDapp {
    fn from(b: ErgoBox) -> Self {
        ErgoBoxJsonDapp {
            box_id: b.box_id().into(),
            value: b.value.into(),
            ergo_tree: b.ergo_tree,
            tokens: b.tokens,
            additional_registers: b.additional_registers,
            creation_height: b.creation_height,
            transaction_id: b.transaction_id,
            index: b.index,
        }
    }
}

#[serde_with::serde_as]
#[derive(
    serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash, Debug, Clone, Copy, FromStr,
)]
// Tries to decode as string first, then fallback to u64. Encodes as string always
// see details - https://docs.rs/serde_with/1.9.4/serde_with/struct.PickFirst.html
/// Box value in nanoERGs with bound checks
pub(crate) struct BoxValueJsonDapp(
    #[serde_as(as = "serde_with::PickFirst<(serde_with::DisplayFromStr, _)>")] pub(crate) u64,
);

impl From<BoxValue> for BoxValueJsonDapp {
    fn from(bv: BoxValue) -> Self {
        BoxValueJsonDapp(*bv.as_u64())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prelude::*;

    proptest! {

        #[test]
        fn ergo_box_roundtrip(b in any::<ErgoBox>()) {
            let b_dapp: ErgoBoxJsonDapp = b.into();
            let j = serde_json::to_string(&b_dapp).unwrap();
            // eprintln!("{}", j);
            let b_parsed: ErgoBox = serde_json::from_str(&j)?;
            prop_assert_eq![b_dapp, b_parsed.into()];
        }

    }
}
