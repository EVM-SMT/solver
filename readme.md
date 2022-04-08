# solver

This is a shared project from a few groups working on tooling for verification of ethereum smart contracts.

The aim is to build out a new SMT solver focused on efficient solving for the kind of queries
commonly encountered when modelling smart contracts in smt. Essentially this means we want to
optimize for non linear arithmetic over 256 bit bitvectors. We additionally aim to support
uninterpreted functions and arrays. Support for other theories is not currently planned.

We believe that starting a new solver instead of extending an existing solver has the following
benefits:

- A smaller domain focused solver should be able to achieve better performance when compared to a
    larger and more general system
- Adding proofs to an existing solver can be a complex task. A new solver can be built from the
    ground up with efficient proof generation as a core design goal. The restricted set of theories is
    also helpful here.
- Existing solvers make heavy use of bit-blasting when solving bit vector problems, while this works
    well for small bv instances, it does not scale to the 256bit needed when analysing evm programs. We
    intend to research and apply techniques more appropriate to very large bv instances (even at
    the expense of performance on small bv instances)
- We can use a more modern toolchain (rust > c/c++)

## Development

You need cargo 1.59 on your `PATH`. For nix users a `shell.nix` is provided that contains all
required development dependencies.

```
cargo build # build
cargo run   # run
```
