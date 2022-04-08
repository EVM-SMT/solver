// SPDX-License-Identifier: GPL-3.0-only
//! Implements a typechecker for the pasrsed smt2 AST

use amzn_smt_ir::fold::{Fold, Folder, SuperFold};
use amzn_smt_ir::{CoreOp::*, ICoreOp, IVar, IUF, IOp, Term, ILet, IMatch, Logic, Void, IConst, IQuantifier};
use amzn_smt_ir::{Script,Ctx,Sorted,ISort,Sort,Identifier};
use std::fmt;

// --- newtype wrapper for typechecked ast instances ---

#[derive(Clone, Default, Debug, Hash, PartialEq, Eq)]
pub struct Typed<L:Logic>(Script<Term<L>>);

impl <L:Logic> fmt::Display for Typed<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- typechecker ---

#[derive(Debug, PartialEq, Eq)]
pub struct TypeError<L: Logic>(Term<L>);

#[derive(Debug)]
pub struct SortChecker<'a>(&'a mut Ctx);

// for each term recurse into the tree and at each node:
//   -- infer the types of the arguments
//   -- check that the arguments are of the type expected by the op
impl<L: Logic> Folder<L> for SortChecker<'_> {
    type Output = Term<L>;
    type Error = TypeError<L>;

    fn fold_const(&mut self, constant: IConst) -> Result<Self::Output, Self::Error> {
        Err(TypeError(constant.into()))
    }

    fn fold_var(&mut self, var: IVar<L::Var>) -> Result<Self::Output, Self::Error> {
        unimplemented!();
    }

    fn fold_core_op(&mut self, op: ICoreOp<L>) -> Result<Self::Output, Self::Error> {
        match op.super_fold_with(self)? {
            True => Ok(op.into()),
            False => Ok(op.into()),

            Not(term) => match term.sort(self.0) {
                Ok(amzn_smt_ir::ISort::Simple { identifier }) => match identifier.as_ref() {
                    Identifier::Simple { symbol } => {
                        match symbol.as_ref().0.as_ref() {
                            "Bool" => Ok(op.into()),
                            _ => Err(TypeError(op.into())),
                        }
                    }
                    _ => Err(TypeError(op.into())),
                },
                _ => Err(TypeError(op.into())),
            },

            And(args) => unimplemented!(),

            Or(args) => unimplemented!(),

            Xor(args) => unimplemented!(),

            Imp(args) => unimplemented!(),

            Eq(args) => unimplemented!(),

            Distinct(args) => unimplemented!(),

            Ite(cond, l, r) => unimplemented!(),
        }
    }

    fn fold_theory_op(&mut self, op: IOp<L>) -> Result<Self::Output, Self::Error> {
        Err(TypeError(op.into()))
    }

    fn fold_uninterpreted_func(&mut self, uf: IUF<L>) -> Result<Self::Output, Self::Error> {
        unimplemented!();
    }

    fn fold_let(&mut self, l: ILet<L>) -> Result<Self::Output, Self::Error> {
        Err(TypeError(l.into()))
    }

    fn fold_match(&mut self, m: IMatch<L>) -> Result<Self::Output, Self::Error> {
        Err(TypeError(m.into()))
    }

    fn fold_quantifier(&mut self, quantifier: IQuantifier<L>) -> Result<Self::Output, Self::Error> {
        Err(TypeError(quantifier.into()))
    }
}

//let f1: Term = !Term::CoreOp(true.into());
//let f2 = f1.clone() & !Term::CoreOp(false.into());
//let mut folder = PartiallyEvaluateNot;
//assert_eq!(f1.fold_with(&mut folder), Ok(false.into()));
//assert_eq!(f2.fold_with(&mut folder), Ok(CoreOp::And([false.into(), true.into()].into()).into()));
