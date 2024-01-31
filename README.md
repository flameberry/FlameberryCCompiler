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
    { return 4 & 12 ^ 13 | (g_GlobalVariable + 2u << 2 & 31) * (((34l + _g_AssumeABoolean == g_NiceVar >> 3) * 34) * 4) * 23.6f - 45 >= 25 != 0; }
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
├╼ FunctionDefinition <117..512>
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
│   └╼ CompoundStatement <138..508>
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
│     └╼ CompoundStatement <370..508>
│       └╼ ReturnStatement <370..508>
│         └╼ BinaryOperatorExpression <370..508>
│           ├╼ Operator -> BitwiseOr <389..390>
│           ├╼ LHS
│           │ └╼ BinaryOperatorExpression <377..388>
│           │   ├╼ Operator -> BitwiseXor <384..385>
│           │   ├╼ LHS
│           │   │ └╼ BinaryOperatorExpression <377..383>
│           │   │   ├╼ Operator -> BitwiseAnd <379..380>
│           │   │   ├╼ LHS
│           │   │   │ └╼ Constant
│           │   │   │   └╼ Integer -> Generic(4), <377..378>
│           │   │   └╼ RHS
│           │   │     └╼ Constant
│           │   │       └╼ Integer -> Generic(12), <381..383>
│           │   └╼ RHS
│           │     └╼ Constant
│           │       └╼ Integer -> Generic(13), <386..388>
│           └╼ RHS
│             └╼ BinaryOperatorExpression <392..507>
│               ├╼ Operator -> NotEquals <503..505>
│               ├╼ LHS
│               │ └╼ BinaryOperatorExpression <392..502>
│               │   ├╼ Operator -> GreaterOrEqual <497..499>
│               │   ├╼ LHS
│               │   │ └╼ BinaryOperatorExpression <392..496>
│               │   │   ├╼ Operator -> Minus <492..493>
│               │   │   ├╼ LHS
│               │   │   │ └╼ BinaryOperatorExpression <392..491>
│               │   │   │   ├╼ Operator -> Multiply <484..485>
│               │   │   │   ├╼ LHS
│               │   │   │   │ └╼ BinaryOperatorExpression <392..482>
│               │   │   │   │   ├╼ Operator -> Multiply <425..426>
│               │   │   │   │   ├╼ LHS
│               │   │   │   │   │ └╼ BinaryOperatorExpression <392..423>
│               │   │   │   │   │   ├╼ Operator -> BitwiseAnd <419..420>
│               │   │   │   │   │   ├╼ LHS
│               │   │   │   │   │   │ └╼ BinaryOperatorExpression <392..418>
│               │   │   │   │   │   │   ├╼ Operator -> ShiftLeft <414..416>
│               │   │   │   │   │   │   ├╼ LHS
│               │   │   │   │   │   │   │ └╼ BinaryOperatorExpression <392..413>
│               │   │   │   │   │   │   │   ├╼ Operator -> Plus <409..410>
│               │   │   │   │   │   │   │   ├╼ LHS
│               │   │   │   │   │   │   │   │ └╼ Identifier -> "g_GlobalVariable" <392..408>
│               │   │   │   │   │   │   │   └╼ RHS
│               │   │   │   │   │   │   │     └╼ Constant
│               │   │   │   │   │   │   │       └╼ Integer -> Unsigned(2), <411..413>
│               │   │   │   │   │   │   └╼ RHS
│               │   │   │   │   │   │     └╼ Constant
│               │   │   │   │   │   │       └╼ Integer -> Generic(2), <417..418>
│               │   │   │   │   │   └╼ RHS
│               │   │   │   │   │     └╼ Constant
│               │   │   │   │   │       └╼ Integer -> Generic(31), <421..423>
│               │   │   │   │   └╼ RHS
│               │   │   │   │     └╼ BinaryOperatorExpression <430..482>
│               │   │   │   │       ├╼ Operator -> Multiply <479..480>
│               │   │   │   │       ├╼ LHS
│               │   │   │   │       │ └╼ BinaryOperatorExpression <430..477>
│               │   │   │   │       │   ├╼ Operator -> Multiply <473..474>
│               │   │   │   │       │   ├╼ LHS
│               │   │   │   │       │   │ └╼ BinaryOperatorExpression <430..471>
│               │   │   │   │       │   │   ├╼ Operator -> Equals <454..456>
│               │   │   │   │       │   │   ├╼ LHS
│               │   │   │   │       │   │   │ └╼ BinaryOperatorExpression <430..453>
│               │   │   │   │       │   │   │   ├╼ Operator -> Plus <434..435>
│               │   │   │   │       │   │   │   ├╼ LHS
│               │   │   │   │       │   │   │   │ └╼ Constant
│               │   │   │   │       │   │   │   │   └╼ Integer -> SignedLong(34), <430..433>
│               │   │   │   │       │   │   │   └╼ RHS
│               │   │   │   │       │   │   │     └╼ Identifier -> "_g_AssumeABoolean" <436..453>
│               │   │   │   │       │   │   └╼ RHS
│               │   │   │   │       │   │     └╼ BinaryOperatorExpression <457..471>
│               │   │   │   │       │   │       ├╼ Operator -> ShiftRight <467..469>
│               │   │   │   │       │   │       ├╼ LHS
│               │   │   │   │       │   │       │ └╼ Identifier -> "g_NiceVar" <457..466>
│               │   │   │   │       │   │       └╼ RHS
│               │   │   │   │       │   │         └╼ Constant
│               │   │   │   │       │   │           └╼ Integer -> Generic(3), <470..471>
│               │   │   │   │       │   └╼ RHS
│               │   │   │   │       │     └╼ Constant
│               │   │   │   │       │       └╼ Integer -> Generic(34), <475..477>
│               │   │   │   │       └╼ RHS
│               │   │   │   │         └╼ Constant
│               │   │   │   │           └╼ Integer -> Generic(4), <481..482>
│               │   │   │   └╼ RHS
│               │   │   │     └╼ Constant
│               │   │   │       └╼ Float -> Float(23.6) <486..491>
│               │   │   └╼ RHS
│               │   │     └╼ Constant
│               │   │       └╼ Integer -> Generic(45), <494..496>
│               │   └╼ RHS
│               │     └╼ Constant
│               │       └╼ Integer -> Generic(25), <500..502>
│               └╼ RHS
│                 └╼ Constant
│                   └╼ Integer -> Generic(0), <506..507>
├╼ FunctionDefinition <514..539>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ ├╼ FunctionSpecifier -> Inline <514..520>
│ │ │ └╼ TypeSpecifier -> Float <521..526>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "AdityaG" <527..534>
│ │   └╼ FunctionParameters
│ │     └╼ Empty
│ └╼ FunctionBody
│   └╼ Empty
├╼ Declaration <540..595>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ TypeQualifier -> Const <540..545>
│ │ └╼ TypeSpecifier -> Int <546..549>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ FunctionDeclarator
│       ├╼ Identifier -> "NiceFunction" <550..562>
│       └╼ FunctionParameters
│         ├╼ FunctionParameter <563..579>
│         │ ├╼ DeclarationSpecifiers
│         │ │ ├╼ TypeQualifier -> Const <563..568>
│         │ │ └╼ TypeSpecifier -> Int <569..572>
│         │ └╼ DirectDeclarator -> "param1" <573..579>
│         └╼ FunctionParameter <581..593>
│           ├╼ DeclarationSpecifiers
│           │ └╼ TypeSpecifier -> Float <581..586>
│           └╼ DirectDeclarator -> "param2" <587..593>
└╼ Declaration <596..644>
  ├╼ DeclarationSpecifiers
  │ ├╼ TypeSpecifier -> Unsigned <596..604>
  │ └╼ TypeSpecifier -> Int <605..608>
  └╼ InitDeclaratorList
    └╼ InitDeclarator
      └╼ FunctionDeclarator
        ├╼ Identifier -> "GetSum" <609..615>
        └╼ FunctionParameters
          ├╼ FunctionParameter <616..629>
          │ ├╼ DeclarationSpecifiers
          │ │ ├╼ TypeSpecifier -> Unsigned <616..624>
          │ │ └╼ TypeSpecifier -> Int <625..628>
          │ └╼ DirectDeclarator -> None
          └╼ FunctionParameter <630..643>
            ├╼ DeclarationSpecifiers
            │ ├╼ TypeSpecifier -> Unsigned <630..638>
            │ └╼ TypeSpecifier -> Int <639..642>
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
