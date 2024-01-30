# Flameberry C Compiler (Written in Rust)
Flameberry C compiler is primitive C (ISO 17 standard compliant) compiler which is in early stages and the future plan is to make it a fully featured compiler with decent performance.

**Currently it supports:**
1. Preprocessing comments
2. Lexing (almost) every kind of C token
3. Parsing of:
    1. External Declarations
    2. Function Definitions
    3. Return Statements
    3. Compound Statements
    3. If Statements
    3. Break and Continue Statements
    4. Additive Expressions
    5. Multiplicative Expressions
    5. Primary Expressions
4. Generation of basic Abstract Syntax Tree for the C Translation Unit

## Demo
For the following Sandbox/test.c program, the generated AST is:
```C
static int g_GlobalVariable;
static int g_NiceVar;
static _Bool _g_AssumeABoolean;

int nice = 69, wow(), good = 3;

int main(void) {
    if (_g_AssumeABoolean)
    {
        char ch = 'A';
        return ch;
    }
    else if (g_NiceVar)
        return 69l;
    else if (g_GlobalVariable)
        return 12ul;
    else
        return 0;

    {}
    { return (g_GlobalVariable + 2u) * (((34l + g_NiceVar) * 34) * 4) * 23.6f - 45; }
}

inline float AdityaG() {}
const int NiceFunction(const int param1, float param2);
unsigned int GetSum(unsigned int, unsigned int);
```

```
TranslationUnit
├╼ Declaration <0..28>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ StorageClassSpecifier -> Static <0..6>
│ │ └╼ TypeSpecifier -> Int <7..10>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ DirectDeclarator -> "g_GlobalVariable" <11..27>
├╼ Declaration <29..50>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ StorageClassSpecifier -> Static <29..35>
│ │ └╼ TypeSpecifier -> Int <36..39>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ DirectDeclarator -> "g_NiceVar" <40..49>
├╼ Declaration <51..82>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ StorageClassSpecifier -> Static <51..57>
│ │ └╼ TypeSpecifier -> Bool <58..63>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ DirectDeclarator -> "_g_AssumeABoolean" <64..81>
├╼ Declaration <84..115>
│ ├╼ DeclarationSpecifiers
│ │ └╼ TypeSpecifier -> Int <84..87>
│ └╼ InitDeclaratorList
│   ├╼ InitDeclarator
│   │ ├╼ DirectDeclarator -> "nice" <88..92>
│   │ └╼ Initializer
│   │   └╼ Constant
│   │     └╼ Integer -> Generic(69), <95..97>
│   ├╼ InitDeclarator
│   │ └╼ FunctionDeclarator
│   │   ├╼ Identifier -> "wow" <99..102>
│   │   └╼ FunctionParameters
│   │     └╼ Empty
│   └╼ InitDeclarator
│     ├╼ DirectDeclarator -> "good" <106..110>
│     └╼ Initializer
│       └╼ Constant
│         └╼ Integer -> Generic(3), <113..114>
├╼ FunctionDefinition <117..435>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ └╼ TypeSpecifier -> Int <117..120>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "main" <121..125>
│ │   └╼ FunctionParameters
│ │     └╼ FunctionParameter <126..131>
│ │       ├╼ DeclarationSpecifiers
│ │       │ └╼ TypeSpecifier -> Void <126..130>
│ │       └╼ DirectDeclarator -> None
│ └╼ FunctionBody
│   └╼ CompoundStatement <138..431>
│     ├╼ IfStatement <138..337>
│     │ ├╼ IfExpression
│     │ │ └╼ Identifier -> "_g_AssumeABoolean" <142..159>
│     │ ├╼ ThenStatement
│     │ │ └╼ CompoundStatement <175..208>
│     │ │   ├╼ Declaration <175..189>
│     │ │   │ ├╼ DeclarationSpecifiers
│     │ │   │ │ └╼ TypeSpecifier -> Char <175..179>
│     │ │   │ └╼ InitDeclaratorList
│     │ │   │   └╼ InitDeclarator
│     │ │   │     ├╼ DirectDeclarator -> "ch" <180..182>
│     │ │   │     └╼ Initializer
│     │ │   │       └╼ Constant
│     │ │   │         └╼ Character -> 'A'
│     │ │   └╼ ReturnStatement <198..208>
│     │ │     └╼ Identifier -> "ch" <198..208>
│     │ └╼ ElseStatement
│     │   └╼ IfStatement <224..337>
│     │     ├╼ IfExpression
│     │     │ └╼ Identifier -> "g_NiceVar" <228..237>
│     │     ├╼ ThenStatement
│     │     │ └╼ ReturnStatement <247..258>
│     │     │   └╼ Constant
│     │     │     └╼ Integer -> SignedLong(69), <247..258>
│     │     └╼ ElseStatement
│     │       └╼ IfStatement <268..337>
│     │         ├╼ IfExpression
│     │         │ └╼ Identifier -> "g_GlobalVariable" <272..288>
│     │         ├╼ ThenStatement
│     │         │ └╼ ReturnStatement <298..310>
│     │         │   └╼ Constant
│     │         │     └╼ Integer -> UnsignedLong(12), <298..310>
│     │         └╼ ElseStatement
│     │           └╼ ReturnStatement <328..337>
│     │             └╼ Constant
│     │               └╼ Integer -> Generic(0), <328..337>
│     ├╼ CompoundStatement <346..346>
│     │ └╼ Empty
│     └╼ CompoundStatement <354..431>
│       └╼ ReturnStatement <354..431>
│         └╼ BinaryOperatorExpression <354..431>
│           ├╼ Operator -> Minus <426..427>
│           ├╼ LHS
│           │ └╼ BinaryOperatorExpression <362..425>
│           │   ├╼ Operator -> Multiply <418..419>
│           │   ├╼ LHS
│           │   │ └╼ BinaryOperatorExpression <362..416>
│           │   │   ├╼ Operator -> Multiply <385..386>
│           │   │   ├╼ LHS
│           │   │   │ └╼ BinaryOperatorExpression <362..383>
│           │   │   │   ├╼ Operator -> Plus <379..380>
│           │   │   │   ├╼ LHS
│           │   │   │   │ └╼ Identifier -> "g_GlobalVariable" <362..378>
│           │   │   │   └╼ RHS
│           │   │   │     └╼ Constant
│           │   │   │       └╼ Integer -> Unsigned(2), <381..383>
│           │   │   └╼ RHS
│           │   │     └╼ BinaryOperatorExpression <390..416>
│           │   │       ├╼ Operator -> Multiply <413..414>
│           │   │       ├╼ LHS
│           │   │       │ └╼ BinaryOperatorExpression <390..411>
│           │   │       │   ├╼ Operator -> Multiply <407..408>
│           │   │       │   ├╼ LHS
│           │   │       │   │ └╼ BinaryOperatorExpression <390..405>
│           │   │       │   │   ├╼ Operator -> Plus <394..395>
│           │   │       │   │   ├╼ LHS
│           │   │       │   │   │ └╼ Constant
│           │   │       │   │   │   └╼ Integer -> SignedLong(34), <390..393>
│           │   │       │   │   └╼ RHS
│           │   │       │   │     └╼ Identifier -> "g_NiceVar" <396..405>
│           │   │       │   └╼ RHS
│           │   │       │     └╼ Constant
│           │   │       │       └╼ Integer -> Generic(34), <409..411>
│           │   │       └╼ RHS
│           │   │         └╼ Constant
│           │   │           └╼ Integer -> Generic(4), <415..416>
│           │   └╼ RHS
│           │     └╼ Constant
│           │       └╼ Float -> Float(23.6) <420..425>
│           └╼ RHS
│             └╼ Constant
│               └╼ Integer -> Generic(45), <428..430>
├╼ FunctionDefinition <437..462>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ ├╼ FunctionSpecifier -> Inline <437..443>
│ │ │ └╼ TypeSpecifier -> Float <444..449>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "AdityaG" <450..457>
│ │   └╼ FunctionParameters
│ │     └╼ Empty
│ └╼ FunctionBody
│   └╼ Empty
├╼ Declaration <463..518>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ TypeQualifier -> Const <463..468>
│ │ └╼ TypeSpecifier -> Int <469..472>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ FunctionDeclarator
│       ├╼ Identifier -> "NiceFunction" <473..485>
│       └╼ FunctionParameters
│         ├╼ FunctionParameter <486..502>
│         │ ├╼ DeclarationSpecifiers
│         │ │ ├╼ TypeQualifier -> Const <486..491>
│         │ │ └╼ TypeSpecifier -> Int <492..495>
│         │ └╼ DirectDeclarator -> "param1" <496..502>
│         └╼ FunctionParameter <504..516>
│           ├╼ DeclarationSpecifiers
│           │ └╼ TypeSpecifier -> Float <504..509>
│           └╼ DirectDeclarator -> "param2" <510..516>
└╼ Declaration <519..567>
  ├╼ DeclarationSpecifiers
  │ ├╼ TypeSpecifier -> Unsigned <519..527>
  │ └╼ TypeSpecifier -> Int <528..531>
  └╼ InitDeclaratorList
    └╼ InitDeclarator
      └╼ FunctionDeclarator
        ├╼ Identifier -> "GetSum" <532..538>
        └╼ FunctionParameters
          ├╼ FunctionParameter <539..552>
          │ ├╼ DeclarationSpecifiers
          │ │ ├╼ TypeSpecifier -> Unsigned <539..547>
          │ │ └╼ TypeSpecifier -> Int <548..551>
          │ └╼ DirectDeclarator -> None
          └╼ FunctionParameter <553..566>
            ├╼ DeclarationSpecifiers
            │ ├╼ TypeSpecifier -> Unsigned <553..561>
            │ └╼ TypeSpecifier -> Int <562..565>
            └╼ DirectDeclarator -> None
```

## Getting Started

To build the rust project:

```sh
cargo build
```

Before running the main.rs, don't forget to set the path where your C programs are located (_This will be abstracted in the future and a script will be provided to do the same instead of needing to modify the rust program itself_):

```rust
fn main() {
    let testpath = "<path/to/parent/directory/containing/c/programs>";
    run_tests(testpath);
}
```

Or if you want to test using a single file:

```rust
fn main() {
    let test_path = "Sandbox/test.c";
    compile_file(test_path);
}
```

To run the project:

```sh
cargo run
```

_Note: The repository currently uses test programs present in the repository https://github.com/nlsandler/writing-a-c-compiler-tests to check the output._
