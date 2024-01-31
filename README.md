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
    { return 4 & 12 ^ (g_GlobalVariable + 2u << 2 & 31) * (((34l + _g_AssumeABoolean == g_NiceVar >> 3) * 34) * 4) * 23.6f - 45 >= 25 != 0; }
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
├╼ FunctionDefinition <117..507>
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
│   └╼ CompoundStatement <138..503>
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
│     └╼ CompoundStatement <370..503>
│       └╼ ReturnStatement <370..503>
│         └╼ BinaryOperatorExpression <370..503>
│           ├╼ Operator -> BitwiseXor <384..385>
│           ├╼ LHS
│           │ └╼ BinaryOperatorExpression <377..383>
│           │   ├╼ Operator -> BitwiseAnd <379..380>
│           │   ├╼ LHS
│           │   │ └╼ Constant
│           │   │   └╼ Integer -> Generic(4), <377..378>
│           │   └╼ RHS
│           │     └╼ Constant
│           │       └╼ Integer -> Generic(12), <381..383>
│           └╼ RHS
│             └╼ BinaryOperatorExpression <387..502>
│               ├╼ Operator -> NotEquals <498..500>
│               ├╼ LHS
│               │ └╼ BinaryOperatorExpression <387..497>
│               │   ├╼ Operator -> GreaterOrEqual <492..494>
│               │   ├╼ LHS
│               │   │ └╼ BinaryOperatorExpression <387..491>
│               │   │   ├╼ Operator -> Minus <487..488>
│               │   │   ├╼ LHS
│               │   │   │ └╼ BinaryOperatorExpression <387..486>
│               │   │   │   ├╼ Operator -> Multiply <479..480>
│               │   │   │   ├╼ LHS
│               │   │   │   │ └╼ BinaryOperatorExpression <387..477>
│               │   │   │   │   ├╼ Operator -> Multiply <420..421>
│               │   │   │   │   ├╼ LHS
│               │   │   │   │   │ └╼ BinaryOperatorExpression <387..418>
│               │   │   │   │   │   ├╼ Operator -> BitwiseAnd <414..415>
│               │   │   │   │   │   ├╼ LHS
│               │   │   │   │   │   │ └╼ BinaryOperatorExpression <387..413>
│               │   │   │   │   │   │   ├╼ Operator -> ShiftLeft <409..411>
│               │   │   │   │   │   │   ├╼ LHS
│               │   │   │   │   │   │   │ └╼ BinaryOperatorExpression <387..408>
│               │   │   │   │   │   │   │   ├╼ Operator -> Plus <404..405>
│               │   │   │   │   │   │   │   ├╼ LHS
│               │   │   │   │   │   │   │   │ └╼ Identifier -> "g_GlobalVariable" <387..403>
│               │   │   │   │   │   │   │   └╼ RHS
│               │   │   │   │   │   │   │     └╼ Constant
│               │   │   │   │   │   │   │       └╼ Integer -> Unsigned(2), <406..408>
│               │   │   │   │   │   │   └╼ RHS
│               │   │   │   │   │   │     └╼ Constant
│               │   │   │   │   │   │       └╼ Integer -> Generic(2), <412..413>
│               │   │   │   │   │   └╼ RHS
│               │   │   │   │   │     └╼ Constant
│               │   │   │   │   │       └╼ Integer -> Generic(31), <416..418>
│               │   │   │   │   └╼ RHS
│               │   │   │   │     └╼ BinaryOperatorExpression <425..477>
│               │   │   │   │       ├╼ Operator -> Multiply <474..475>
│               │   │   │   │       ├╼ LHS
│               │   │   │   │       │ └╼ BinaryOperatorExpression <425..472>
│               │   │   │   │       │   ├╼ Operator -> Multiply <468..469>
│               │   │   │   │       │   ├╼ LHS
│               │   │   │   │       │   │ └╼ BinaryOperatorExpression <425..466>
│               │   │   │   │       │   │   ├╼ Operator -> Equals <449..451>
│               │   │   │   │       │   │   ├╼ LHS
│               │   │   │   │       │   │   │ └╼ BinaryOperatorExpression <425..448>
│               │   │   │   │       │   │   │   ├╼ Operator -> Plus <429..430>
│               │   │   │   │       │   │   │   ├╼ LHS
│               │   │   │   │       │   │   │   │ └╼ Constant
│               │   │   │   │       │   │   │   │   └╼ Integer -> SignedLong(34), <425..428>
│               │   │   │   │       │   │   │   └╼ RHS
│               │   │   │   │       │   │   │     └╼ Identifier -> "_g_AssumeABoolean" <431..448>
│               │   │   │   │       │   │   └╼ RHS
│               │   │   │   │       │   │     └╼ BinaryOperatorExpression <452..466>
│               │   │   │   │       │   │       ├╼ Operator -> ShiftRight <462..464>
│               │   │   │   │       │   │       ├╼ LHS
│               │   │   │   │       │   │       │ └╼ Identifier -> "g_NiceVar" <452..461>
│               │   │   │   │       │   │       └╼ RHS
│               │   │   │   │       │   │         └╼ Constant
│               │   │   │   │       │   │           └╼ Integer -> Generic(3), <465..466>
│               │   │   │   │       │   └╼ RHS
│               │   │   │   │       │     └╼ Constant
│               │   │   │   │       │       └╼ Integer -> Generic(34), <470..472>
│               │   │   │   │       └╼ RHS
│               │   │   │   │         └╼ Constant
│               │   │   │   │           └╼ Integer -> Generic(4), <476..477>
│               │   │   │   └╼ RHS
│               │   │   │     └╼ Constant
│               │   │   │       └╼ Float -> Float(23.6) <481..486>
│               │   │   └╼ RHS
│               │   │     └╼ Constant
│               │   │       └╼ Integer -> Generic(45), <489..491>
│               │   └╼ RHS
│               │     └╼ Constant
│               │       └╼ Integer -> Generic(25), <495..497>
│               └╼ RHS
│                 └╼ Constant
│                   └╼ Integer -> Generic(0), <501..502>
├╼ FunctionDefinition <509..534>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ ├╼ FunctionSpecifier -> Inline <509..515>
│ │ │ └╼ TypeSpecifier -> Float <516..521>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "AdityaG" <522..529>
│ │   └╼ FunctionParameters
│ │     └╼ Empty
│ └╼ FunctionBody
│   └╼ Empty
├╼ Declaration <535..590>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ TypeQualifier -> Const <535..540>
│ │ └╼ TypeSpecifier -> Int <541..544>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ FunctionDeclarator
│       ├╼ Identifier -> "NiceFunction" <545..557>
│       └╼ FunctionParameters
│         ├╼ FunctionParameter <558..574>
│         │ ├╼ DeclarationSpecifiers
│         │ │ ├╼ TypeQualifier -> Const <558..563>
│         │ │ └╼ TypeSpecifier -> Int <564..567>
│         │ └╼ DirectDeclarator -> "param1" <568..574>
│         └╼ FunctionParameter <576..588>
│           ├╼ DeclarationSpecifiers
│           │ └╼ TypeSpecifier -> Float <576..581>
│           └╼ DirectDeclarator -> "param2" <582..588>
└╼ Declaration <591..639>
  ├╼ DeclarationSpecifiers
  │ ├╼ TypeSpecifier -> Unsigned <591..599>
  │ └╼ TypeSpecifier -> Int <600..603>
  └╼ InitDeclaratorList
    └╼ InitDeclarator
      └╼ FunctionDeclarator
        ├╼ Identifier -> "GetSum" <604..610>
        └╼ FunctionParameters
          ├╼ FunctionParameter <611..624>
          │ ├╼ DeclarationSpecifiers
          │ │ ├╼ TypeSpecifier -> Unsigned <611..619>
          │ │ └╼ TypeSpecifier -> Int <620..623>
          │ └╼ DirectDeclarator -> None
          └╼ FunctionParameter <625..638>
            ├╼ DeclarationSpecifiers
            │ ├╼ TypeSpecifier -> Unsigned <625..633>
            │ └╼ TypeSpecifier -> Int <634..637>
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
