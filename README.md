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
    { return (g_GlobalVariable + 2u << 2) * (((34l + _g_AssumeABoolean == g_NiceVar >> 3) * 34) * 4) * 23.6f - 45 >= 25 != 0; }
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
├╼ FunctionDefinition <117..493>
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
│   └╼ CompoundStatement <138..489>
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
│     └╼ CompoundStatement <370..489>
│       └╼ ReturnStatement <370..489>
│         └╼ BinaryOperatorExpression <370..489>
│           ├╼ Operator -> NotEquals <484..486>
│           ├╼ LHS
│           │ └╼ BinaryOperatorExpression <378..483>
│           │   ├╼ Operator -> GreaterOrEqual <478..480>
│           │   ├╼ LHS
│           │   │ └╼ BinaryOperatorExpression <378..477>
│           │   │   ├╼ Operator -> Minus <473..474>
│           │   │   ├╼ LHS
│           │   │   │ └╼ BinaryOperatorExpression <378..472>
│           │   │   │   ├╼ Operator -> Multiply <465..466>
│           │   │   │   ├╼ LHS
│           │   │   │   │ └╼ BinaryOperatorExpression <378..463>
│           │   │   │   │   ├╼ Operator -> Multiply <406..407>
│           │   │   │   │   ├╼ LHS
│           │   │   │   │   │ └╼ BinaryOperatorExpression <378..404>
│           │   │   │   │   │   ├╼ Operator -> ShiftLeft <400..402>
│           │   │   │   │   │   ├╼ LHS
│           │   │   │   │   │   │ └╼ BinaryOperatorExpression <378..399>
│           │   │   │   │   │   │   ├╼ Operator -> Plus <395..396>
│           │   │   │   │   │   │   ├╼ LHS
│           │   │   │   │   │   │   │ └╼ Identifier -> "g_GlobalVariable" <378..394>
│           │   │   │   │   │   │   └╼ RHS
│           │   │   │   │   │   │     └╼ Constant
│           │   │   │   │   │   │       └╼ Integer -> Unsigned(2), <397..399>
│           │   │   │   │   │   └╼ RHS
│           │   │   │   │   │     └╼ Constant
│           │   │   │   │   │       └╼ Integer -> Generic(2), <403..404>
│           │   │   │   │   └╼ RHS
│           │   │   │   │     └╼ BinaryOperatorExpression <411..463>
│           │   │   │   │       ├╼ Operator -> Multiply <460..461>
│           │   │   │   │       ├╼ LHS
│           │   │   │   │       │ └╼ BinaryOperatorExpression <411..458>
│           │   │   │   │       │   ├╼ Operator -> Multiply <454..455>
│           │   │   │   │       │   ├╼ LHS
│           │   │   │   │       │   │ └╼ BinaryOperatorExpression <411..452>
│           │   │   │   │       │   │   ├╼ Operator -> Equals <435..437>
│           │   │   │   │       │   │   ├╼ LHS
│           │   │   │   │       │   │   │ └╼ BinaryOperatorExpression <411..434>
│           │   │   │   │       │   │   │   ├╼ Operator -> Plus <415..416>
│           │   │   │   │       │   │   │   ├╼ LHS
│           │   │   │   │       │   │   │   │ └╼ Constant
│           │   │   │   │       │   │   │   │   └╼ Integer -> SignedLong(34), <411..414>
│           │   │   │   │       │   │   │   └╼ RHS
│           │   │   │   │       │   │   │     └╼ Identifier -> "_g_AssumeABoolean" <417..434>
│           │   │   │   │       │   │   └╼ RHS
│           │   │   │   │       │   │     └╼ BinaryOperatorExpression <438..452>
│           │   │   │   │       │   │       ├╼ Operator -> ShiftRight <448..450>
│           │   │   │   │       │   │       ├╼ LHS
│           │   │   │   │       │   │       │ └╼ Identifier -> "g_NiceVar" <438..447>
│           │   │   │   │       │   │       └╼ RHS
│           │   │   │   │       │   │         └╼ Constant
│           │   │   │   │       │   │           └╼ Integer -> Generic(3), <451..452>
│           │   │   │   │       │   └╼ RHS
│           │   │   │   │       │     └╼ Constant
│           │   │   │   │       │       └╼ Integer -> Generic(34), <456..458>
│           │   │   │   │       └╼ RHS
│           │   │   │   │         └╼ Constant
│           │   │   │   │           └╼ Integer -> Generic(4), <462..463>
│           │   │   │   └╼ RHS
│           │   │   │     └╼ Constant
│           │   │   │       └╼ Float -> Float(23.6) <467..472>
│           │   │   └╼ RHS
│           │   │     └╼ Constant
│           │   │       └╼ Integer -> Generic(45), <475..477>
│           │   └╼ RHS
│           │     └╼ Constant
│           │       └╼ Integer -> Generic(25), <481..483>
│           └╼ RHS
│             └╼ Constant
│               └╼ Integer -> Generic(0), <487..488>
├╼ FunctionDefinition <495..520>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ ├╼ FunctionSpecifier -> Inline <495..501>
│ │ │ └╼ TypeSpecifier -> Float <502..507>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "AdityaG" <508..515>
│ │   └╼ FunctionParameters
│ │     └╼ Empty
│ └╼ FunctionBody
│   └╼ Empty
├╼ Declaration <521..576>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ TypeQualifier -> Const <521..526>
│ │ └╼ TypeSpecifier -> Int <527..530>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ FunctionDeclarator
│       ├╼ Identifier -> "NiceFunction" <531..543>
│       └╼ FunctionParameters
│         ├╼ FunctionParameter <544..560>
│         │ ├╼ DeclarationSpecifiers
│         │ │ ├╼ TypeQualifier -> Const <544..549>
│         │ │ └╼ TypeSpecifier -> Int <550..553>
│         │ └╼ DirectDeclarator -> "param1" <554..560>
│         └╼ FunctionParameter <562..574>
│           ├╼ DeclarationSpecifiers
│           │ └╼ TypeSpecifier -> Float <562..567>
│           └╼ DirectDeclarator -> "param2" <568..574>
└╼ Declaration <577..625>
  ├╼ DeclarationSpecifiers
  │ ├╼ TypeSpecifier -> Unsigned <577..585>
  │ └╼ TypeSpecifier -> Int <586..589>
  └╼ InitDeclaratorList
    └╼ InitDeclarator
      └╼ FunctionDeclarator
        ├╼ Identifier -> "GetSum" <590..596>
        └╼ FunctionParameters
          ├╼ FunctionParameter <597..610>
          │ ├╼ DeclarationSpecifiers
          │ │ ├╼ TypeSpecifier -> Unsigned <597..605>
          │ │ └╼ TypeSpecifier -> Int <606..609>
          │ └╼ DirectDeclarator -> None
          └╼ FunctionParameter <611..624>
            ├╼ DeclarationSpecifiers
            │ ├╼ TypeSpecifier -> Unsigned <611..619>
            │ └╼ TypeSpecifier -> Int <620..623>
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
