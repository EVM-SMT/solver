// SPDX-License-Identifier: GPL-3.0-only
//! Instantiation of QF_EUF for the amzn_smt_ir parser

use amzn_smt_ir::{Logic, Void, QualIdentifier, UF, Term};

#[derive(Clone, Default, Debug, Hash, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct QF_EUF;

impl Logic for QF_EUF {
    type Var = QualIdentifier;
    type Op = Void;
    type UninterpretedFunc = UF<Term<Self>>;
    type Quantifier = Void;
}
