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
    { return 4 & 12 ^ 13 | 1 && 5 || (g_GlobalVariable + 2u << 2 & 31) * (((34l + _g_AssumeABoolean == g_NiceVar >> 3) * 34) * 4) * 23.6f - 45 >= 25 != 0; }
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
├╼ FunctionDefinition <117..522>
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
│   └╼ CompoundStatement <138..518>
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
│     └╼ CompoundStatement <370..518>
│       └╼ ReturnStatement <370..518>
│         └╼ BinaryOperatorExpression <370..518>
│           ├╼ Operator -> LogicalOr <398..400>
│           ├╼ LHS
│           │ └╼ BinaryOperatorExpression <377..397>
│           │   ├╼ Operator -> LogicalAnd <393..395>
│           │   ├╼ LHS
│           │   │ └╼ BinaryOperatorExpression <377..392>
│           │   │   ├╼ Operator -> BitwiseOr <389..390>
│           │   │   ├╼ LHS
│           │   │   │ └╼ BinaryOperatorExpression <377..388>
│           │   │   │   ├╼ Operator -> BitwiseXor <384..385>
│           │   │   │   ├╼ LHS
│           │   │   │   │ └╼ BinaryOperatorExpression <377..383>
│           │   │   │   │   ├╼ Operator -> BitwiseAnd <379..380>
│           │   │   │   │   ├╼ LHS
│           │   │   │   │   │ └╼ Constant
│           │   │   │   │   │   └╼ Integer -> Generic(4), <377..378>
│           │   │   │   │   └╼ RHS
│           │   │   │   │     └╼ Constant
│           │   │   │   │       └╼ Integer -> Generic(12), <381..383>
│           │   │   │   └╼ RHS
│           │   │   │     └╼ Constant
│           │   │   │       └╼ Integer -> Generic(13), <386..388>
│           │   │   └╼ RHS
│           │   │     └╼ Constant
│           │   │       └╼ Integer -> Generic(1), <391..392>
│           │   └╼ RHS
│           │     └╼ Constant
│           │       └╼ Integer -> Generic(5), <396..397>
│           └╼ RHS
│             └╼ BinaryOperatorExpression <402..517>
│               ├╼ Operator -> NotEquals <513..515>
│               ├╼ LHS
│               │ └╼ BinaryOperatorExpression <402..512>
│               │   ├╼ Operator -> GreaterOrEqual <507..509>
│               │   ├╼ LHS
│               │   │ └╼ BinaryOperatorExpression <402..506>
│               │   │   ├╼ Operator -> Minus <502..503>
│               │   │   ├╼ LHS
│               │   │   │ └╼ BinaryOperatorExpression <402..501>
│               │   │   │   ├╼ Operator -> Multiply <494..495>
│               │   │   │   ├╼ LHS
│               │   │   │   │ └╼ BinaryOperatorExpression <402..492>
│               │   │   │   │   ├╼ Operator -> Multiply <435..436>
│               │   │   │   │   ├╼ LHS
│               │   │   │   │   │ └╼ BinaryOperatorExpression <402..433>
│               │   │   │   │   │   ├╼ Operator -> BitwiseAnd <429..430>
│               │   │   │   │   │   ├╼ LHS
│               │   │   │   │   │   │ └╼ BinaryOperatorExpression <402..428>
│               │   │   │   │   │   │   ├╼ Operator -> ShiftLeft <424..426>
│               │   │   │   │   │   │   ├╼ LHS
│               │   │   │   │   │   │   │ └╼ BinaryOperatorExpression <402..423>
│               │   │   │   │   │   │   │   ├╼ Operator -> Plus <419..420>
│               │   │   │   │   │   │   │   ├╼ LHS
│               │   │   │   │   │   │   │   │ └╼ Identifier -> "g_GlobalVariable" <402..418>
│               │   │   │   │   │   │   │   └╼ RHS
│               │   │   │   │   │   │   │     └╼ Constant
│               │   │   │   │   │   │   │       └╼ Integer -> Unsigned(2), <421..423>
│               │   │   │   │   │   │   └╼ RHS
│               │   │   │   │   │   │     └╼ Constant
│               │   │   │   │   │   │       └╼ Integer -> Generic(2), <427..428>
│               │   │   │   │   │   └╼ RHS
│               │   │   │   │   │     └╼ Constant
│               │   │   │   │   │       └╼ Integer -> Generic(31), <431..433>
│               │   │   │   │   └╼ RHS
│               │   │   │   │     └╼ BinaryOperatorExpression <440..492>
│               │   │   │   │       ├╼ Operator -> Multiply <489..490>
│               │   │   │   │       ├╼ LHS
│               │   │   │   │       │ └╼ BinaryOperatorExpression <440..487>
│               │   │   │   │       │   ├╼ Operator -> Multiply <483..484>
│               │   │   │   │       │   ├╼ LHS
│               │   │   │   │       │   │ └╼ BinaryOperatorExpression <440..481>
│               │   │   │   │       │   │   ├╼ Operator -> Equals <464..466>
│               │   │   │   │       │   │   ├╼ LHS
│               │   │   │   │       │   │   │ └╼ BinaryOperatorExpression <440..463>
│               │   │   │   │       │   │   │   ├╼ Operator -> Plus <444..445>
│               │   │   │   │       │   │   │   ├╼ LHS
│               │   │   │   │       │   │   │   │ └╼ Constant
│               │   │   │   │       │   │   │   │   └╼ Integer -> SignedLong(34), <440..443>
│               │   │   │   │       │   │   │   └╼ RHS
│               │   │   │   │       │   │   │     └╼ Identifier -> "_g_AssumeABoolean" <446..463>
│               │   │   │   │       │   │   └╼ RHS
│               │   │   │   │       │   │     └╼ BinaryOperatorExpression <467..481>
│               │   │   │   │       │   │       ├╼ Operator -> ShiftRight <477..479>
│               │   │   │   │       │   │       ├╼ LHS
│               │   │   │   │       │   │       │ └╼ Identifier -> "g_NiceVar" <467..476>
│               │   │   │   │       │   │       └╼ RHS
│               │   │   │   │       │   │         └╼ Constant
│               │   │   │   │       │   │           └╼ Integer -> Generic(3), <480..481>
│               │   │   │   │       │   └╼ RHS
│               │   │   │   │       │     └╼ Constant
│               │   │   │   │       │       └╼ Integer -> Generic(34), <485..487>
│               │   │   │   │       └╼ RHS
│               │   │   │   │         └╼ Constant
│               │   │   │   │           └╼ Integer -> Generic(4), <491..492>
│               │   │   │   └╼ RHS
│               │   │   │     └╼ Constant
│               │   │   │       └╼ Float -> Float(23.6) <496..501>
│               │   │   └╼ RHS
│               │   │     └╼ Constant
│               │   │       └╼ Integer -> Generic(45), <504..506>
│               │   └╼ RHS
│               │     └╼ Constant
│               │       └╼ Integer -> Generic(25), <510..512>
│               └╼ RHS
│                 └╼ Constant
│                   └╼ Integer -> Generic(0), <516..517>
├╼ FunctionDefinition <524..549>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ ├╼ FunctionSpecifier -> Inline <524..530>
│ │ │ └╼ TypeSpecifier -> Float <531..536>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "AdityaG" <537..544>
│ │   └╼ FunctionParameters
│ │     └╼ Empty
│ └╼ FunctionBody
│   └╼ Empty
├╼ Declaration <550..605>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ TypeQualifier -> Const <550..555>
│ │ └╼ TypeSpecifier -> Int <556..559>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ FunctionDeclarator
│       ├╼ Identifier -> "NiceFunction" <560..572>
│       └╼ FunctionParameters
│         ├╼ FunctionParameter <573..589>
│         │ ├╼ DeclarationSpecifiers
│         │ │ ├╼ TypeQualifier -> Const <573..578>
│         │ │ └╼ TypeSpecifier -> Int <579..582>
│         │ └╼ DirectDeclarator -> "param1" <583..589>
│         └╼ FunctionParameter <591..603>
│           ├╼ DeclarationSpecifiers
│           │ └╼ TypeSpecifier -> Float <591..596>
│           └╼ DirectDeclarator -> "param2" <597..603>
└╼ Declaration <606..654>
  ├╼ DeclarationSpecifiers
  │ ├╼ TypeSpecifier -> Unsigned <606..614>
  │ └╼ TypeSpecifier -> Int <615..618>
  └╼ InitDeclaratorList
    └╼ InitDeclarator
      └╼ FunctionDeclarator
        ├╼ Identifier -> "GetSum" <619..625>
        └╼ FunctionParameters
          ├╼ FunctionParameter <626..639>
          │ ├╼ DeclarationSpecifiers
          │ │ ├╼ TypeSpecifier -> Unsigned <626..634>
          │ │ └╼ TypeSpecifier -> Int <635..638>
          │ └╼ DirectDeclarator -> None
          └╼ FunctionParameter <640..653>
            ├╼ DeclarationSpecifiers
            │ ├╼ TypeSpecifier -> Unsigned <640..648>
            │ └╼ TypeSpecifier -> Int <649..652>
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
