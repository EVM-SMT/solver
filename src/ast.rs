// SPDX-License-Identifier: GPL-3.0-only
//! Post typechecking AST, we only support QF_EUF at the moment

pub struct Problem(Vec<Command>);

pub enum Command {
    Assert(BoolOp),
    DeclareFun(String, Vec<Atom>, Atom),
    DeclareVar(String, Atom),
    CheckSat(),
    GetModel(),
}

pub enum Atom {
    Bool(),
    BV(),
}

pub enum PolyOp<T> {
    Eq(Vec<T>),
    ITE(Box<BoolOp>, Box<T>, Box<T>),
}

pub enum BVOp {
    Shr(Box<BVOp>, Box<BVOp>),
    Shl(Box<BVOp>, Box<BVOp>),
    Concat(Box<BVOp>, Box<BVOp>),
    Extract(usize, usize),
    BVNot(Box<BVOp>),
    BVNeg(Box<BVOp>),
    // ... more to come
}

pub enum BoolOp {
    LitBool(bool),
    VarBool(String),

    Not(Box<BoolOp>),
    And(Vec<BoolOp>),
    Or(Vec<BoolOp>),
    XOr(Vec<BoolOp>),
    Impl(Vec<BoolOp>),
    Distinct(Vec<BoolOp>),
}

