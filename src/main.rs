// SPDX-License-Identifier: GPL-3.0-only

use amzn_smt_ir::{SortChecker,Ctx};
use solver::parser;

use std::env;

fn main() {
    let file_to_convert = env::args()
        .nth(1)
        .expect("Expected SMTLIB file with file extension");
    let parsed = parser::parse_from_file(file_to_convert).unwrap();

    let mut ctx = Ctx::default();
    let checker = SortChecker(&mut ctx);
    let typed = parsed.try_fold(checker);

    println!("{}", typed);
}
