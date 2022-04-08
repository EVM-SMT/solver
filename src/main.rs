// SPDX-License-Identifier: GPL-3.0-only

use amzn_smt_ir::{Script,Term,Ctx,Logic,UnknownSort,Command};
use solver::parser;
use solver::sorts;

use std::env;
use std::fmt;

fn main() {
    let file_to_convert = env::args()
        .nth(1)
        .expect("Expected SMTLIB file with file extension");

    // parse input file
    let parsed = parser::parse_from_file(file_to_convert).unwrap();

    // typecheck input
    let mut ctx = Ctx::default();
    let sort_checker = SortChecker(&mut ctx);
    let typed = parsed.try_fold(sort_checker).unwrap();

    println!("{}", typed);
}
