// SPDX-License-Identifier: GPL-3.0-only
//! Post typechecking AST, we only support QF_EUF at the moment

pub struct Problem(Vec<Term>);

pub enum Term {
    O(Op),
    C(Command),
}

pub enum Op {
    LitBool(bool),
    VarBool(String),

    Eq(Vec<Op>),
    And(Vec<Op>),
    Or(Vec<Op>),
    XOr(Vec<Op>),
    Impl(Vec<Op>),
    Distinct(Vec<Op>),
    ITE(Box<Op>, Box<Op>, Box<Op>),
}

pub enum Command {
    Assert(Op),
    CheckSat(),
    GetModel(),
}
