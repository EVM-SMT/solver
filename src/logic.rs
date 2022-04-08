
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use amzn_smt_ir::{visit::Visit, fold::Fold, *};

#[derive(Clone, Default, Debug, Hash, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct QF_EUF;

#[derive(Clone, Operation, Fold, Visit, Hash, PartialEq, Eq)]
enum Op<Term> {
    Negative(Term),
}

impl<L: Logic> Sorted<L> for Op<Term<L>> {
    fn sort(&self, _: &mut Ctx) -> Result<ISort, UnknownSort<Term<L>>> {
        Ok(ISort::int())
    }
}

impl Logic for QF_EUF {
    type Var = QualIdentifier;
    type Op = Op<Term<Self>>;
    type UninterpretedFunc = UF<Term<Self>>;
    type Quantifier = Void;
}
