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
        do
        {
            char character = 'Z' + 'D';
        } while (i == 5 - 3);

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
├╼ FunctionDefinition <117..1149>
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
│   └╼ CompoundStatement <133..1145>
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
│     ├╼ WhileStatement <603..778>
│     │ ├╼ WhileExpression
│     │ │ └╼ BinaryOperatorExpression <610..616>
│     │ │   ├╼ Operator -> Less <612..613>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <610..611>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(10), <614..616>
│     │ └╼ WhileBlock
│     │   └╼ CompoundStatement <623..778>
│     │     ├╼ WhileStatement <632..663>
│     │     │ ├╼ WhileExpression
│     │     │ │ └╼ Constant
│     │     │ │   └╼ Integer -> Generic(1), <639..640>
│     │     │ └╼ WhileBlock
│     │     │   └╼ ReturnStatement <654..663>
│     │     │     └╼ Constant
│     │     │       └╼ Integer -> Generic(2), <654..663>
│     │     ├╼ DoWhileStatement <672..754>
│     │     │ ├╼ DoBlock
│     │     │ │ └╼ CompoundStatement <684..724>
│     │     │ │   └╼ Declaration <697..724>
│     │     │ │     ├╼ DeclarationSpecifiers
│     │     │ │     │ └╼ TypeSpecifier -> Char <697..701>
│     │     │ │     └╼ InitDeclaratorList
│     │     │ │       └╼ InitDeclarator
│     │     │ │         ├╼ DirectDeclarator -> "character" <702..711>
│     │     │ │         └╼ Initializer
│     │     │ │           └╼ BinaryOperatorExpression <714..723>
│     │     │ │             ├╼ Operator -> Plus <718..719>
│     │     │ │             ├╼ LHS
│     │     │ │             │ └╼ Constant
│     │     │ │             │   └╼ Character -> 'Z'
│     │     │ │             └╼ RHS
│     │     │ │               └╼ Constant
│     │     │ │                 └╼ Character -> 'D'
│     │     │ └╼ DoWhileExpression
│     │     │   └╼ BinaryOperatorExpression <742..752>
│     │     │     ├╼ Operator -> Equals <744..746>
│     │     │     ├╼ LHS
│     │     │     │ └╼ Identifier -> "i" <742..743>
│     │     │     └╼ RHS
│     │     │       └╼ BinaryOperatorExpression <747..752>
│     │     │         ├╼ Operator -> Minus <749..750>
│     │     │         ├╼ LHS
│     │     │         │ └╼ Constant
│     │     │         │   └╼ Integer -> Generic(5), <747..748>
│     │     │         └╼ RHS
│     │     │           └╼ Constant
│     │     │             └╼ Integer -> Generic(3), <751..752>
│     │     └╼ Declaration <764..778>
│     │       ├╼ DeclarationSpecifiers
│     │       │ └╼ TypeSpecifier -> Int <764..767>
│     │       └╼ InitDeclaratorList
│     │         └╼ InitDeclarator
│     │           ├╼ DirectDeclarator -> "j" <768..769>
│     │           └╼ Initializer
│     │             └╼ BinaryOperatorExpression <772..777>
│     │               ├╼ Operator -> Plus <774..775>
│     │               ├╼ LHS
│     │               │ └╼ Identifier -> "i" <772..773>
│     │               └╼ RHS
│     │                 └╼ Constant
│     │                   └╼ Integer -> Generic(1), <776..777>
│     ├╼ SwitchStatement <790..896>
│     │ ├╼ SwitchExpression
│     │ │ └╼ BinaryOperatorExpression <798..807>
│     │ │   ├╼ Operator -> Plus <804..805>
│     │ │   ├╼ LHS
│     │ │   │ └╼ BinaryOperatorExpression <798..803>
│     │ │   │   ├╼ Operator -> Multiply <800..801>
│     │ │   │   ├╼ LHS
│     │ │   │   │ └╼ Identifier -> "i" <798..799>
│     │ │   │   └╼ RHS
│     │ │   │     └╼ Identifier -> "i" <802..803>
│     │ │   └╼ RHS
│     │ │     └╼ Identifier -> "i" <806..807>
│     │ └╼ SwitchBlock
│     │   └╼ CompoundStatement <814..896>
│     │     ├╼ CaseStatement <823..872>
│     │     │ ├╼ CaseExpression
│     │     │ │ └╼ BinaryOperatorExpression <828..834>
│     │     │ │   ├╼ Operator -> Multiply <830..831>
│     │     │ │   ├╼ LHS
│     │     │ │   │ └╼ Constant
│     │     │ │   │   └╼ Integer -> Generic(2), <828..829>
│     │     │ │   └╼ RHS
│     │     │ │     └╼ Constant
│     │     │ │       └╼ Integer -> Generic(32), <832..834>
│     │     │ └╼ CaseBlock
│     │     │   └╼ ReturnStatement <836..872>
│     │     │     └╼ BinaryOperatorExpression <836..872>
│     │     │       ├╼ Operator -> Multiply <860..861>
│     │     │       ├╼ LHS
│     │     │       │ └╼ Identifier -> "g_GlobalVariable" <843..859>
│     │     │       └╼ RHS
│     │     │         └╼ Identifier -> "g_NiceVar" <862..871>
│     │     └╼ DefaultStatement <881..896>
│     │       └╼ DefaultBlock
│     │         └╼ BreakStatement <890..896>
│     └╼ CompoundStatement <971..1145>
│       └╼ ReturnStatement <972..1145>
│         └╼ TernaryOperatorExpression <972..1145>
│           ├╼ Condition
│           │ └╼ Identifier -> "_g_AssumeABoolean" <979..996>
│           ├╼ IfExpression
│           │ └╼ BinaryOperatorExpression <999..1139>
│           │   ├╼ Operator -> LogicalOr <1020..1022>
│           │   ├╼ LHS
│           │   │ └╼ BinaryOperatorExpression <999..1019>
│           │   │   ├╼ Operator -> LogicalAnd <1015..1017>
│           │   │   ├╼ LHS
│           │   │   │ └╼ BinaryOperatorExpression <999..1014>
│           │   │   │   ├╼ Operator -> BitwiseOr <1011..1012>
│           │   │   │   ├╼ LHS
│           │   │   │   │ └╼ BinaryOperatorExpression <999..1010>
│           │   │   │   │   ├╼ Operator -> BitwiseXor <1006..1007>
│           │   │   │   │   ├╼ LHS
│           │   │   │   │   │ └╼ BinaryOperatorExpression <999..1005>
│           │   │   │   │   │   ├╼ Operator -> BitwiseAnd <1001..1002>
│           │   │   │   │   │   ├╼ LHS
│           │   │   │   │   │   │ └╼ Constant
│           │   │   │   │   │   │   └╼ Integer -> Generic(4), <999..1000>
│           │   │   │   │   │   └╼ RHS
│           │   │   │   │   │     └╼ Constant
│           │   │   │   │   │       └╼ Integer -> Generic(12), <1003..1005>
│           │   │   │   │   └╼ RHS
│           │   │   │   │     └╼ Constant
│           │   │   │   │       └╼ Integer -> Generic(13), <1008..1010>
│           │   │   │   └╼ RHS
│           │   │   │     └╼ Constant
│           │   │   │       └╼ Integer -> Generic(1), <1013..1014>
│           │   │   └╼ RHS
│           │   │     └╼ Constant
│           │   │       └╼ Integer -> Generic(5), <1018..1019>
│           │   └╼ RHS
│           │     └╼ BinaryOperatorExpression <1024..1139>
│           │       ├╼ Operator -> NotEquals <1135..1137>
│           │       ├╼ LHS
│           │       │ └╼ BinaryOperatorExpression <1024..1134>
│           │       │   ├╼ Operator -> GreaterOrEqual <1129..1131>
│           │       │   ├╼ LHS
│           │       │   │ └╼ BinaryOperatorExpression <1024..1128>
│           │       │   │   ├╼ Operator -> Minus <1124..1125>
│           │       │   │   ├╼ LHS
│           │       │   │   │ └╼ BinaryOperatorExpression <1024..1123>
│           │       │   │   │   ├╼ Operator -> Multiply <1116..1117>
│           │       │   │   │   ├╼ LHS
│           │       │   │   │   │ └╼ BinaryOperatorExpression <1024..1114>
│           │       │   │   │   │   ├╼ Operator -> Multiply <1057..1058>
│           │       │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │ └╼ BinaryOperatorExpression <1024..1055>
│           │       │   │   │   │   │   ├╼ Operator -> BitwiseAnd <1051..1052>
│           │       │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │ └╼ BinaryOperatorExpression <1024..1050>
│           │       │   │   │   │   │   │   ├╼ Operator -> ShiftLeft <1046..1048>
│           │       │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │ └╼ BinaryOperatorExpression <1024..1045>
│           │       │   │   │   │   │   │   │   ├╼ Operator -> Plus <1041..1042>
│           │       │   │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │   │ └╼ Identifier -> "g_GlobalVariable" <1024..1040>
│           │       │   │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │   │       └╼ Integer -> Unsigned(2), <1043..1045>
│           │       │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │       └╼ Integer -> Generic(2), <1049..1050>
│           │       │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │       └╼ Integer -> Generic(31), <1053..1055>
│           │       │   │   │   │   └╼ RHS
│           │       │   │   │   │     └╼ BinaryOperatorExpression <1062..1114>
│           │       │   │   │   │       ├╼ Operator -> Multiply <1111..1112>
│           │       │   │   │   │       ├╼ LHS
│           │       │   │   │   │       │ └╼ BinaryOperatorExpression <1062..1109>
│           │       │   │   │   │       │   ├╼ Operator -> Multiply <1105..1106>
│           │       │   │   │   │       │   ├╼ LHS
│           │       │   │   │   │       │   │ └╼ BinaryOperatorExpression <1062..1103>
│           │       │   │   │   │       │   │   ├╼ Operator -> Equals <1086..1088>
│           │       │   │   │   │       │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │ └╼ BinaryOperatorExpression <1062..1085>
│           │       │   │   │   │       │   │   │   ├╼ Operator -> Plus <1066..1067>
│           │       │   │   │   │       │   │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │   │ └╼ Constant
│           │       │   │   │   │       │   │   │   │   └╼ Integer -> SignedLong(34), <1062..1065>
│           │       │   │   │   │       │   │   │   └╼ RHS
│           │       │   │   │   │       │   │   │     └╼ Identifier -> "_g_AssumeABoolean" <1068..1085>
│           │       │   │   │   │       │   │   └╼ RHS
│           │       │   │   │   │       │   │     └╼ BinaryOperatorExpression <1089..1103>
│           │       │   │   │   │       │   │       ├╼ Operator -> ShiftRight <1099..1101>
│           │       │   │   │   │       │   │       ├╼ LHS
│           │       │   │   │   │       │   │       │ └╼ Identifier -> "g_NiceVar" <1089..1098>
│           │       │   │   │   │       │   │       └╼ RHS
│           │       │   │   │   │       │   │         └╼ Constant
│           │       │   │   │   │       │   │           └╼ Integer -> Generic(3), <1102..1103>
│           │       │   │   │   │       │   └╼ RHS
│           │       │   │   │   │       │     └╼ Constant
│           │       │   │   │   │       │       └╼ Integer -> Generic(34), <1107..1109>
│           │       │   │   │   │       └╼ RHS
│           │       │   │   │   │         └╼ Constant
│           │       │   │   │   │           └╼ Integer -> Generic(4), <1113..1114>
│           │       │   │   │   └╼ RHS
│           │       │   │   │     └╼ Constant
│           │       │   │   │       └╼ Float -> Float(23.6) <1118..1123>
│           │       │   │   └╼ RHS
│           │       │   │     └╼ Constant
│           │       │   │       └╼ Integer -> Generic(45), <1126..1128>
│           │       │   └╼ RHS
│           │       │     └╼ Constant
│           │       │       └╼ Integer -> Generic(25), <1132..1134>
│           │       └╼ RHS
│           │         └╼ Constant
│           │           └╼ Integer -> Generic(0), <1138..1139>
│           └╼ ElseExpression
│             └╼ Constant
│               └╼ Integer -> Generic(77), <1142..1144>
├╼ FunctionDefinition <1151..1176>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ ├╼ FunctionSpecifier -> Inline <1151..1157>
│ │ │ └╼ TypeSpecifier -> Float <1158..1163>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "AdityaG" <1164..1171>
│ │   └╼ FunctionParameters
│ │     └╼ Empty
│ └╼ FunctionBody
│   └╼ Empty
├╼ Declaration <1177..1232>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ TypeQualifier -> Const <1177..1182>
│ │ └╼ TypeSpecifier -> Int <1183..1186>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ FunctionDeclarator
│       ├╼ Identifier -> "NiceFunction" <1187..1199>
│       └╼ FunctionParameters
│         ├╼ FunctionParameter <1200..1216>
│         │ ├╼ DeclarationSpecifiers
│         │ │ ├╼ TypeQualifier -> Const <1200..1205>
│         │ │ └╼ TypeSpecifier -> Int <1206..1209>
│         │ └╼ DirectDeclarator -> "param1" <1210..1216>
│         └╼ FunctionParameter <1218..1230>
│           ├╼ DeclarationSpecifiers
│           │ └╼ TypeSpecifier -> Float <1218..1223>
│           └╼ DirectDeclarator -> "param2" <1224..1230>
└╼ Declaration <1233..1281>
  ├╼ DeclarationSpecifiers
  │ ├╼ TypeSpecifier -> Unsigned <1233..1241>
  │ └╼ TypeSpecifier -> Int <1242..1245>
  └╼ InitDeclaratorList
    └╼ InitDeclarator
      └╼ FunctionDeclarator
        ├╼ Identifier -> "GetSum" <1246..1252>
        └╼ FunctionParameters
          ├╼ FunctionParameter <1253..1266>
          │ ├╼ DeclarationSpecifiers
          │ │ ├╼ TypeSpecifier -> Unsigned <1253..1261>
          │ │ └╼ TypeSpecifier -> Int <1262..1265>
          │ └╼ DirectDeclarator -> None
          └╼ FunctionParameter <1267..1280>
            ├╼ DeclarationSpecifiers
            │ ├╼ TypeSpecifier -> Unsigned <1267..1275>
            │ └╼ TypeSpecifier -> Int <1276..1279>
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
