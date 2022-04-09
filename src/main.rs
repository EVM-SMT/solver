// SPDX-License-Identifier: MIT

use std::env;

fn main() {
    let file_to_convert = env::args()
        .nth(1)
        .expect("Expected SMTLIB file with file extension");
    println!("{}", file_to_convert);
}
