use serde::{Deserialize, Serialize};

use crate::{WireId, Value, Gate};

/// BuildGate is similar to Gate but without output wires.
/// Useful in combination with Builder.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum BuildGate {
    Constant(Value),
    AssertZero(WireId),
    Copy(WireId),
    Add(WireId, WireId),
    Mul(WireId, WireId),
    AddConstant(WireId, Value),
    MulConstant(WireId, Value),
    And(WireId, WireId),
    Xor(WireId, WireId),
    Not(WireId),
    Instance,
    Witness,
}

pub(crate) const NO_OUTPUT: WireId = WireId::max_value();

use BuildGate::*;

impl BuildGate {
    pub fn with_output(self, output: WireId) -> Gate {
        match self {
            Constant(value) => Gate::Constant(output, value),
            AssertZero(input) => {
                assert_eq!(output, NO_OUTPUT);
                Gate::AssertZero(input)
            },
            Copy(input) => Gate::Copy(output, input),
            Add(left, right) => Gate::Add(output, left, right),
            Mul(left, right) => Gate::Mul(output, left, right),
            AddConstant(left, value) => Gate::AddConstant(output, left, value),
            MulConstant(left, value) => Gate::MulConstant(output, left, value),
            And(left, right) => Gate::And(output, left, right),
            Xor(left, right) => Gate::Xor(output, left, right),
            Not(input) => Gate::Not(output, input),
            Instance => Gate::Instance(output),
            Witness => Gate::Witness(output),
        }
    }

    pub fn has_output(&self) -> bool {
        match *self {
            AssertZero(_) => false,
            _ => true,
        }
    }
}
