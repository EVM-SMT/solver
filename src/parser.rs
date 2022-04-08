// SPDX-License-Identifier: GPL-3.0-only
//! parses scripts in QF_EUF into an instance of the smt2 ir

use amzn_smt_ir::{ParseError, Script, Term as IRTerm};

use crate::logic::QF_EUF;

use std::{fs, io};

type Term = IRTerm<QF_EUF>;

fn parse(smtlib: impl std::io::BufRead) -> Result<Script<Term>, ParseError<QF_EUF>> {
    Script::<Term>::parse(smtlib)
}

pub fn parse_from_string(content: &str) -> Result<Script<Term>, ParseError<QF_EUF>> {
    parse(content.as_bytes())
}

pub fn parse_from_file(filename: String) -> Result<Script<Term>, ParseError<QF_EUF>> {
    let file = fs::File::open(filename);
    let file =
        file.expect("Error reading input file: did you try to read a file without extension smt2?");
    let reader = io::BufReader::new(file);

    parse(reader)
}
