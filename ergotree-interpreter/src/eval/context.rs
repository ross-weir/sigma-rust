pub(crate) mod ir_ergo_box_dummy;

use std::rc::Rc;

use crate::sigma_protocol::prover::ContextExtension;
use ergotree_ir::ir_ergo_box::IrErgoBox;
use ergotree_ir::mir::header::PreHeader;

/// Interpreter's context (blockchain state)
#[derive(Debug)]
pub struct Context {
    /// Current height
    pub height: u32,
    /// Box that contains the script we're evaluating (from spending transaction inputs)
    pub self_box: Rc<dyn IrErgoBox>,
    /// Spending transaction outputs
    pub outputs: Vec<Rc<dyn IrErgoBox>>,
    /// Spending transaction data inputs
    pub data_inputs: Vec<Rc<dyn IrErgoBox>>,
    /// Spending transaction inputs
    pub inputs: Vec<Rc<dyn IrErgoBox>>,
    /// Pre header of current block
    pub pre_header: PreHeader,
    /// prover-defined key-value pairs, that may be used inside a script
    pub extension: ContextExtension,
}

impl Context {
    /// Return a new Context with given context extension
    pub fn with_extension(self, ext: ContextExtension) -> Self {
        Context {
            extension: ext,
            ..self
        }
    }
}

#[cfg(feature = "arbitrary")]
mod arbitrary {

    use super::ir_ergo_box_dummy::*;
    use super::*;
    use ergotree_ir::ir_ergo_box::IrErgoBox;
    use proptest::collection::vec;
    use proptest::prelude::*;

    impl Arbitrary for Context {
        type Parameters = ();

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            (
                0..i32::MAX as u32,
                any::<IrErgoBoxDummy>(),
                vec(any::<IrErgoBoxDummy>(), 1..3),
                vec(any::<IrErgoBoxDummy>(), 1..3),
                vec(any::<IrErgoBoxDummy>(), 0..3),
                any::<PreHeader>(),
                any::<ContextExtension>(),
            )
                .prop_map(
                    |(height, self_box, outputs, inputs, data_inputs, pre_header, extension)| {
                        Self {
                            height,
                            self_box: Rc::new(self_box),
                            outputs: outputs
                                .into_iter()
                                .map(|b| Rc::new(b) as Rc<dyn IrErgoBox>)
                                .collect(),
                            data_inputs: data_inputs
                                .into_iter()
                                .map(|b| Rc::new(b) as Rc<dyn IrErgoBox>)
                                .collect(),
                            inputs: inputs
                                .into_iter()
                                .map(|b| Rc::new(b) as Rc<dyn IrErgoBox>)
                                .collect(),
                            pre_header,
                            extension,
                        }
                    },
                )
                .boxed()
        }

        type Strategy = BoxedStrategy<Self>;
    }
}

#[cfg(test)]
mod tests {}
