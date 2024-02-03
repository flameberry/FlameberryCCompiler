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
    // This is a comment to try and mess with the tokenizer :D
    if (_g_AssumeABoolean + 3 < 4)
    {
        char ch = 'A';
        return g_NiceVar ? ch > 1 ? ch : 0 : 11;
    }
    else if (g_NiceVar <= 44)
        return 69l;
    else if (g_GlobalVariable > 3)
        // This is a comment to try and mess with the tokenizer :D
        return 12ul;
    else
        // This is a comment to try and mess with the tokenizer :D
        return 0;
    {}
    // This is a comment to try and mess with the tokenizer :D
    { return _g_AssumeABoolean ? 4 & 12 ^ 13 | 1 && 5 || (g_GlobalVariable + 2u << 2 & 31) * (((34l + _g_AssumeABoolean == g_NiceVar >> 3) * 34) * 4) * 23.6f - 45 >= 25 != 0 : 77; }
}

inline float AdityaG() {}
const int NiceFunction(const int param1, float param2);
unsigned int GetSum(unsigned int, unsigned int);
// This is a comment to try and mess with the tokenizer :D
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
├╼ FunctionDefinition <117..836>
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
│   └╼ CompoundStatement <133..832>
│     ├╼ IfStatement <201..582>
│     │ ├╼ IfExpression
│     │ │ └╼ BinaryOperatorExpression <205..230>
│     │ │   ├╼ Operator -> Less <227..228>
│     │ │   ├╼ LHS
│     │ │   │ └╼ BinaryOperatorExpression <205..226>
│     │ │   │   ├╼ Operator -> Plus <223..224>
│     │ │   │   ├╼ LHS
│     │ │   │   │ └╼ Identifier -> "_g_AssumeABoolean" <205..222>
│     │ │   │   └╼ RHS
│     │ │   │     └╼ Constant
│     │ │   │       └╼ Integer -> Generic(3), <225..226>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(4), <229..230>
│     │ ├╼ ThenStatement
│     │ │ └╼ CompoundStatement <237..309>
│     │ │   ├╼ Declaration <246..260>
│     │ │   │ ├╼ DeclarationSpecifiers
│     │ │   │ │ └╼ TypeSpecifier -> Char <246..250>
│     │ │   │ └╼ InitDeclaratorList
│     │ │   │   └╼ InitDeclarator
│     │ │   │     ├╼ DirectDeclarator -> "ch" <251..253>
│     │ │   │     └╼ Initializer
│     │ │   │       └╼ Constant
│     │ │   │         └╼ Character -> 'A'
│     │ │   └╼ ReturnStatement <269..309>
│     │ │     └╼ TernaryOperatorExpression <269..309>
│     │ │       ├╼ Condition
│     │ │       │ └╼ Identifier -> "g_NiceVar" <276..285>
│     │ │       ├╼ IfExpression
│     │ │       │ └╼ TernaryOperatorExpression <288..303>
│     │ │       │   ├╼ Condition
│     │ │       │   │ └╼ BinaryOperatorExpression <288..294>
│     │ │       │   │   ├╼ Operator -> Greater <291..292>
│     │ │       │   │   ├╼ LHS
│     │ │       │   │   │ └╼ Identifier -> "ch" <288..290>
│     │ │       │   │   └╼ RHS
│     │ │       │   │     └╼ Constant
│     │ │       │   │       └╼ Integer -> Generic(1), <293..294>
│     │ │       │   ├╼ IfExpression
│     │ │       │   │ └╼ Identifier -> "ch" <297..299>
│     │ │       │   └╼ ElseExpression
│     │ │       │     └╼ Constant
│     │ │       │       └╼ Integer -> Generic(0), <302..303>
│     │ │       └╼ ElseExpression
│     │ │         └╼ Constant
│     │ │           └╼ Integer -> Generic(11), <306..308>
│     │ └╼ ElseStatement
│     │   └╼ IfStatement <325..582>
│     │     ├╼ IfExpression
│     │     │ └╼ BinaryOperatorExpression <329..344>
│     │     │   ├╼ Operator -> LessOrEqual <339..341>
│     │     │   ├╼ LHS
│     │     │   │ └╼ Identifier -> "g_NiceVar" <329..338>
│     │     │   └╼ RHS
│     │     │     └╼ Constant
│     │     │       └╼ Integer -> Generic(44), <342..344>
│     │     ├╼ ThenStatement
│     │     │ └╼ ReturnStatement <354..365>
│     │     │   └╼ Constant
│     │     │     └╼ Integer -> SignedLong(69), <354..365>
│     │     └╼ ElseStatement
│     │       └╼ IfStatement <375..582>
│     │         ├╼ IfExpression
│     │         │ └╼ BinaryOperatorExpression <379..399>
│     │         │   ├╼ Operator -> Greater <396..397>
│     │         │   ├╼ LHS
│     │         │   │ └╼ Identifier -> "g_GlobalVariable" <379..395>
│     │         │   └╼ RHS
│     │         │     └╼ Constant
│     │         │       └╼ Integer -> Generic(3), <398..399>
│     │         ├╼ ThenStatement
│     │         │ └╼ ReturnStatement <476..488>
│     │         │   └╼ Constant
│     │         │     └╼ Integer -> UnsignedLong(12), <476..488>
│     │         └╼ ElseStatement
│     │           └╼ ReturnStatement <573..582>
│     │             └╼ Constant
│     │               └╼ Integer -> Generic(0), <573..582>
│     ├╼ CompoundStatement <588..588>
│     │ └╼ Empty
│     └╼ CompoundStatement <658..832>
│       └╼ ReturnStatement <659..832>
│         └╼ TernaryOperatorExpression <659..832>
│           ├╼ Condition
│           │ └╼ Identifier -> "_g_AssumeABoolean" <666..683>
│           ├╼ IfExpression
│           │ └╼ BinaryOperatorExpression <686..826>
│           │   ├╼ Operator -> LogicalOr <707..709>
│           │   ├╼ LHS
│           │   │ └╼ BinaryOperatorExpression <686..706>
│           │   │   ├╼ Operator -> LogicalAnd <702..704>
│           │   │   ├╼ LHS
│           │   │   │ └╼ BinaryOperatorExpression <686..701>
│           │   │   │   ├╼ Operator -> BitwiseOr <698..699>
│           │   │   │   ├╼ LHS
│           │   │   │   │ └╼ BinaryOperatorExpression <686..697>
│           │   │   │   │   ├╼ Operator -> BitwiseXor <693..694>
│           │   │   │   │   ├╼ LHS
│           │   │   │   │   │ └╼ BinaryOperatorExpression <686..692>
│           │   │   │   │   │   ├╼ Operator -> BitwiseAnd <688..689>
│           │   │   │   │   │   ├╼ LHS
│           │   │   │   │   │   │ └╼ Constant
│           │   │   │   │   │   │   └╼ Integer -> Generic(4), <686..687>
│           │   │   │   │   │   └╼ RHS
│           │   │   │   │   │     └╼ Constant
│           │   │   │   │   │       └╼ Integer -> Generic(12), <690..692>
│           │   │   │   │   └╼ RHS
│           │   │   │   │     └╼ Constant
│           │   │   │   │       └╼ Integer -> Generic(13), <695..697>
│           │   │   │   └╼ RHS
│           │   │   │     └╼ Constant
│           │   │   │       └╼ Integer -> Generic(1), <700..701>
│           │   │   └╼ RHS
│           │   │     └╼ Constant
│           │   │       └╼ Integer -> Generic(5), <705..706>
│           │   └╼ RHS
│           │     └╼ BinaryOperatorExpression <711..826>
│           │       ├╼ Operator -> NotEquals <822..824>
│           │       ├╼ LHS
│           │       │ └╼ BinaryOperatorExpression <711..821>
│           │       │   ├╼ Operator -> GreaterOrEqual <816..818>
│           │       │   ├╼ LHS
│           │       │   │ └╼ BinaryOperatorExpression <711..815>
│           │       │   │   ├╼ Operator -> Minus <811..812>
│           │       │   │   ├╼ LHS
│           │       │   │   │ └╼ BinaryOperatorExpression <711..810>
│           │       │   │   │   ├╼ Operator -> Multiply <803..804>
│           │       │   │   │   ├╼ LHS
│           │       │   │   │   │ └╼ BinaryOperatorExpression <711..801>
│           │       │   │   │   │   ├╼ Operator -> Multiply <744..745>
│           │       │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │ └╼ BinaryOperatorExpression <711..742>
│           │       │   │   │   │   │   ├╼ Operator -> BitwiseAnd <738..739>
│           │       │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │ └╼ BinaryOperatorExpression <711..737>
│           │       │   │   │   │   │   │   ├╼ Operator -> ShiftLeft <733..735>
│           │       │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │ └╼ BinaryOperatorExpression <711..732>
│           │       │   │   │   │   │   │   │   ├╼ Operator -> Plus <728..729>
│           │       │   │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │   │ └╼ Identifier -> "g_GlobalVariable" <711..727>
│           │       │   │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │   │       └╼ Integer -> Unsigned(2), <730..732>
│           │       │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │       └╼ Integer -> Generic(2), <736..737>
│           │       │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │       └╼ Integer -> Generic(31), <740..742>
│           │       │   │   │   │   └╼ RHS
│           │       │   │   │   │     └╼ BinaryOperatorExpression <749..801>
│           │       │   │   │   │       ├╼ Operator -> Multiply <798..799>
│           │       │   │   │   │       ├╼ LHS
│           │       │   │   │   │       │ └╼ BinaryOperatorExpression <749..796>
│           │       │   │   │   │       │   ├╼ Operator -> Multiply <792..793>
│           │       │   │   │   │       │   ├╼ LHS
│           │       │   │   │   │       │   │ └╼ BinaryOperatorExpression <749..790>
│           │       │   │   │   │       │   │   ├╼ Operator -> Equals <773..775>
│           │       │   │   │   │       │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │ └╼ BinaryOperatorExpression <749..772>
│           │       │   │   │   │       │   │   │   ├╼ Operator -> Plus <753..754>
│           │       │   │   │   │       │   │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │   │ └╼ Constant
│           │       │   │   │   │       │   │   │   │   └╼ Integer -> SignedLong(34), <749..752>
│           │       │   │   │   │       │   │   │   └╼ RHS
│           │       │   │   │   │       │   │   │     └╼ Identifier -> "_g_AssumeABoolean" <755..772>
│           │       │   │   │   │       │   │   └╼ RHS
│           │       │   │   │   │       │   │     └╼ BinaryOperatorExpression <776..790>
│           │       │   │   │   │       │   │       ├╼ Operator -> ShiftRight <786..788>
│           │       │   │   │   │       │   │       ├╼ LHS
│           │       │   │   │   │       │   │       │ └╼ Identifier -> "g_NiceVar" <776..785>
│           │       │   │   │   │       │   │       └╼ RHS
│           │       │   │   │   │       │   │         └╼ Constant
│           │       │   │   │   │       │   │           └╼ Integer -> Generic(3), <789..790>
│           │       │   │   │   │       │   └╼ RHS
│           │       │   │   │   │       │     └╼ Constant
│           │       │   │   │   │       │       └╼ Integer -> Generic(34), <794..796>
│           │       │   │   │   │       └╼ RHS
│           │       │   │   │   │         └╼ Constant
│           │       │   │   │   │           └╼ Integer -> Generic(4), <800..801>
│           │       │   │   │   └╼ RHS
│           │       │   │   │     └╼ Constant
│           │       │   │   │       └╼ Float -> Float(23.6) <805..810>
│           │       │   │   └╼ RHS
│           │       │   │     └╼ Constant
│           │       │   │       └╼ Integer -> Generic(45), <813..815>
│           │       │   └╼ RHS
│           │       │     └╼ Constant
│           │       │       └╼ Integer -> Generic(25), <819..821>
│           │       └╼ RHS
│           │         └╼ Constant
│           │           └╼ Integer -> Generic(0), <825..826>
│           └╼ ElseExpression
│             └╼ Constant
│               └╼ Integer -> Generic(77), <829..831>
├╼ FunctionDefinition <838..863>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ ├╼ FunctionSpecifier -> Inline <838..844>
│ │ │ └╼ TypeSpecifier -> Float <845..850>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "AdityaG" <851..858>
│ │   └╼ FunctionParameters
│ │     └╼ Empty
│ └╼ FunctionBody
│   └╼ Empty
├╼ Declaration <864..919>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ TypeQualifier -> Const <864..869>
│ │ └╼ TypeSpecifier -> Int <870..873>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ FunctionDeclarator
│       ├╼ Identifier -> "NiceFunction" <874..886>
│       └╼ FunctionParameters
│         ├╼ FunctionParameter <887..903>
│         │ ├╼ DeclarationSpecifiers
│         │ │ ├╼ TypeQualifier -> Const <887..892>
│         │ │ └╼ TypeSpecifier -> Int <893..896>
│         │ └╼ DirectDeclarator -> "param1" <897..903>
│         └╼ FunctionParameter <905..917>
│           ├╼ DeclarationSpecifiers
│           │ └╼ TypeSpecifier -> Float <905..910>
│           └╼ DirectDeclarator -> "param2" <911..917>
└╼ Declaration <920..968>
  ├╼ DeclarationSpecifiers
  │ ├╼ TypeSpecifier -> Unsigned <920..928>
  │ └╼ TypeSpecifier -> Int <929..932>
  └╼ InitDeclaratorList
    └╼ InitDeclarator
      └╼ FunctionDeclarator
        ├╼ Identifier -> "GetSum" <933..939>
        └╼ FunctionParameters
          ├╼ FunctionParameter <940..953>
          │ ├╼ DeclarationSpecifiers
          │ │ ├╼ TypeSpecifier -> Unsigned <940..948>
          │ │ └╼ TypeSpecifier -> Int <949..952>
          │ └╼ DirectDeclarator -> None
          └╼ FunctionParameter <954..967>
            ├╼ DeclarationSpecifiers
            │ ├╼ TypeSpecifier -> Unsigned <954..962>
            │ └╼ TypeSpecifier -> Int <963..966>
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
