# Flameberry C Compiler (Written in Rust)

Flameberry C compiler is primitive C (somewhat ISO 17 standard compliant) compiler which is in early stages and the future plan is to make it a fully featured compiler with decent performance.

**Currently it supports:**

1. Preprocessing comments
2. Lexing (almost) every kind of C token
3. Parsing of:
    1. External Declarations
    2. Function Definitions
    3. Statements:
        1. Labeled
        2. Compound
        3. Expression
        4. Selection
        5. Iteration
        6. Jump
    4. Expressions:
        1. Primary except Generic Associations
        2. Postfix
        3. Unary
        4. Cast
        5. Multiplicative
        6. Additive
        7. Shift
        8. Relational
        9. Equality
        10. AND
        11. Exclusive OR
        12. Inclusive OR
        13. Logical AND
        14. Logical OR
        15. Conditional
        16. Assignment
        17. Generic Expression (which contains combinations of all the above expressions)
        18. Constant Expression
4. Generation of basic Abstract Syntax Tree for the C Translation Unit
5. Semantic Analysis:
    1. Type Checking
    2. Implicit Casting
    3. Symbol Table
    4. Variable Shadowing
6. Lowering the AST to a small linear IR (three-address style, one instruction list per function)
7. ARM64 assembly generation for macOS (Apple silicon), currently covering:
    1. Functions with parameters and calls (Apple AAPCS64 calling convention), including recursion
    2. Local variables and assignment
    3. Integer arithmetic: `+` `-` `*` `/` `%`
    4. Comparisons: `<` `<=` `>` `>=` `==` `!=`
    5. `if`/`else`, `while` and `for`
    6. `return` from anywhere in a function

The backend is deliberately kept simple for now: everything is an `int`, and there is no register allocation yet — every variable and temporary lives in a stack slot. The generated assembly links with `clang` and runs natively.

## Examples

Two walkthroughs with real compiler output live in the `docs/` folder:

- [AST generation](docs/ast-generation.md) — a C program and the syntax tree `--dump-ast` prints for it
- [IR and ARM64 assembly generation](docs/ir-asm-generation.md) — `tests/test_ir.c` taken through `--dump-ir` and `--emit-asm`, down to the binary's exit code

## Getting Started

To run the command line tool:

```sh
cargo run -p cli -- --dump-ast <path/to/source/file>
```

Available options:

- `--dump-ast` — print the Abstract Syntax Tree of the program
- `--dump-ir` — print the intermediate representation each function is lowered to
- `--dump-asm` — print the generated ARM64 assembly
- `--emit-asm` — write the assembly to a `.s` file next to the input (use `-o <path>` to pick the output path)

The compiler exits with a non-zero code if compilation fails, so it plays nicely with scripts.

The quickest way to compile and run a C file end to end is the `run.sh` helper, which compiles the file, assembles and links it with `clang`, runs the binary and prints its exit code:

```sh
./run.sh tests/test_ir.c
```

The programs in `tests/backend/` are small self-contained test cases for the backend — each one states the exit code it is expected to produce in a comment at the top.