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

    int i = 0;
    while (i < 10)
    {
        while (1)
            return 2;
        int j = i + 1;
    }

    switch (i * i + i)
    {
        case 2 * 32: return g_GlobalVariable * g_NiceVar;
        default: break;
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
├╼ FunctionDefinition <117..1057>
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
│   └╼ CompoundStatement <133..1053>
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
│     ├╼ Declaration <588..598>
│     │ ├╼ DeclarationSpecifiers
│     │ │ └╼ TypeSpecifier -> Int <588..591>
│     │ └╼ InitDeclaratorList
│     │   └╼ InitDeclarator
│     │     ├╼ DirectDeclarator -> "i" <592..593>
│     │     └╼ Initializer
│     │       └╼ Constant
│     │         └╼ Integer -> Generic(0), <596..597>
│     ├╼ WhileStatement <603..686>
│     │ ├╼ WhileExpression
│     │ │ └╼ BinaryOperatorExpression <610..616>
│     │ │   ├╼ Operator -> Less <612..613>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <610..611>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(10), <614..616>
│     │ └╼ WhileBlock
│     │   └╼ CompoundStatement <623..686>
│     │     ├╼ WhileStatement <632..663>
│     │     │ ├╼ WhileExpression
│     │     │ │ └╼ Constant
│     │     │ │   └╼ Integer -> Generic(1), <639..640>
│     │     │ └╼ WhileBlock
│     │     │   └╼ ReturnStatement <654..663>
│     │     │     └╼ Constant
│     │     │       └╼ Integer -> Generic(2), <654..663>
│     │     └╼ Declaration <672..686>
│     │       ├╼ DeclarationSpecifiers
│     │       │ └╼ TypeSpecifier -> Int <672..675>
│     │       └╼ InitDeclaratorList
│     │         └╼ InitDeclarator
│     │           ├╼ DirectDeclarator -> "j" <676..677>
│     │           └╼ Initializer
│     │             └╼ BinaryOperatorExpression <680..685>
│     │               ├╼ Operator -> Plus <682..683>
│     │               ├╼ LHS
│     │               │ └╼ Identifier -> "i" <680..681>
│     │               └╼ RHS
│     │                 └╼ Constant
│     │                   └╼ Integer -> Generic(1), <684..685>
│     ├╼ SwitchStatement <698..804>
│     │ ├╼ SwitchExpression
│     │ │ └╼ BinaryOperatorExpression <706..715>
│     │ │   ├╼ Operator -> Plus <712..713>
│     │ │   ├╼ LHS
│     │ │   │ └╼ BinaryOperatorExpression <706..711>
│     │ │   │   ├╼ Operator -> Multiply <708..709>
│     │ │   │   ├╼ LHS
│     │ │   │   │ └╼ Identifier -> "i" <706..707>
│     │ │   │   └╼ RHS
│     │ │   │     └╼ Identifier -> "i" <710..711>
│     │ │   └╼ RHS
│     │ │     └╼ Identifier -> "i" <714..715>
│     │ └╼ SwitchBlock
│     │   └╼ CompoundStatement <722..804>
│     │     ├╼ CaseStatement <731..780>
│     │     │ ├╼ CaseExpression
│     │     │ │ └╼ BinaryOperatorExpression <736..742>
│     │     │ │   ├╼ Operator -> Multiply <738..739>
│     │     │ │   ├╼ LHS
│     │     │ │   │ └╼ Constant
│     │     │ │   │   └╼ Integer -> Generic(2), <736..737>
│     │     │ │   └╼ RHS
│     │     │ │     └╼ Constant
│     │     │ │       └╼ Integer -> Generic(32), <740..742>
│     │     │ └╼ CaseBlock
│     │     │   └╼ ReturnStatement <744..780>
│     │     │     └╼ BinaryOperatorExpression <744..780>
│     │     │       ├╼ Operator -> Multiply <768..769>
│     │     │       ├╼ LHS
│     │     │       │ └╼ Identifier -> "g_GlobalVariable" <751..767>
│     │     │       └╼ RHS
│     │     │         └╼ Identifier -> "g_NiceVar" <770..779>
│     │     └╼ DefaultStatement <789..804>
│     │       └╼ DefaultBlock
│     │         └╼ BreakStatement <798..804>
│     └╼ CompoundStatement <879..1053>
│       └╼ ReturnStatement <880..1053>
│         └╼ TernaryOperatorExpression <880..1053>
│           ├╼ Condition
│           │ └╼ Identifier -> "_g_AssumeABoolean" <887..904>
│           ├╼ IfExpression
│           │ └╼ BinaryOperatorExpression <907..1047>
│           │   ├╼ Operator -> LogicalOr <928..930>
│           │   ├╼ LHS
│           │   │ └╼ BinaryOperatorExpression <907..927>
│           │   │   ├╼ Operator -> LogicalAnd <923..925>
│           │   │   ├╼ LHS
│           │   │   │ └╼ BinaryOperatorExpression <907..922>
│           │   │   │   ├╼ Operator -> BitwiseOr <919..920>
│           │   │   │   ├╼ LHS
│           │   │   │   │ └╼ BinaryOperatorExpression <907..918>
│           │   │   │   │   ├╼ Operator -> BitwiseXor <914..915>
│           │   │   │   │   ├╼ LHS
│           │   │   │   │   │ └╼ BinaryOperatorExpression <907..913>
│           │   │   │   │   │   ├╼ Operator -> BitwiseAnd <909..910>
│           │   │   │   │   │   ├╼ LHS
│           │   │   │   │   │   │ └╼ Constant
│           │   │   │   │   │   │   └╼ Integer -> Generic(4), <907..908>
│           │   │   │   │   │   └╼ RHS
│           │   │   │   │   │     └╼ Constant
│           │   │   │   │   │       └╼ Integer -> Generic(12), <911..913>
│           │   │   │   │   └╼ RHS
│           │   │   │   │     └╼ Constant
│           │   │   │   │       └╼ Integer -> Generic(13), <916..918>
│           │   │   │   └╼ RHS
│           │   │   │     └╼ Constant
│           │   │   │       └╼ Integer -> Generic(1), <921..922>
│           │   │   └╼ RHS
│           │   │     └╼ Constant
│           │   │       └╼ Integer -> Generic(5), <926..927>
│           │   └╼ RHS
│           │     └╼ BinaryOperatorExpression <932..1047>
│           │       ├╼ Operator -> NotEquals <1043..1045>
│           │       ├╼ LHS
│           │       │ └╼ BinaryOperatorExpression <932..1042>
│           │       │   ├╼ Operator -> GreaterOrEqual <1037..1039>
│           │       │   ├╼ LHS
│           │       │   │ └╼ BinaryOperatorExpression <932..1036>
│           │       │   │   ├╼ Operator -> Minus <1032..1033>
│           │       │   │   ├╼ LHS
│           │       │   │   │ └╼ BinaryOperatorExpression <932..1031>
│           │       │   │   │   ├╼ Operator -> Multiply <1024..1025>
│           │       │   │   │   ├╼ LHS
│           │       │   │   │   │ └╼ BinaryOperatorExpression <932..1022>
│           │       │   │   │   │   ├╼ Operator -> Multiply <965..966>
│           │       │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │ └╼ BinaryOperatorExpression <932..963>
│           │       │   │   │   │   │   ├╼ Operator -> BitwiseAnd <959..960>
│           │       │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │ └╼ BinaryOperatorExpression <932..958>
│           │       │   │   │   │   │   │   ├╼ Operator -> ShiftLeft <954..956>
│           │       │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │ └╼ BinaryOperatorExpression <932..953>
│           │       │   │   │   │   │   │   │   ├╼ Operator -> Plus <949..950>
│           │       │   │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │   │ └╼ Identifier -> "g_GlobalVariable" <932..948>
│           │       │   │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │   │       └╼ Integer -> Unsigned(2), <951..953>
│           │       │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │       └╼ Integer -> Generic(2), <957..958>
│           │       │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │       └╼ Integer -> Generic(31), <961..963>
│           │       │   │   │   │   └╼ RHS
│           │       │   │   │   │     └╼ BinaryOperatorExpression <970..1022>
│           │       │   │   │   │       ├╼ Operator -> Multiply <1019..1020>
│           │       │   │   │   │       ├╼ LHS
│           │       │   │   │   │       │ └╼ BinaryOperatorExpression <970..1017>
│           │       │   │   │   │       │   ├╼ Operator -> Multiply <1013..1014>
│           │       │   │   │   │       │   ├╼ LHS
│           │       │   │   │   │       │   │ └╼ BinaryOperatorExpression <970..1011>
│           │       │   │   │   │       │   │   ├╼ Operator -> Equals <994..996>
│           │       │   │   │   │       │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │ └╼ BinaryOperatorExpression <970..993>
│           │       │   │   │   │       │   │   │   ├╼ Operator -> Plus <974..975>
│           │       │   │   │   │       │   │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │   │ └╼ Constant
│           │       │   │   │   │       │   │   │   │   └╼ Integer -> SignedLong(34), <970..973>
│           │       │   │   │   │       │   │   │   └╼ RHS
│           │       │   │   │   │       │   │   │     └╼ Identifier -> "_g_AssumeABoolean" <976..993>
│           │       │   │   │   │       │   │   └╼ RHS
│           │       │   │   │   │       │   │     └╼ BinaryOperatorExpression <997..1011>
│           │       │   │   │   │       │   │       ├╼ Operator -> ShiftRight <1007..1009>
│           │       │   │   │   │       │   │       ├╼ LHS
│           │       │   │   │   │       │   │       │ └╼ Identifier -> "g_NiceVar" <997..1006>
│           │       │   │   │   │       │   │       └╼ RHS
│           │       │   │   │   │       │   │         └╼ Constant
│           │       │   │   │   │       │   │           └╼ Integer -> Generic(3), <1010..1011>
│           │       │   │   │   │       │   └╼ RHS
│           │       │   │   │   │       │     └╼ Constant
│           │       │   │   │   │       │       └╼ Integer -> Generic(34), <1015..1017>
│           │       │   │   │   │       └╼ RHS
│           │       │   │   │   │         └╼ Constant
│           │       │   │   │   │           └╼ Integer -> Generic(4), <1021..1022>
│           │       │   │   │   └╼ RHS
│           │       │   │   │     └╼ Constant
│           │       │   │   │       └╼ Float -> Float(23.6) <1026..1031>
│           │       │   │   └╼ RHS
│           │       │   │     └╼ Constant
│           │       │   │       └╼ Integer -> Generic(45), <1034..1036>
│           │       │   └╼ RHS
│           │       │     └╼ Constant
│           │       │       └╼ Integer -> Generic(25), <1040..1042>
│           │       └╼ RHS
│           │         └╼ Constant
│           │           └╼ Integer -> Generic(0), <1046..1047>
│           └╼ ElseExpression
│             └╼ Constant
│               └╼ Integer -> Generic(77), <1050..1052>
├╼ FunctionDefinition <1059..1084>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ ├╼ FunctionSpecifier -> Inline <1059..1065>
│ │ │ └╼ TypeSpecifier -> Float <1066..1071>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "AdityaG" <1072..1079>
│ │   └╼ FunctionParameters
│ │     └╼ Empty
│ └╼ FunctionBody
│   └╼ Empty
├╼ Declaration <1085..1140>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ TypeQualifier -> Const <1085..1090>
│ │ └╼ TypeSpecifier -> Int <1091..1094>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ FunctionDeclarator
│       ├╼ Identifier -> "NiceFunction" <1095..1107>
│       └╼ FunctionParameters
│         ├╼ FunctionParameter <1108..1124>
│         │ ├╼ DeclarationSpecifiers
│         │ │ ├╼ TypeQualifier -> Const <1108..1113>
│         │ │ └╼ TypeSpecifier -> Int <1114..1117>
│         │ └╼ DirectDeclarator -> "param1" <1118..1124>
│         └╼ FunctionParameter <1126..1138>
│           ├╼ DeclarationSpecifiers
│           │ └╼ TypeSpecifier -> Float <1126..1131>
│           └╼ DirectDeclarator -> "param2" <1132..1138>
└╼ Declaration <1141..1189>
  ├╼ DeclarationSpecifiers
  │ ├╼ TypeSpecifier -> Unsigned <1141..1149>
  │ └╼ TypeSpecifier -> Int <1150..1153>
  └╼ InitDeclaratorList
    └╼ InitDeclarator
      └╼ FunctionDeclarator
        ├╼ Identifier -> "GetSum" <1154..1160>
        └╼ FunctionParameters
          ├╼ FunctionParameter <1161..1174>
          │ ├╼ DeclarationSpecifiers
          │ │ ├╼ TypeSpecifier -> Unsigned <1161..1169>
          │ │ └╼ TypeSpecifier -> Int <1170..1173>
          │ └╼ DirectDeclarator -> None
          └╼ FunctionParameter <1175..1188>
            ├╼ DeclarationSpecifiers
            │ ├╼ TypeSpecifier -> Unsigned <1175..1183>
            │ └╼ TypeSpecifier -> Int <1184..1187>
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
