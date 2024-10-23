# Fuzzer

Simple program to generate random LISP-style SMT expressions and write them to a file

Usage: fuzzer [OPTIONS] --file <FILE>

Options:
-   -f, --file <FILE>      File path to write to
-   -n, --number <NUMBER>  Number of LISP-style SMT expressions [default: 10]
-   -h, --help             Print help
-   -V, --version          Print version

## Build instructions:
-   Run: "cargo build" as usual for rust
