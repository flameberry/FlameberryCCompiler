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
    { return (g_GlobalVariable + 2u << 2) * (((34l + g_NiceVar >> 3) * 34) * 4) * 23.6f - 45 >= 25; }
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
├╼ FunctionDefinition <117..467>
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
│   └╼ CompoundStatement <138..463>
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
│     └╼ CompoundStatement <370..463>
│       └╼ ReturnStatement <370..463>
│         └╼ BinaryOperatorExpression <370..463>
│           ├╼ Operator -> GreaterOrEqual <457..459>
│           ├╼ LHS
│           │ └╼ BinaryOperatorExpression <378..456>
│           │   ├╼ Operator -> Minus <452..453>
│           │   ├╼ LHS
│           │   │ └╼ BinaryOperatorExpression <378..451>
│           │   │   ├╼ Operator -> Multiply <444..445>
│           │   │   ├╼ LHS
│           │   │   │ └╼ BinaryOperatorExpression <378..442>
│           │   │   │   ├╼ Operator -> Multiply <406..407>
│           │   │   │   ├╼ LHS
│           │   │   │   │ └╼ BinaryOperatorExpression <378..404>
│           │   │   │   │   ├╼ Operator -> ShiftLeft <400..402>
│           │   │   │   │   ├╼ LHS
│           │   │   │   │   │ └╼ BinaryOperatorExpression <378..399>
│           │   │   │   │   │   ├╼ Operator -> Plus <395..396>
│           │   │   │   │   │   ├╼ LHS
│           │   │   │   │   │   │ └╼ Identifier -> "g_GlobalVariable" <378..394>
│           │   │   │   │   │   └╼ RHS
│           │   │   │   │   │     └╼ Constant
│           │   │   │   │   │       └╼ Integer -> Unsigned(2), <397..399>
│           │   │   │   │   └╼ RHS
│           │   │   │   │     └╼ Constant
│           │   │   │   │       └╼ Integer -> Generic(2), <403..404>
│           │   │   │   └╼ RHS
│           │   │   │     └╼ BinaryOperatorExpression <411..442>
│           │   │   │       ├╼ Operator -> Multiply <439..440>
│           │   │   │       ├╼ LHS
│           │   │   │       │ └╼ BinaryOperatorExpression <411..437>
│           │   │   │       │   ├╼ Operator -> Multiply <433..434>
│           │   │   │       │   ├╼ LHS
│           │   │   │       │   │ └╼ BinaryOperatorExpression <411..431>
│           │   │   │       │   │   ├╼ Operator -> ShiftRight <427..429>
│           │   │   │       │   │   ├╼ LHS
│           │   │   │       │   │   │ └╼ BinaryOperatorExpression <411..426>
│           │   │   │       │   │   │   ├╼ Operator -> Plus <415..416>
│           │   │   │       │   │   │   ├╼ LHS
│           │   │   │       │   │   │   │ └╼ Constant
│           │   │   │       │   │   │   │   └╼ Integer -> SignedLong(34), <411..414>
│           │   │   │       │   │   │   └╼ RHS
│           │   │   │       │   │   │     └╼ Identifier -> "g_NiceVar" <417..426>
│           │   │   │       │   │   └╼ RHS
│           │   │   │       │   │     └╼ Constant
│           │   │   │       │   │       └╼ Integer -> Generic(3), <430..431>
│           │   │   │       │   └╼ RHS
│           │   │   │       │     └╼ Constant
│           │   │   │       │       └╼ Integer -> Generic(34), <435..437>
│           │   │   │       └╼ RHS
│           │   │   │         └╼ Constant
│           │   │   │           └╼ Integer -> Generic(4), <441..442>
│           │   │   └╼ RHS
│           │   │     └╼ Constant
│           │   │       └╼ Float -> Float(23.6) <446..451>
│           │   └╼ RHS
│           │     └╼ Constant
│           │       └╼ Integer -> Generic(45), <454..456>
│           └╼ RHS
│             └╼ Constant
│               └╼ Integer -> Generic(25), <460..462>
├╼ FunctionDefinition <469..494>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ ├╼ FunctionSpecifier -> Inline <469..475>
│ │ │ └╼ TypeSpecifier -> Float <476..481>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "AdityaG" <482..489>
│ │   └╼ FunctionParameters
│ │     └╼ Empty
│ └╼ FunctionBody
│   └╼ Empty
├╼ Declaration <495..550>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ TypeQualifier -> Const <495..500>
│ │ └╼ TypeSpecifier -> Int <501..504>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ FunctionDeclarator
│       ├╼ Identifier -> "NiceFunction" <505..517>
│       └╼ FunctionParameters
│         ├╼ FunctionParameter <518..534>
│         │ ├╼ DeclarationSpecifiers
│         │ │ ├╼ TypeQualifier -> Const <518..523>
│         │ │ └╼ TypeSpecifier -> Int <524..527>
│         │ └╼ DirectDeclarator -> "param1" <528..534>
│         └╼ FunctionParameter <536..548>
│           ├╼ DeclarationSpecifiers
│           │ └╼ TypeSpecifier -> Float <536..541>
│           └╼ DirectDeclarator -> "param2" <542..548>
└╼ Declaration <551..599>
  ├╼ DeclarationSpecifiers
  │ ├╼ TypeSpecifier -> Unsigned <551..559>
  │ └╼ TypeSpecifier -> Int <560..563>
  └╼ InitDeclaratorList
    └╼ InitDeclarator
      └╼ FunctionDeclarator
        ├╼ Identifier -> "GetSum" <564..570>
        └╼ FunctionParameters
          ├╼ FunctionParameter <571..584>
          │ ├╼ DeclarationSpecifiers
          │ │ ├╼ TypeSpecifier -> Unsigned <571..579>
          │ │ └╼ TypeSpecifier -> Int <580..583>
          │ └╼ DirectDeclarator -> None
          └╼ FunctionParameter <585..598>
            ├╼ DeclarationSpecifiers
            │ ├╼ TypeSpecifier -> Unsigned <585..593>
            │ └╼ TypeSpecifier -> Int <594..597>
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
