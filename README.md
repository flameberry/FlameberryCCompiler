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

    for (int i = 0; i < 12; i - 1)
    {
        for (int j = 2; i > 0; i + 4)
            continue;

        int counter = 45;
        return 20 * counter;
    }

    2 * 2;
    ;
    goto Checkpoint;

Checkpoint: return 34;

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
├╼ FunctionDefinition <117..1376>
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
│   └╼ CompoundStatement <133..1372>
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
│     ├╼ ForStatement <790..942>
│     │ ├╼ ForInitializer
│     │ │ └╼ Declaration <795..805>
│     │ │   ├╼ DeclarationSpecifiers
│     │ │   │ └╼ TypeSpecifier -> Int <795..798>
│     │ │   └╼ InitDeclaratorList
│     │ │     └╼ InitDeclarator
│     │ │       ├╼ DirectDeclarator -> "i" <799..800>
│     │ │       └╼ Initializer
│     │ │         └╼ Constant
│     │ │           └╼ Integer -> Generic(0), <803..804>
│     │ ├╼ ForCondition
│     │ │ └╼ BinaryOperatorExpression <806..812>
│     │ │   ├╼ Operator -> Less <808..809>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <806..807>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(12), <810..812>
│     │ ├╼ ForStepExpression
│     │ │ └╼ BinaryOperatorExpression <814..819>
│     │ │   ├╼ Operator -> Minus <816..817>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <814..815>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(1), <818..819>
│     │ └╼ ForBlock
│     │   └╼ CompoundStatement <826..942>
│     │     ├╼ ForStatement <835..886>
│     │     │ ├╼ ForInitializer
│     │     │ │ └╼ Declaration <840..850>
│     │     │ │   ├╼ DeclarationSpecifiers
│     │     │ │   │ └╼ TypeSpecifier -> Int <840..843>
│     │     │ │   └╼ InitDeclaratorList
│     │     │ │     └╼ InitDeclarator
│     │     │ │       ├╼ DirectDeclarator -> "j" <844..845>
│     │     │ │       └╼ Initializer
│     │     │ │         └╼ Constant
│     │     │ │           └╼ Integer -> Generic(2), <848..849>
│     │     │ ├╼ ForCondition
│     │     │ │ └╼ BinaryOperatorExpression <851..856>
│     │     │ │   ├╼ Operator -> Greater <853..854>
│     │     │ │   ├╼ LHS
│     │     │ │   │ └╼ Identifier -> "i" <851..852>
│     │     │ │   └╼ RHS
│     │     │ │     └╼ Constant
│     │     │ │       └╼ Integer -> Generic(0), <855..856>
│     │     │ ├╼ ForStepExpression
│     │     │ │ └╼ BinaryOperatorExpression <858..863>
│     │     │ │   ├╼ Operator -> Plus <860..861>
│     │     │ │   ├╼ LHS
│     │     │ │   │ └╼ Identifier -> "i" <858..859>
│     │     │ │   └╼ RHS
│     │     │ │     └╼ Constant
│     │     │ │       └╼ Integer -> Generic(4), <862..863>
│     │     │ └╼ ForBlock
│     │     │   └╼ ContinueStatement <877..886>
│     │     ├╼ Declaration <896..913>
│     │     │ ├╼ DeclarationSpecifiers
│     │     │ │ └╼ TypeSpecifier -> Int <896..899>
│     │     │ └╼ InitDeclaratorList
│     │     │   └╼ InitDeclarator
│     │     │     ├╼ DirectDeclarator -> "counter" <900..907>
│     │     │     └╼ Initializer
│     │     │       └╼ Constant
│     │     │         └╼ Integer -> Generic(45), <910..912>
│     │     └╼ ReturnStatement <922..942>
│     │       └╼ BinaryOperatorExpression <922..942>
│     │         ├╼ Operator -> Multiply <932..933>
│     │         ├╼ LHS
│     │         │ └╼ Constant
│     │         │   └╼ Integer -> Generic(20), <929..931>
│     │         └╼ RHS
│     │           └╼ Identifier -> "counter" <934..941>
│     ├╼ ExpressionStatement <954..960>
│     │ └╼ Expression
│     │   └╼ BinaryOperatorExpression <954..959>
│     │     ├╼ Operator -> Multiply <956..957>
│     │     ├╼ LHS
│     │     │ └╼ Constant
│     │     │   └╼ Integer -> Generic(2), <954..955>
│     │     └╼ RHS
│     │       └╼ Constant
│     │         └╼ Integer -> Generic(2), <958..959>
│     ├╼ ExpressionStatement <965..966>
│     │ └╼ Expression
│     │   └╼ Empty
│     ├╼ GotoStatement <971..987>
│     │ └╼ Identifier -> "Checkpoint" <976..986>
│     ├╼ LabeledStatement <989..1011>
│     │ ├╼ Identifier -> "Checkpoint" <989..999>
│     │ └╼ LabeledBlock
│     │   └╼ ReturnStatement <1001..1011>
│     │     └╼ Constant
│     │       └╼ Integer -> Generic(34), <1001..1011>
│     ├╼ SwitchStatement <1017..1123>
│     │ ├╼ SwitchExpression
│     │ │ └╼ BinaryOperatorExpression <1025..1034>
│     │ │   ├╼ Operator -> Plus <1031..1032>
│     │ │   ├╼ LHS
│     │ │   │ └╼ BinaryOperatorExpression <1025..1030>
│     │ │   │   ├╼ Operator -> Multiply <1027..1028>
│     │ │   │   ├╼ LHS
│     │ │   │   │ └╼ Identifier -> "i" <1025..1026>
│     │ │   │   └╼ RHS
│     │ │   │     └╼ Identifier -> "i" <1029..1030>
│     │ │   └╼ RHS
│     │ │     └╼ Identifier -> "i" <1033..1034>
│     │ └╼ SwitchBlock
│     │   └╼ CompoundStatement <1041..1123>
│     │     ├╼ CaseStatement <1050..1099>
│     │     │ ├╼ CaseExpression
│     │     │ │ └╼ BinaryOperatorExpression <1055..1061>
│     │     │ │   ├╼ Operator -> Multiply <1057..1058>
│     │     │ │   ├╼ LHS
│     │     │ │   │ └╼ Constant
│     │     │ │   │   └╼ Integer -> Generic(2), <1055..1056>
│     │     │ │   └╼ RHS
│     │     │ │     └╼ Constant
│     │     │ │       └╼ Integer -> Generic(32), <1059..1061>
│     │     │ └╼ CaseBlock
│     │     │   └╼ ReturnStatement <1063..1099>
│     │     │     └╼ BinaryOperatorExpression <1063..1099>
│     │     │       ├╼ Operator -> Multiply <1087..1088>
│     │     │       ├╼ LHS
│     │     │       │ └╼ Identifier -> "g_GlobalVariable" <1070..1086>
│     │     │       └╼ RHS
│     │     │         └╼ Identifier -> "g_NiceVar" <1089..1098>
│     │     └╼ DefaultStatement <1108..1123>
│     │       └╼ DefaultBlock
│     │         └╼ BreakStatement <1117..1123>
│     └╼ CompoundStatement <1198..1372>
│       └╼ ReturnStatement <1199..1372>
│         └╼ TernaryOperatorExpression <1199..1372>
│           ├╼ Condition
│           │ └╼ Identifier -> "_g_AssumeABoolean" <1206..1223>
│           ├╼ IfExpression
│           │ └╼ BinaryOperatorExpression <1226..1366>
│           │   ├╼ Operator -> LogicalOr <1247..1249>
│           │   ├╼ LHS
│           │   │ └╼ BinaryOperatorExpression <1226..1246>
│           │   │   ├╼ Operator -> LogicalAnd <1242..1244>
│           │   │   ├╼ LHS
│           │   │   │ └╼ BinaryOperatorExpression <1226..1241>
│           │   │   │   ├╼ Operator -> BitwiseOr <1238..1239>
│           │   │   │   ├╼ LHS
│           │   │   │   │ └╼ BinaryOperatorExpression <1226..1237>
│           │   │   │   │   ├╼ Operator -> BitwiseXor <1233..1234>
│           │   │   │   │   ├╼ LHS
│           │   │   │   │   │ └╼ BinaryOperatorExpression <1226..1232>
│           │   │   │   │   │   ├╼ Operator -> BitwiseAnd <1228..1229>
│           │   │   │   │   │   ├╼ LHS
│           │   │   │   │   │   │ └╼ Constant
│           │   │   │   │   │   │   └╼ Integer -> Generic(4), <1226..1227>
│           │   │   │   │   │   └╼ RHS
│           │   │   │   │   │     └╼ Constant
│           │   │   │   │   │       └╼ Integer -> Generic(12), <1230..1232>
│           │   │   │   │   └╼ RHS
│           │   │   │   │     └╼ Constant
│           │   │   │   │       └╼ Integer -> Generic(13), <1235..1237>
│           │   │   │   └╼ RHS
│           │   │   │     └╼ Constant
│           │   │   │       └╼ Integer -> Generic(1), <1240..1241>
│           │   │   └╼ RHS
│           │   │     └╼ Constant
│           │   │       └╼ Integer -> Generic(5), <1245..1246>
│           │   └╼ RHS
│           │     └╼ BinaryOperatorExpression <1251..1366>
│           │       ├╼ Operator -> NotEquals <1362..1364>
│           │       ├╼ LHS
│           │       │ └╼ BinaryOperatorExpression <1251..1361>
│           │       │   ├╼ Operator -> GreaterOrEqual <1356..1358>
│           │       │   ├╼ LHS
│           │       │   │ └╼ BinaryOperatorExpression <1251..1355>
│           │       │   │   ├╼ Operator -> Minus <1351..1352>
│           │       │   │   ├╼ LHS
│           │       │   │   │ └╼ BinaryOperatorExpression <1251..1350>
│           │       │   │   │   ├╼ Operator -> Multiply <1343..1344>
│           │       │   │   │   ├╼ LHS
│           │       │   │   │   │ └╼ BinaryOperatorExpression <1251..1341>
│           │       │   │   │   │   ├╼ Operator -> Multiply <1284..1285>
│           │       │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │ └╼ BinaryOperatorExpression <1251..1282>
│           │       │   │   │   │   │   ├╼ Operator -> BitwiseAnd <1278..1279>
│           │       │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │ └╼ BinaryOperatorExpression <1251..1277>
│           │       │   │   │   │   │   │   ├╼ Operator -> ShiftLeft <1273..1275>
│           │       │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │ └╼ BinaryOperatorExpression <1251..1272>
│           │       │   │   │   │   │   │   │   ├╼ Operator -> Plus <1268..1269>
│           │       │   │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │   │ └╼ Identifier -> "g_GlobalVariable" <1251..1267>
│           │       │   │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │   │       └╼ Integer -> Unsigned(2), <1270..1272>
│           │       │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │       └╼ Integer -> Generic(2), <1276..1277>
│           │       │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │       └╼ Integer -> Generic(31), <1280..1282>
│           │       │   │   │   │   └╼ RHS
│           │       │   │   │   │     └╼ BinaryOperatorExpression <1289..1341>
│           │       │   │   │   │       ├╼ Operator -> Multiply <1338..1339>
│           │       │   │   │   │       ├╼ LHS
│           │       │   │   │   │       │ └╼ BinaryOperatorExpression <1289..1336>
│           │       │   │   │   │       │   ├╼ Operator -> Multiply <1332..1333>
│           │       │   │   │   │       │   ├╼ LHS
│           │       │   │   │   │       │   │ └╼ BinaryOperatorExpression <1289..1330>
│           │       │   │   │   │       │   │   ├╼ Operator -> Equals <1313..1315>
│           │       │   │   │   │       │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │ └╼ BinaryOperatorExpression <1289..1312>
│           │       │   │   │   │       │   │   │   ├╼ Operator -> Plus <1293..1294>
│           │       │   │   │   │       │   │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │   │ └╼ Constant
│           │       │   │   │   │       │   │   │   │   └╼ Integer -> SignedLong(34), <1289..1292>
│           │       │   │   │   │       │   │   │   └╼ RHS
│           │       │   │   │   │       │   │   │     └╼ Identifier -> "_g_AssumeABoolean" <1295..1312>
│           │       │   │   │   │       │   │   └╼ RHS
│           │       │   │   │   │       │   │     └╼ BinaryOperatorExpression <1316..1330>
│           │       │   │   │   │       │   │       ├╼ Operator -> ShiftRight <1326..1328>
│           │       │   │   │   │       │   │       ├╼ LHS
│           │       │   │   │   │       │   │       │ └╼ Identifier -> "g_NiceVar" <1316..1325>
│           │       │   │   │   │       │   │       └╼ RHS
│           │       │   │   │   │       │   │         └╼ Constant
│           │       │   │   │   │       │   │           └╼ Integer -> Generic(3), <1329..1330>
│           │       │   │   │   │       │   └╼ RHS
│           │       │   │   │   │       │     └╼ Constant
│           │       │   │   │   │       │       └╼ Integer -> Generic(34), <1334..1336>
│           │       │   │   │   │       └╼ RHS
│           │       │   │   │   │         └╼ Constant
│           │       │   │   │   │           └╼ Integer -> Generic(4), <1340..1341>
│           │       │   │   │   └╼ RHS
│           │       │   │   │     └╼ Constant
│           │       │   │   │       └╼ Float -> Float(23.6) <1345..1350>
│           │       │   │   └╼ RHS
│           │       │   │     └╼ Constant
│           │       │   │       └╼ Integer -> Generic(45), <1353..1355>
│           │       │   └╼ RHS
│           │       │     └╼ Constant
│           │       │       └╼ Integer -> Generic(25), <1359..1361>
│           │       └╼ RHS
│           │         └╼ Constant
│           │           └╼ Integer -> Generic(0), <1365..1366>
│           └╼ ElseExpression
│             └╼ Constant
│               └╼ Integer -> Generic(77), <1369..1371>
├╼ FunctionDefinition <1378..1403>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ ├╼ FunctionSpecifier -> Inline <1378..1384>
│ │ │ └╼ TypeSpecifier -> Float <1385..1390>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "AdityaG" <1391..1398>
│ │   └╼ FunctionParameters
│ │     └╼ Empty
│ └╼ FunctionBody
│   └╼ Empty
├╼ Declaration <1404..1459>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ TypeQualifier -> Const <1404..1409>
│ │ └╼ TypeSpecifier -> Int <1410..1413>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ FunctionDeclarator
│       ├╼ Identifier -> "NiceFunction" <1414..1426>
│       └╼ FunctionParameters
│         ├╼ FunctionParameter <1427..1443>
│         │ ├╼ DeclarationSpecifiers
│         │ │ ├╼ TypeQualifier -> Const <1427..1432>
│         │ │ └╼ TypeSpecifier -> Int <1433..1436>
│         │ └╼ DirectDeclarator -> "param1" <1437..1443>
│         └╼ FunctionParameter <1445..1457>
│           ├╼ DeclarationSpecifiers
│           │ └╼ TypeSpecifier -> Float <1445..1450>
│           └╼ DirectDeclarator -> "param2" <1451..1457>
└╼ Declaration <1460..1508>
  ├╼ DeclarationSpecifiers
  │ ├╼ TypeSpecifier -> Unsigned <1460..1468>
  │ └╼ TypeSpecifier -> Int <1469..1472>
  └╼ InitDeclaratorList
    └╼ InitDeclarator
      └╼ FunctionDeclarator
        ├╼ Identifier -> "GetSum" <1473..1479>
        └╼ FunctionParameters
          ├╼ FunctionParameter <1480..1493>
          │ ├╼ DeclarationSpecifiers
          │ │ ├╼ TypeSpecifier -> Unsigned <1480..1488>
          │ │ └╼ TypeSpecifier -> Int <1489..1492>
          │ └╼ DirectDeclarator -> None
          └╼ FunctionParameter <1494..1507>
            ├╼ DeclarationSpecifiers
            │ ├╼ TypeSpecifier -> Unsigned <1494..1502>
            │ └╼ TypeSpecifier -> Int <1503..1506>
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
