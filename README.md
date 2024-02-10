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
        for (;;)
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
├╼ FunctionDefinition <117..1355>
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
│   └╼ CompoundStatement <133..1351>
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
│     ├╼ ForStatement <790..921>
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
│     │   └╼ CompoundStatement <826..921>
│     │     ├╼ ForStatement <835..865>
│     │     │ ├╼ ForInitializer
│     │     │ │ └╼ Empty
│     │     │ ├╼ ForCondition
│     │     │ │ └╼ Empty
│     │     │ ├╼ ForStepExpression
│     │     │ │ └╼ Empty
│     │     │ └╼ ForBlock
│     │     │   └╼ ContinueStatement <856..865>
│     │     ├╼ Declaration <875..892>
│     │     │ ├╼ DeclarationSpecifiers
│     │     │ │ └╼ TypeSpecifier -> Int <875..878>
│     │     │ └╼ InitDeclaratorList
│     │     │   └╼ InitDeclarator
│     │     │     ├╼ DirectDeclarator -> "counter" <879..886>
│     │     │     └╼ Initializer
│     │     │       └╼ Constant
│     │     │         └╼ Integer -> Generic(45), <889..891>
│     │     └╼ ReturnStatement <901..921>
│     │       └╼ BinaryOperatorExpression <901..921>
│     │         ├╼ Operator -> Multiply <911..912>
│     │         ├╼ LHS
│     │         │ └╼ Constant
│     │         │   └╼ Integer -> Generic(20), <908..910>
│     │         └╼ RHS
│     │           └╼ Identifier -> "counter" <913..920>
│     ├╼ ExpressionStatement <933..939>
│     │ └╼ Expression
│     │   └╼ BinaryOperatorExpression <933..938>
│     │     ├╼ Operator -> Multiply <935..936>
│     │     ├╼ LHS
│     │     │ └╼ Constant
│     │     │   └╼ Integer -> Generic(2), <933..934>
│     │     └╼ RHS
│     │       └╼ Constant
│     │         └╼ Integer -> Generic(2), <937..938>
│     ├╼ ExpressionStatement <944..945>
│     │ └╼ Expression
│     │   └╼ Empty
│     ├╼ GotoStatement <950..966>
│     │ └╼ Identifier -> "Checkpoint" <955..965>
│     ├╼ LabeledStatement <968..990>
│     │ ├╼ Identifier -> "Checkpoint" <968..978>
│     │ └╼ LabeledBlock
│     │   └╼ ReturnStatement <980..990>
│     │     └╼ Constant
│     │       └╼ Integer -> Generic(34), <980..990>
│     ├╼ SwitchStatement <996..1102>
│     │ ├╼ SwitchExpression
│     │ │ └╼ BinaryOperatorExpression <1004..1013>
│     │ │   ├╼ Operator -> Plus <1010..1011>
│     │ │   ├╼ LHS
│     │ │   │ └╼ BinaryOperatorExpression <1004..1009>
│     │ │   │   ├╼ Operator -> Multiply <1006..1007>
│     │ │   │   ├╼ LHS
│     │ │   │   │ └╼ Identifier -> "i" <1004..1005>
│     │ │   │   └╼ RHS
│     │ │   │     └╼ Identifier -> "i" <1008..1009>
│     │ │   └╼ RHS
│     │ │     └╼ Identifier -> "i" <1012..1013>
│     │ └╼ SwitchBlock
│     │   └╼ CompoundStatement <1020..1102>
│     │     ├╼ CaseStatement <1029..1078>
│     │     │ ├╼ CaseExpression
│     │     │ │ └╼ BinaryOperatorExpression <1034..1040>
│     │     │ │   ├╼ Operator -> Multiply <1036..1037>
│     │     │ │   ├╼ LHS
│     │     │ │   │ └╼ Constant
│     │     │ │   │   └╼ Integer -> Generic(2), <1034..1035>
│     │     │ │   └╼ RHS
│     │     │ │     └╼ Constant
│     │     │ │       └╼ Integer -> Generic(32), <1038..1040>
│     │     │ └╼ CaseBlock
│     │     │   └╼ ReturnStatement <1042..1078>
│     │     │     └╼ BinaryOperatorExpression <1042..1078>
│     │     │       ├╼ Operator -> Multiply <1066..1067>
│     │     │       ├╼ LHS
│     │     │       │ └╼ Identifier -> "g_GlobalVariable" <1049..1065>
│     │     │       └╼ RHS
│     │     │         └╼ Identifier -> "g_NiceVar" <1068..1077>
│     │     └╼ DefaultStatement <1087..1102>
│     │       └╼ DefaultBlock
│     │         └╼ BreakStatement <1096..1102>
│     └╼ CompoundStatement <1177..1351>
│       └╼ ReturnStatement <1178..1351>
│         └╼ TernaryOperatorExpression <1178..1351>
│           ├╼ Condition
│           │ └╼ Identifier -> "_g_AssumeABoolean" <1185..1202>
│           ├╼ IfExpression
│           │ └╼ BinaryOperatorExpression <1205..1345>
│           │   ├╼ Operator -> LogicalOr <1226..1228>
│           │   ├╼ LHS
│           │   │ └╼ BinaryOperatorExpression <1205..1225>
│           │   │   ├╼ Operator -> LogicalAnd <1221..1223>
│           │   │   ├╼ LHS
│           │   │   │ └╼ BinaryOperatorExpression <1205..1220>
│           │   │   │   ├╼ Operator -> BitwiseOr <1217..1218>
│           │   │   │   ├╼ LHS
│           │   │   │   │ └╼ BinaryOperatorExpression <1205..1216>
│           │   │   │   │   ├╼ Operator -> BitwiseXor <1212..1213>
│           │   │   │   │   ├╼ LHS
│           │   │   │   │   │ └╼ BinaryOperatorExpression <1205..1211>
│           │   │   │   │   │   ├╼ Operator -> BitwiseAnd <1207..1208>
│           │   │   │   │   │   ├╼ LHS
│           │   │   │   │   │   │ └╼ Constant
│           │   │   │   │   │   │   └╼ Integer -> Generic(4), <1205..1206>
│           │   │   │   │   │   └╼ RHS
│           │   │   │   │   │     └╼ Constant
│           │   │   │   │   │       └╼ Integer -> Generic(12), <1209..1211>
│           │   │   │   │   └╼ RHS
│           │   │   │   │     └╼ Constant
│           │   │   │   │       └╼ Integer -> Generic(13), <1214..1216>
│           │   │   │   └╼ RHS
│           │   │   │     └╼ Constant
│           │   │   │       └╼ Integer -> Generic(1), <1219..1220>
│           │   │   └╼ RHS
│           │   │     └╼ Constant
│           │   │       └╼ Integer -> Generic(5), <1224..1225>
│           │   └╼ RHS
│           │     └╼ BinaryOperatorExpression <1230..1345>
│           │       ├╼ Operator -> NotEquals <1341..1343>
│           │       ├╼ LHS
│           │       │ └╼ BinaryOperatorExpression <1230..1340>
│           │       │   ├╼ Operator -> GreaterOrEqual <1335..1337>
│           │       │   ├╼ LHS
│           │       │   │ └╼ BinaryOperatorExpression <1230..1334>
│           │       │   │   ├╼ Operator -> Minus <1330..1331>
│           │       │   │   ├╼ LHS
│           │       │   │   │ └╼ BinaryOperatorExpression <1230..1329>
│           │       │   │   │   ├╼ Operator -> Multiply <1322..1323>
│           │       │   │   │   ├╼ LHS
│           │       │   │   │   │ └╼ BinaryOperatorExpression <1230..1320>
│           │       │   │   │   │   ├╼ Operator -> Multiply <1263..1264>
│           │       │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │ └╼ BinaryOperatorExpression <1230..1261>
│           │       │   │   │   │   │   ├╼ Operator -> BitwiseAnd <1257..1258>
│           │       │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │ └╼ BinaryOperatorExpression <1230..1256>
│           │       │   │   │   │   │   │   ├╼ Operator -> ShiftLeft <1252..1254>
│           │       │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │ └╼ BinaryOperatorExpression <1230..1251>
│           │       │   │   │   │   │   │   │   ├╼ Operator -> Plus <1247..1248>
│           │       │   │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │   │ └╼ Identifier -> "g_GlobalVariable" <1230..1246>
│           │       │   │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │   │       └╼ Integer -> Unsigned(2), <1249..1251>
│           │       │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │       └╼ Integer -> Generic(2), <1255..1256>
│           │       │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │       └╼ Integer -> Generic(31), <1259..1261>
│           │       │   │   │   │   └╼ RHS
│           │       │   │   │   │     └╼ BinaryOperatorExpression <1268..1320>
│           │       │   │   │   │       ├╼ Operator -> Multiply <1317..1318>
│           │       │   │   │   │       ├╼ LHS
│           │       │   │   │   │       │ └╼ BinaryOperatorExpression <1268..1315>
│           │       │   │   │   │       │   ├╼ Operator -> Multiply <1311..1312>
│           │       │   │   │   │       │   ├╼ LHS
│           │       │   │   │   │       │   │ └╼ BinaryOperatorExpression <1268..1309>
│           │       │   │   │   │       │   │   ├╼ Operator -> Equals <1292..1294>
│           │       │   │   │   │       │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │ └╼ BinaryOperatorExpression <1268..1291>
│           │       │   │   │   │       │   │   │   ├╼ Operator -> Plus <1272..1273>
│           │       │   │   │   │       │   │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │   │ └╼ Constant
│           │       │   │   │   │       │   │   │   │   └╼ Integer -> SignedLong(34), <1268..1271>
│           │       │   │   │   │       │   │   │   └╼ RHS
│           │       │   │   │   │       │   │   │     └╼ Identifier -> "_g_AssumeABoolean" <1274..1291>
│           │       │   │   │   │       │   │   └╼ RHS
│           │       │   │   │   │       │   │     └╼ BinaryOperatorExpression <1295..1309>
│           │       │   │   │   │       │   │       ├╼ Operator -> ShiftRight <1305..1307>
│           │       │   │   │   │       │   │       ├╼ LHS
│           │       │   │   │   │       │   │       │ └╼ Identifier -> "g_NiceVar" <1295..1304>
│           │       │   │   │   │       │   │       └╼ RHS
│           │       │   │   │   │       │   │         └╼ Constant
│           │       │   │   │   │       │   │           └╼ Integer -> Generic(3), <1308..1309>
│           │       │   │   │   │       │   └╼ RHS
│           │       │   │   │   │       │     └╼ Constant
│           │       │   │   │   │       │       └╼ Integer -> Generic(34), <1313..1315>
│           │       │   │   │   │       └╼ RHS
│           │       │   │   │   │         └╼ Constant
│           │       │   │   │   │           └╼ Integer -> Generic(4), <1319..1320>
│           │       │   │   │   └╼ RHS
│           │       │   │   │     └╼ Constant
│           │       │   │   │       └╼ Float -> Float(23.6) <1324..1329>
│           │       │   │   └╼ RHS
│           │       │   │     └╼ Constant
│           │       │   │       └╼ Integer -> Generic(45), <1332..1334>
│           │       │   └╼ RHS
│           │       │     └╼ Constant
│           │       │       └╼ Integer -> Generic(25), <1338..1340>
│           │       └╼ RHS
│           │         └╼ Constant
│           │           └╼ Integer -> Generic(0), <1344..1345>
│           └╼ ElseExpression
│             └╼ Constant
│               └╼ Integer -> Generic(77), <1348..1350>
├╼ FunctionDefinition <1357..1382>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ ├╼ FunctionSpecifier -> Inline <1357..1363>
│ │ │ └╼ TypeSpecifier -> Float <1364..1369>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "AdityaG" <1370..1377>
│ │   └╼ FunctionParameters
│ │     └╼ Empty
│ └╼ FunctionBody
│   └╼ Empty
├╼ Declaration <1383..1438>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ TypeQualifier -> Const <1383..1388>
│ │ └╼ TypeSpecifier -> Int <1389..1392>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ FunctionDeclarator
│       ├╼ Identifier -> "NiceFunction" <1393..1405>
│       └╼ FunctionParameters
│         ├╼ FunctionParameter <1406..1422>
│         │ ├╼ DeclarationSpecifiers
│         │ │ ├╼ TypeQualifier -> Const <1406..1411>
│         │ │ └╼ TypeSpecifier -> Int <1412..1415>
│         │ └╼ DirectDeclarator -> "param1" <1416..1422>
│         └╼ FunctionParameter <1424..1436>
│           ├╼ DeclarationSpecifiers
│           │ └╼ TypeSpecifier -> Float <1424..1429>
│           └╼ DirectDeclarator -> "param2" <1430..1436>
└╼ Declaration <1439..1487>
  ├╼ DeclarationSpecifiers
  │ ├╼ TypeSpecifier -> Unsigned <1439..1447>
  │ └╼ TypeSpecifier -> Int <1448..1451>
  └╼ InitDeclaratorList
    └╼ InitDeclarator
      └╼ FunctionDeclarator
        ├╼ Identifier -> "GetSum" <1452..1458>
        └╼ FunctionParameters
          ├╼ FunctionParameter <1459..1472>
          │ ├╼ DeclarationSpecifiers
          │ │ ├╼ TypeSpecifier -> Unsigned <1459..1467>
          │ │ └╼ TypeSpecifier -> Int <1468..1471>
          │ └╼ DirectDeclarator -> None
          └╼ FunctionParameter <1473..1486>
            ├╼ DeclarationSpecifiers
            │ ├╼ TypeSpecifier -> Unsigned <1473..1481>
            │ └╼ TypeSpecifier -> Int <1482..1485>
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
