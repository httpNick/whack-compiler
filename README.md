# Whack Language

`whack` is a "dumb" but fully functional programming language and compiler built to explore the deep magic of language design, Pratt Parsing, and code generation in Rust.

## Features

- **Lexical Analysis**: A hand-written scanner that turns raw text into a stream of tokens.
- **Recursive Descent + Pratt Parsing**: Implements top-down operator precedence to handle math. No more messy parentheses-only math!
- **Dual Pipeline**:
  - **Interpreter**: A tree-walk evaluator for instant execution.
  - **Transpiler**: Compiles Whack code directly into valid C, then lets `gcc` finish the job.
- **Support for**: Variables (`let`), Basic Arithmetic (`+`, `-`, `*`, `/`, `%`), and Parenthesized groupings.

## Quick Start

### 1. Write some Whack code
Create a `test.whack` file:
```whack
let a = 10;
let b = 2;
let c = (a + b) * 5;
print c;
print c % 7;
```

### 2. Run the Compiler
```bash
cargo run
```
This will interpret the code directly and also generate an `output.c` file.

### 3. Compile and Run the C Binary
```bash
gcc output.c -o whack_app
./whack_app
```

## Technical Details

Whack uses a **Pratt Parser** (Top Down Operator Precedence) to handle expression parsing. This allows for clean handling of operator precedence (`5 + 2 * 10` correctly results in `25`) without a massive, nested grammar.

### The "Machine"
Variables in Whack are mapped to C `long` types (`i64` in Rust). The transpiler emits C boilerplate including `#include <stdio.h>` and a `main` function wrapper, making the final binary remarkably lightweight.
