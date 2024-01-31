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
    { return 4 & 12 ^ 13 | 1 && (g_GlobalVariable + 2u << 2 & 31) * (((34l + _g_AssumeABoolean == g_NiceVar >> 3) * 34) * 4) * 23.6f - 45 >= 25 != 0; }
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
├╼ FunctionDefinition <117..517>
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
│   └╼ CompoundStatement <138..513>
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
│     └╼ CompoundStatement <370..513>
│       └╼ ReturnStatement <370..513>
│         └╼ BinaryOperatorExpression <370..513>
│           ├╼ Operator -> LogicalAnd <393..395>
│           ├╼ LHS
│           │ └╼ BinaryOperatorExpression <377..392>
│           │   ├╼ Operator -> BitwiseOr <389..390>
│           │   ├╼ LHS
│           │   │ └╼ BinaryOperatorExpression <377..388>
│           │   │   ├╼ Operator -> BitwiseXor <384..385>
│           │   │   ├╼ LHS
│           │   │   │ └╼ BinaryOperatorExpression <377..383>
│           │   │   │   ├╼ Operator -> BitwiseAnd <379..380>
│           │   │   │   ├╼ LHS
│           │   │   │   │ └╼ Constant
│           │   │   │   │   └╼ Integer -> Generic(4), <377..378>
│           │   │   │   └╼ RHS
│           │   │   │     └╼ Constant
│           │   │   │       └╼ Integer -> Generic(12), <381..383>
│           │   │   └╼ RHS
│           │   │     └╼ Constant
│           │   │       └╼ Integer -> Generic(13), <386..388>
│           │   └╼ RHS
│           │     └╼ Constant
│           │       └╼ Integer -> Generic(1), <391..392>
│           └╼ RHS
│             └╼ BinaryOperatorExpression <397..512>
│               ├╼ Operator -> NotEquals <508..510>
│               ├╼ LHS
│               │ └╼ BinaryOperatorExpression <397..507>
│               │   ├╼ Operator -> GreaterOrEqual <502..504>
│               │   ├╼ LHS
│               │   │ └╼ BinaryOperatorExpression <397..501>
│               │   │   ├╼ Operator -> Minus <497..498>
│               │   │   ├╼ LHS
│               │   │   │ └╼ BinaryOperatorExpression <397..496>
│               │   │   │   ├╼ Operator -> Multiply <489..490>
│               │   │   │   ├╼ LHS
│               │   │   │   │ └╼ BinaryOperatorExpression <397..487>
│               │   │   │   │   ├╼ Operator -> Multiply <430..431>
│               │   │   │   │   ├╼ LHS
│               │   │   │   │   │ └╼ BinaryOperatorExpression <397..428>
│               │   │   │   │   │   ├╼ Operator -> BitwiseAnd <424..425>
│               │   │   │   │   │   ├╼ LHS
│               │   │   │   │   │   │ └╼ BinaryOperatorExpression <397..423>
│               │   │   │   │   │   │   ├╼ Operator -> ShiftLeft <419..421>
│               │   │   │   │   │   │   ├╼ LHS
│               │   │   │   │   │   │   │ └╼ BinaryOperatorExpression <397..418>
│               │   │   │   │   │   │   │   ├╼ Operator -> Plus <414..415>
│               │   │   │   │   │   │   │   ├╼ LHS
│               │   │   │   │   │   │   │   │ └╼ Identifier -> "g_GlobalVariable" <397..413>
│               │   │   │   │   │   │   │   └╼ RHS
│               │   │   │   │   │   │   │     └╼ Constant
│               │   │   │   │   │   │   │       └╼ Integer -> Unsigned(2), <416..418>
│               │   │   │   │   │   │   └╼ RHS
│               │   │   │   │   │   │     └╼ Constant
│               │   │   │   │   │   │       └╼ Integer -> Generic(2), <422..423>
│               │   │   │   │   │   └╼ RHS
│               │   │   │   │   │     └╼ Constant
│               │   │   │   │   │       └╼ Integer -> Generic(31), <426..428>
│               │   │   │   │   └╼ RHS
│               │   │   │   │     └╼ BinaryOperatorExpression <435..487>
│               │   │   │   │       ├╼ Operator -> Multiply <484..485>
│               │   │   │   │       ├╼ LHS
│               │   │   │   │       │ └╼ BinaryOperatorExpression <435..482>
│               │   │   │   │       │   ├╼ Operator -> Multiply <478..479>
│               │   │   │   │       │   ├╼ LHS
│               │   │   │   │       │   │ └╼ BinaryOperatorExpression <435..476>
│               │   │   │   │       │   │   ├╼ Operator -> Equals <459..461>
│               │   │   │   │       │   │   ├╼ LHS
│               │   │   │   │       │   │   │ └╼ BinaryOperatorExpression <435..458>
│               │   │   │   │       │   │   │   ├╼ Operator -> Plus <439..440>
│               │   │   │   │       │   │   │   ├╼ LHS
│               │   │   │   │       │   │   │   │ └╼ Constant
│               │   │   │   │       │   │   │   │   └╼ Integer -> SignedLong(34), <435..438>
│               │   │   │   │       │   │   │   └╼ RHS
│               │   │   │   │       │   │   │     └╼ Identifier -> "_g_AssumeABoolean" <441..458>
│               │   │   │   │       │   │   └╼ RHS
│               │   │   │   │       │   │     └╼ BinaryOperatorExpression <462..476>
│               │   │   │   │       │   │       ├╼ Operator -> ShiftRight <472..474>
│               │   │   │   │       │   │       ├╼ LHS
│               │   │   │   │       │   │       │ └╼ Identifier -> "g_NiceVar" <462..471>
│               │   │   │   │       │   │       └╼ RHS
│               │   │   │   │       │   │         └╼ Constant
│               │   │   │   │       │   │           └╼ Integer -> Generic(3), <475..476>
│               │   │   │   │       │   └╼ RHS
│               │   │   │   │       │     └╼ Constant
│               │   │   │   │       │       └╼ Integer -> Generic(34), <480..482>
│               │   │   │   │       └╼ RHS
│               │   │   │   │         └╼ Constant
│               │   │   │   │           └╼ Integer -> Generic(4), <486..487>
│               │   │   │   └╼ RHS
│               │   │   │     └╼ Constant
│               │   │   │       └╼ Float -> Float(23.6) <491..496>
│               │   │   └╼ RHS
│               │   │     └╼ Constant
│               │   │       └╼ Integer -> Generic(45), <499..501>
│               │   └╼ RHS
│               │     └╼ Constant
│               │       └╼ Integer -> Generic(25), <505..507>
│               └╼ RHS
│                 └╼ Constant
│                   └╼ Integer -> Generic(0), <511..512>
├╼ FunctionDefinition <519..544>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ ├╼ FunctionSpecifier -> Inline <519..525>
│ │ │ └╼ TypeSpecifier -> Float <526..531>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "AdityaG" <532..539>
│ │   └╼ FunctionParameters
│ │     └╼ Empty
│ └╼ FunctionBody
│   └╼ Empty
├╼ Declaration <545..600>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ TypeQualifier -> Const <545..550>
│ │ └╼ TypeSpecifier -> Int <551..554>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ FunctionDeclarator
│       ├╼ Identifier -> "NiceFunction" <555..567>
│       └╼ FunctionParameters
│         ├╼ FunctionParameter <568..584>
│         │ ├╼ DeclarationSpecifiers
│         │ │ ├╼ TypeQualifier -> Const <568..573>
│         │ │ └╼ TypeSpecifier -> Int <574..577>
│         │ └╼ DirectDeclarator -> "param1" <578..584>
│         └╼ FunctionParameter <586..598>
│           ├╼ DeclarationSpecifiers
│           │ └╼ TypeSpecifier -> Float <586..591>
│           └╼ DirectDeclarator -> "param2" <592..598>
└╼ Declaration <601..649>
  ├╼ DeclarationSpecifiers
  │ ├╼ TypeSpecifier -> Unsigned <601..609>
  │ └╼ TypeSpecifier -> Int <610..613>
  └╼ InitDeclaratorList
    └╼ InitDeclarator
      └╼ FunctionDeclarator
        ├╼ Identifier -> "GetSum" <614..620>
        └╼ FunctionParameters
          ├╼ FunctionParameter <621..634>
          │ ├╼ DeclarationSpecifiers
          │ │ ├╼ TypeSpecifier -> Unsigned <621..629>
          │ │ └╼ TypeSpecifier -> Int <630..633>
          │ └╼ DirectDeclarator -> None
          └╼ FunctionParameter <635..648>
            ├╼ DeclarationSpecifiers
            │ ├╼ TypeSpecifier -> Unsigned <635..643>
            │ └╼ TypeSpecifier -> Int <644..647>
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
