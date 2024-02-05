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
    {
        case 2 * 32: return g_GlobalVariable * g_NiceVar;
        default: break;
    }
    int i = 0;
    while (i < 10)
    {
        while (1)
            return 2;
        int j = i + 1;
    }
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
├╼ FunctionDefinition <117..1032>
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
│   └╼ CompoundStatement <133..1028>
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
│     ├╼ CompoundStatement <588..670>
│     │ ├╼ CaseStatement <597..646>
│     │ │ ├╼ CaseExpression
│     │ │ │ └╼ BinaryOperatorExpression <602..608>
│     │ │ │   ├╼ Operator -> Multiply <604..605>
│     │ │ │   ├╼ LHS
│     │ │ │   │ └╼ Constant
│     │ │ │   │   └╼ Integer -> Generic(2), <602..603>
│     │ │ │   └╼ RHS
│     │ │ │     └╼ Constant
│     │ │ │       └╼ Integer -> Generic(32), <606..608>
│     │ │ └╼ CaseBlock
│     │ │   └╼ ReturnStatement <610..646>
│     │ │     └╼ BinaryOperatorExpression <610..646>
│     │ │       ├╼ Operator -> Multiply <634..635>
│     │ │       ├╼ LHS
│     │ │       │ └╼ Identifier -> "g_GlobalVariable" <617..633>
│     │ │       └╼ RHS
│     │ │         └╼ Identifier -> "g_NiceVar" <636..645>
│     │ └╼ DefaultStatement <655..670>
│     │   └╼ DefaultBlock
│     │     └╼ BreakStatement <664..670>
│     ├╼ Declaration <681..691>
│     │ ├╼ DeclarationSpecifiers
│     │ │ └╼ TypeSpecifier -> Int <681..684>
│     │ └╼ InitDeclaratorList
│     │   └╼ InitDeclarator
│     │     ├╼ DirectDeclarator -> "i" <685..686>
│     │     └╼ Initializer
│     │       └╼ Constant
│     │         └╼ Integer -> Generic(0), <689..690>
│     ├╼ WhileStatement <696..779>
│     │ ├╼ WhileExpression
│     │ │ └╼ BinaryOperatorExpression <703..709>
│     │ │   ├╼ Operator -> Less <705..706>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <703..704>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(10), <707..709>
│     │ └╼ WhileBlock
│     │   └╼ CompoundStatement <716..779>
│     │     ├╼ WhileStatement <725..756>
│     │     │ ├╼ WhileExpression
│     │     │ │ └╼ Constant
│     │     │ │   └╼ Integer -> Generic(1), <732..733>
│     │     │ └╼ WhileBlock
│     │     │   └╼ ReturnStatement <747..756>
│     │     │     └╼ Constant
│     │     │       └╼ Integer -> Generic(2), <747..756>
│     │     └╼ Declaration <765..779>
│     │       ├╼ DeclarationSpecifiers
│     │       │ └╼ TypeSpecifier -> Int <765..768>
│     │       └╼ InitDeclaratorList
│     │         └╼ InitDeclarator
│     │           ├╼ DirectDeclarator -> "j" <769..770>
│     │           └╼ Initializer
│     │             └╼ BinaryOperatorExpression <773..778>
│     │               ├╼ Operator -> Plus <775..776>
│     │               ├╼ LHS
│     │               │ └╼ Identifier -> "i" <773..774>
│     │               └╼ RHS
│     │                 └╼ Constant
│     │                   └╼ Integer -> Generic(1), <777..778>
│     └╼ CompoundStatement <854..1028>
│       └╼ ReturnStatement <855..1028>
│         └╼ TernaryOperatorExpression <855..1028>
│           ├╼ Condition
│           │ └╼ Identifier -> "_g_AssumeABoolean" <862..879>
│           ├╼ IfExpression
│           │ └╼ BinaryOperatorExpression <882..1022>
│           │   ├╼ Operator -> LogicalOr <903..905>
│           │   ├╼ LHS
│           │   │ └╼ BinaryOperatorExpression <882..902>
│           │   │   ├╼ Operator -> LogicalAnd <898..900>
│           │   │   ├╼ LHS
│           │   │   │ └╼ BinaryOperatorExpression <882..897>
│           │   │   │   ├╼ Operator -> BitwiseOr <894..895>
│           │   │   │   ├╼ LHS
│           │   │   │   │ └╼ BinaryOperatorExpression <882..893>
│           │   │   │   │   ├╼ Operator -> BitwiseXor <889..890>
│           │   │   │   │   ├╼ LHS
│           │   │   │   │   │ └╼ BinaryOperatorExpression <882..888>
│           │   │   │   │   │   ├╼ Operator -> BitwiseAnd <884..885>
│           │   │   │   │   │   ├╼ LHS
│           │   │   │   │   │   │ └╼ Constant
│           │   │   │   │   │   │   └╼ Integer -> Generic(4), <882..883>
│           │   │   │   │   │   └╼ RHS
│           │   │   │   │   │     └╼ Constant
│           │   │   │   │   │       └╼ Integer -> Generic(12), <886..888>
│           │   │   │   │   └╼ RHS
│           │   │   │   │     └╼ Constant
│           │   │   │   │       └╼ Integer -> Generic(13), <891..893>
│           │   │   │   └╼ RHS
│           │   │   │     └╼ Constant
│           │   │   │       └╼ Integer -> Generic(1), <896..897>
│           │   │   └╼ RHS
│           │   │     └╼ Constant
│           │   │       └╼ Integer -> Generic(5), <901..902>
│           │   └╼ RHS
│           │     └╼ BinaryOperatorExpression <907..1022>
│           │       ├╼ Operator -> NotEquals <1018..1020>
│           │       ├╼ LHS
│           │       │ └╼ BinaryOperatorExpression <907..1017>
│           │       │   ├╼ Operator -> GreaterOrEqual <1012..1014>
│           │       │   ├╼ LHS
│           │       │   │ └╼ BinaryOperatorExpression <907..1011>
│           │       │   │   ├╼ Operator -> Minus <1007..1008>
│           │       │   │   ├╼ LHS
│           │       │   │   │ └╼ BinaryOperatorExpression <907..1006>
│           │       │   │   │   ├╼ Operator -> Multiply <999..1000>
│           │       │   │   │   ├╼ LHS
│           │       │   │   │   │ └╼ BinaryOperatorExpression <907..997>
│           │       │   │   │   │   ├╼ Operator -> Multiply <940..941>
│           │       │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │ └╼ BinaryOperatorExpression <907..938>
│           │       │   │   │   │   │   ├╼ Operator -> BitwiseAnd <934..935>
│           │       │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │ └╼ BinaryOperatorExpression <907..933>
│           │       │   │   │   │   │   │   ├╼ Operator -> ShiftLeft <929..931>
│           │       │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │ └╼ BinaryOperatorExpression <907..928>
│           │       │   │   │   │   │   │   │   ├╼ Operator -> Plus <924..925>
│           │       │   │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │   │ └╼ Identifier -> "g_GlobalVariable" <907..923>
│           │       │   │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │   │       └╼ Integer -> Unsigned(2), <926..928>
│           │       │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │       └╼ Integer -> Generic(2), <932..933>
│           │       │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │       └╼ Integer -> Generic(31), <936..938>
│           │       │   │   │   │   └╼ RHS
│           │       │   │   │   │     └╼ BinaryOperatorExpression <945..997>
│           │       │   │   │   │       ├╼ Operator -> Multiply <994..995>
│           │       │   │   │   │       ├╼ LHS
│           │       │   │   │   │       │ └╼ BinaryOperatorExpression <945..992>
│           │       │   │   │   │       │   ├╼ Operator -> Multiply <988..989>
│           │       │   │   │   │       │   ├╼ LHS
│           │       │   │   │   │       │   │ └╼ BinaryOperatorExpression <945..986>
│           │       │   │   │   │       │   │   ├╼ Operator -> Equals <969..971>
│           │       │   │   │   │       │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │ └╼ BinaryOperatorExpression <945..968>
│           │       │   │   │   │       │   │   │   ├╼ Operator -> Plus <949..950>
│           │       │   │   │   │       │   │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │   │ └╼ Constant
│           │       │   │   │   │       │   │   │   │   └╼ Integer -> SignedLong(34), <945..948>
│           │       │   │   │   │       │   │   │   └╼ RHS
│           │       │   │   │   │       │   │   │     └╼ Identifier -> "_g_AssumeABoolean" <951..968>
│           │       │   │   │   │       │   │   └╼ RHS
│           │       │   │   │   │       │   │     └╼ BinaryOperatorExpression <972..986>
│           │       │   │   │   │       │   │       ├╼ Operator -> ShiftRight <982..984>
│           │       │   │   │   │       │   │       ├╼ LHS
│           │       │   │   │   │       │   │       │ └╼ Identifier -> "g_NiceVar" <972..981>
│           │       │   │   │   │       │   │       └╼ RHS
│           │       │   │   │   │       │   │         └╼ Constant
│           │       │   │   │   │       │   │           └╼ Integer -> Generic(3), <985..986>
│           │       │   │   │   │       │   └╼ RHS
│           │       │   │   │   │       │     └╼ Constant
│           │       │   │   │   │       │       └╼ Integer -> Generic(34), <990..992>
│           │       │   │   │   │       └╼ RHS
│           │       │   │   │   │         └╼ Constant
│           │       │   │   │   │           └╼ Integer -> Generic(4), <996..997>
│           │       │   │   │   └╼ RHS
│           │       │   │   │     └╼ Constant
│           │       │   │   │       └╼ Float -> Float(23.6) <1001..1006>
│           │       │   │   └╼ RHS
│           │       │   │     └╼ Constant
│           │       │   │       └╼ Integer -> Generic(45), <1009..1011>
│           │       │   └╼ RHS
│           │       │     └╼ Constant
│           │       │       └╼ Integer -> Generic(25), <1015..1017>
│           │       └╼ RHS
│           │         └╼ Constant
│           │           └╼ Integer -> Generic(0), <1021..1022>
│           └╼ ElseExpression
│             └╼ Constant
│               └╼ Integer -> Generic(77), <1025..1027>
├╼ FunctionDefinition <1034..1059>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ ├╼ FunctionSpecifier -> Inline <1034..1040>
│ │ │ └╼ TypeSpecifier -> Float <1041..1046>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "AdityaG" <1047..1054>
│ │   └╼ FunctionParameters
│ │     └╼ Empty
│ └╼ FunctionBody
│   └╼ Empty
├╼ Declaration <1060..1115>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ TypeQualifier -> Const <1060..1065>
│ │ └╼ TypeSpecifier -> Int <1066..1069>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ FunctionDeclarator
│       ├╼ Identifier -> "NiceFunction" <1070..1082>
│       └╼ FunctionParameters
│         ├╼ FunctionParameter <1083..1099>
│         │ ├╼ DeclarationSpecifiers
│         │ │ ├╼ TypeQualifier -> Const <1083..1088>
│         │ │ └╼ TypeSpecifier -> Int <1089..1092>
│         │ └╼ DirectDeclarator -> "param1" <1093..1099>
│         └╼ FunctionParameter <1101..1113>
│           ├╼ DeclarationSpecifiers
│           │ └╼ TypeSpecifier -> Float <1101..1106>
│           └╼ DirectDeclarator -> "param2" <1107..1113>
└╼ Declaration <1116..1164>
  ├╼ DeclarationSpecifiers
  │ ├╼ TypeSpecifier -> Unsigned <1116..1124>
  │ └╼ TypeSpecifier -> Int <1125..1128>
  └╼ InitDeclaratorList
    └╼ InitDeclarator
      └╼ FunctionDeclarator
        ├╼ Identifier -> "GetSum" <1129..1135>
        └╼ FunctionParameters
          ├╼ FunctionParameter <1136..1149>
          │ ├╼ DeclarationSpecifiers
          │ │ ├╼ TypeSpecifier -> Unsigned <1136..1144>
          │ │ └╼ TypeSpecifier -> Int <1145..1148>
          │ └╼ DirectDeclarator -> None
          └╼ FunctionParameter <1150..1163>
            ├╼ DeclarationSpecifiers
            │ ├╼ TypeSpecifier -> Unsigned <1150..1158>
            │ └╼ TypeSpecifier -> Int <1159..1162>
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
