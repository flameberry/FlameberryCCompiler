# Flameberry C Compiler (Written in Rust)
Flameberry C compiler is primitive C (ISO 17 standard compliant) compiler which is in early stages and the future plan is to make it a fully featured compiler with decent performance.

**Currently it supports:**
1. Preprocessing comments
2. Lexing (almost) every kind of C token
3. Parsing of:
    1. External Declarations
    2. Function Definitions
    3. Return Statements
    4. Compound Statements
    5. If Statements
    6. Break and Continue Statements
    7. Expressions
4. Generation of basic Abstract Syntax Tree for the C Translation Unit

## Demo
For the following Sandbox/test.c program, the generated AST is:
```C
static int g_GlobalVariable;
static int g_NiceVar;
static _Bool _g_AssumeABoolean;

int nice = 69, wow(), good = 3;

int main(void) {
    if (_g_AssumeABoolean + 3 < 4)
    {
        char ch = 'A';
        return ch;
    }
    else if (g_NiceVar <= 44)
        return 69l;
    else if (g_GlobalVariable > 3)
        return 12ul;
    else
        return 0;

    {}
    { return 4 & (g_GlobalVariable + 2u << 2 & 31) * (((34l + _g_AssumeABoolean == g_NiceVar >> 3) * 34) * 4) * 23.6f - 45 >= 25 != 0; }
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
├╼ FunctionDefinition <117..502>
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
│   └╼ CompoundStatement <138..498>
│     ├╼ IfStatement <138..355>
│     │ ├╼ IfExpression
│     │ │ └╼ BinaryOperatorExpression <142..167>
│     │ │   ├╼ Operator -> Less <164..165>
│     │ │   ├╼ LHS
│     │ │   │ └╼ BinaryOperatorExpression <142..163>
│     │ │   │   ├╼ Operator -> Plus <160..161>
│     │ │   │   ├╼ LHS
│     │ │   │   │ └╼ Identifier -> "_g_AssumeABoolean" <142..159>
│     │ │   │   └╼ RHS
│     │ │   │     └╼ Constant
│     │ │   │       └╼ Integer -> Generic(3), <162..163>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(4), <166..167>
│     │ ├╼ ThenStatement
│     │ │ └╼ CompoundStatement <183..216>
│     │ │   ├╼ Declaration <183..197>
│     │ │   │ ├╼ DeclarationSpecifiers
│     │ │   │ │ └╼ TypeSpecifier -> Char <183..187>
│     │ │   │ └╼ InitDeclaratorList
│     │ │   │   └╼ InitDeclarator
│     │ │   │     ├╼ DirectDeclarator -> "ch" <188..190>
│     │ │   │     └╼ Initializer
│     │ │   │       └╼ Constant
│     │ │   │         └╼ Character -> 'A'
│     │ │   └╼ ReturnStatement <206..216>
│     │ │     └╼ Identifier -> "ch" <206..216>
│     │ └╼ ElseStatement
│     │   └╼ IfStatement <232..355>
│     │     ├╼ IfExpression
│     │     │ └╼ BinaryOperatorExpression <236..251>
│     │     │   ├╼ Operator -> LessOrEqual <246..248>
│     │     │   ├╼ LHS
│     │     │   │ └╼ Identifier -> "g_NiceVar" <236..245>
│     │     │   └╼ RHS
│     │     │     └╼ Constant
│     │     │       └╼ Integer -> Generic(44), <249..251>
│     │     ├╼ ThenStatement
│     │     │ └╼ ReturnStatement <261..272>
│     │     │   └╼ Constant
│     │     │     └╼ Integer -> SignedLong(69), <261..272>
│     │     └╼ ElseStatement
│     │       └╼ IfStatement <282..355>
│     │         ├╼ IfExpression
│     │         │ └╼ BinaryOperatorExpression <286..306>
│     │         │   ├╼ Operator -> Greater <303..304>
│     │         │   ├╼ LHS
│     │         │   │ └╼ Identifier -> "g_GlobalVariable" <286..302>
│     │         │   └╼ RHS
│     │         │     └╼ Constant
│     │         │       └╼ Integer -> Generic(3), <305..306>
│     │         ├╼ ThenStatement
│     │         │ └╼ ReturnStatement <316..328>
│     │         │   └╼ Constant
│     │         │     └╼ Integer -> UnsignedLong(12), <316..328>
│     │         └╼ ElseStatement
│     │           └╼ ReturnStatement <346..355>
│     │             └╼ Constant
│     │               └╼ Integer -> Generic(0), <346..355>
│     ├╼ CompoundStatement <362..362>
│     │ └╼ Empty
│     └╼ CompoundStatement <370..498>
│       └╼ ReturnStatement <370..498>
│         └╼ BinaryOperatorExpression <370..498>
│           ├╼ Operator -> BitwiseAnd <379..380>
│           ├╼ LHS
│           │ └╼ Constant
│           │   └╼ Integer -> Generic(4), <377..378>
│           └╼ RHS
│             └╼ BinaryOperatorExpression <382..497>
│               ├╼ Operator -> NotEquals <493..495>
│               ├╼ LHS
│               │ └╼ BinaryOperatorExpression <382..492>
│               │   ├╼ Operator -> GreaterOrEqual <487..489>
│               │   ├╼ LHS
│               │   │ └╼ BinaryOperatorExpression <382..486>
│               │   │   ├╼ Operator -> Minus <482..483>
│               │   │   ├╼ LHS
│               │   │   │ └╼ BinaryOperatorExpression <382..481>
│               │   │   │   ├╼ Operator -> Multiply <474..475>
│               │   │   │   ├╼ LHS
│               │   │   │   │ └╼ BinaryOperatorExpression <382..472>
│               │   │   │   │   ├╼ Operator -> Multiply <415..416>
│               │   │   │   │   ├╼ LHS
│               │   │   │   │   │ └╼ BinaryOperatorExpression <382..413>
│               │   │   │   │   │   ├╼ Operator -> BitwiseAnd <409..410>
│               │   │   │   │   │   ├╼ LHS
│               │   │   │   │   │   │ └╼ BinaryOperatorExpression <382..408>
│               │   │   │   │   │   │   ├╼ Operator -> ShiftLeft <404..406>
│               │   │   │   │   │   │   ├╼ LHS
│               │   │   │   │   │   │   │ └╼ BinaryOperatorExpression <382..403>
│               │   │   │   │   │   │   │   ├╼ Operator -> Plus <399..400>
│               │   │   │   │   │   │   │   ├╼ LHS
│               │   │   │   │   │   │   │   │ └╼ Identifier -> "g_GlobalVariable" <382..398>
│               │   │   │   │   │   │   │   └╼ RHS
│               │   │   │   │   │   │   │     └╼ Constant
│               │   │   │   │   │   │   │       └╼ Integer -> Unsigned(2), <401..403>
│               │   │   │   │   │   │   └╼ RHS
│               │   │   │   │   │   │     └╼ Constant
│               │   │   │   │   │   │       └╼ Integer -> Generic(2), <407..408>
│               │   │   │   │   │   └╼ RHS
│               │   │   │   │   │     └╼ Constant
│               │   │   │   │   │       └╼ Integer -> Generic(31), <411..413>
│               │   │   │   │   └╼ RHS
│               │   │   │   │     └╼ BinaryOperatorExpression <420..472>
│               │   │   │   │       ├╼ Operator -> Multiply <469..470>
│               │   │   │   │       ├╼ LHS
│               │   │   │   │       │ └╼ BinaryOperatorExpression <420..467>
│               │   │   │   │       │   ├╼ Operator -> Multiply <463..464>
│               │   │   │   │       │   ├╼ LHS
│               │   │   │   │       │   │ └╼ BinaryOperatorExpression <420..461>
│               │   │   │   │       │   │   ├╼ Operator -> Equals <444..446>
│               │   │   │   │       │   │   ├╼ LHS
│               │   │   │   │       │   │   │ └╼ BinaryOperatorExpression <420..443>
│               │   │   │   │       │   │   │   ├╼ Operator -> Plus <424..425>
│               │   │   │   │       │   │   │   ├╼ LHS
│               │   │   │   │       │   │   │   │ └╼ Constant
│               │   │   │   │       │   │   │   │   └╼ Integer -> SignedLong(34), <420..423>
│               │   │   │   │       │   │   │   └╼ RHS
│               │   │   │   │       │   │   │     └╼ Identifier -> "_g_AssumeABoolean" <426..443>
│               │   │   │   │       │   │   └╼ RHS
│               │   │   │   │       │   │     └╼ BinaryOperatorExpression <447..461>
│               │   │   │   │       │   │       ├╼ Operator -> ShiftRight <457..459>
│               │   │   │   │       │   │       ├╼ LHS
│               │   │   │   │       │   │       │ └╼ Identifier -> "g_NiceVar" <447..456>
│               │   │   │   │       │   │       └╼ RHS
│               │   │   │   │       │   │         └╼ Constant
│               │   │   │   │       │   │           └╼ Integer -> Generic(3), <460..461>
│               │   │   │   │       │   └╼ RHS
│               │   │   │   │       │     └╼ Constant
│               │   │   │   │       │       └╼ Integer -> Generic(34), <465..467>
│               │   │   │   │       └╼ RHS
│               │   │   │   │         └╼ Constant
│               │   │   │   │           └╼ Integer -> Generic(4), <471..472>
│               │   │   │   └╼ RHS
│               │   │   │     └╼ Constant
│               │   │   │       └╼ Float -> Float(23.6) <476..481>
│               │   │   └╼ RHS
│               │   │     └╼ Constant
│               │   │       └╼ Integer -> Generic(45), <484..486>
│               │   └╼ RHS
│               │     └╼ Constant
│               │       └╼ Integer -> Generic(25), <490..492>
│               └╼ RHS
│                 └╼ Constant
│                   └╼ Integer -> Generic(0), <496..497>
├╼ FunctionDefinition <504..529>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ ├╼ FunctionSpecifier -> Inline <504..510>
│ │ │ └╼ TypeSpecifier -> Float <511..516>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "AdityaG" <517..524>
│ │   └╼ FunctionParameters
│ │     └╼ Empty
│ └╼ FunctionBody
│   └╼ Empty
├╼ Declaration <530..585>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ TypeQualifier -> Const <530..535>
│ │ └╼ TypeSpecifier -> Int <536..539>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ FunctionDeclarator
│       ├╼ Identifier -> "NiceFunction" <540..552>
│       └╼ FunctionParameters
│         ├╼ FunctionParameter <553..569>
│         │ ├╼ DeclarationSpecifiers
│         │ │ ├╼ TypeQualifier -> Const <553..558>
│         │ │ └╼ TypeSpecifier -> Int <559..562>
│         │ └╼ DirectDeclarator -> "param1" <563..569>
│         └╼ FunctionParameter <571..583>
│           ├╼ DeclarationSpecifiers
│           │ └╼ TypeSpecifier -> Float <571..576>
│           └╼ DirectDeclarator -> "param2" <577..583>
└╼ Declaration <586..634>
  ├╼ DeclarationSpecifiers
  │ ├╼ TypeSpecifier -> Unsigned <586..594>
  │ └╼ TypeSpecifier -> Int <595..598>
  └╼ InitDeclaratorList
    └╼ InitDeclarator
      └╼ FunctionDeclarator
        ├╼ Identifier -> "GetSum" <599..605>
        └╼ FunctionParameters
          ├╼ FunctionParameter <606..619>
          │ ├╼ DeclarationSpecifiers
          │ │ ├╼ TypeSpecifier -> Unsigned <606..614>
          │ │ └╼ TypeSpecifier -> Int <615..618>
          │ └╼ DirectDeclarator -> None
          └╼ FunctionParameter <620..633>
            ├╼ DeclarationSpecifiers
            │ ├╼ TypeSpecifier -> Unsigned <620..628>
            │ └╼ TypeSpecifier -> Int <629..632>
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
