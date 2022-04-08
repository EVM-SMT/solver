// SPDX-License-Identifier: GPL-3.0-only

use solver::parser;

use std::env;

fn main() {
    let file_to_convert = env::args()
        .nth(1)
        .expect("Expected SMTLIB file with file extension");
    let problem = parser::parse_from_file(file_to_convert).unwrap();

    println!("{}", problem);
}
