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
├╼ FunctionDefinition <117..1212>
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
│   └╼ CompoundStatement <133..1208>
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
│     ├╼ ExpressionStatement <790..796>
│     │ └╼ Expression
│     │   └╼ BinaryOperatorExpression <790..795>
│     │     ├╼ Operator -> Multiply <792..793>
│     │     ├╼ LHS
│     │     │ └╼ Constant
│     │     │   └╼ Integer -> Generic(2), <790..791>
│     │     └╼ RHS
│     │       └╼ Constant
│     │         └╼ Integer -> Generic(2), <794..795>
│     ├╼ ExpressionStatement <801..802>
│     │ └╼ Expression
│     │   └╼ Empty
│     ├╼ GotoStatement <807..823>
│     │ └╼ Identifier -> "Checkpoint" <812..822>
│     ├╼ LabeledStatement <825..847>
│     │ ├╼ Identifier -> "Checkpoint" <825..835>
│     │ └╼ LabeledBlock
│     │   └╼ ReturnStatement <837..847>
│     │     └╼ Constant
│     │       └╼ Integer -> Generic(34), <837..847>
│     ├╼ SwitchStatement <853..959>
│     │ ├╼ SwitchExpression
│     │ │ └╼ BinaryOperatorExpression <861..870>
│     │ │   ├╼ Operator -> Plus <867..868>
│     │ │   ├╼ LHS
│     │ │   │ └╼ BinaryOperatorExpression <861..866>
│     │ │   │   ├╼ Operator -> Multiply <863..864>
│     │ │   │   ├╼ LHS
│     │ │   │   │ └╼ Identifier -> "i" <861..862>
│     │ │   │   └╼ RHS
│     │ │   │     └╼ Identifier -> "i" <865..866>
│     │ │   └╼ RHS
│     │ │     └╼ Identifier -> "i" <869..870>
│     │ └╼ SwitchBlock
│     │   └╼ CompoundStatement <877..959>
│     │     ├╼ CaseStatement <886..935>
│     │     │ ├╼ CaseExpression
│     │     │ │ └╼ BinaryOperatorExpression <891..897>
│     │     │ │   ├╼ Operator -> Multiply <893..894>
│     │     │ │   ├╼ LHS
│     │     │ │   │ └╼ Constant
│     │     │ │   │   └╼ Integer -> Generic(2), <891..892>
│     │     │ │   └╼ RHS
│     │     │ │     └╼ Constant
│     │     │ │       └╼ Integer -> Generic(32), <895..897>
│     │     │ └╼ CaseBlock
│     │     │   └╼ ReturnStatement <899..935>
│     │     │     └╼ BinaryOperatorExpression <899..935>
│     │     │       ├╼ Operator -> Multiply <923..924>
│     │     │       ├╼ LHS
│     │     │       │ └╼ Identifier -> "g_GlobalVariable" <906..922>
│     │     │       └╼ RHS
│     │     │         └╼ Identifier -> "g_NiceVar" <925..934>
│     │     └╼ DefaultStatement <944..959>
│     │       └╼ DefaultBlock
│     │         └╼ BreakStatement <953..959>
│     └╼ CompoundStatement <1034..1208>
│       └╼ ReturnStatement <1035..1208>
│         └╼ TernaryOperatorExpression <1035..1208>
│           ├╼ Condition
│           │ └╼ Identifier -> "_g_AssumeABoolean" <1042..1059>
│           ├╼ IfExpression
│           │ └╼ BinaryOperatorExpression <1062..1202>
│           │   ├╼ Operator -> LogicalOr <1083..1085>
│           │   ├╼ LHS
│           │   │ └╼ BinaryOperatorExpression <1062..1082>
│           │   │   ├╼ Operator -> LogicalAnd <1078..1080>
│           │   │   ├╼ LHS
│           │   │   │ └╼ BinaryOperatorExpression <1062..1077>
│           │   │   │   ├╼ Operator -> BitwiseOr <1074..1075>
│           │   │   │   ├╼ LHS
│           │   │   │   │ └╼ BinaryOperatorExpression <1062..1073>
│           │   │   │   │   ├╼ Operator -> BitwiseXor <1069..1070>
│           │   │   │   │   ├╼ LHS
│           │   │   │   │   │ └╼ BinaryOperatorExpression <1062..1068>
│           │   │   │   │   │   ├╼ Operator -> BitwiseAnd <1064..1065>
│           │   │   │   │   │   ├╼ LHS
│           │   │   │   │   │   │ └╼ Constant
│           │   │   │   │   │   │   └╼ Integer -> Generic(4), <1062..1063>
│           │   │   │   │   │   └╼ RHS
│           │   │   │   │   │     └╼ Constant
│           │   │   │   │   │       └╼ Integer -> Generic(12), <1066..1068>
│           │   │   │   │   └╼ RHS
│           │   │   │   │     └╼ Constant
│           │   │   │   │       └╼ Integer -> Generic(13), <1071..1073>
│           │   │   │   └╼ RHS
│           │   │   │     └╼ Constant
│           │   │   │       └╼ Integer -> Generic(1), <1076..1077>
│           │   │   └╼ RHS
│           │   │     └╼ Constant
│           │   │       └╼ Integer -> Generic(5), <1081..1082>
│           │   └╼ RHS
│           │     └╼ BinaryOperatorExpression <1087..1202>
│           │       ├╼ Operator -> NotEquals <1198..1200>
│           │       ├╼ LHS
│           │       │ └╼ BinaryOperatorExpression <1087..1197>
│           │       │   ├╼ Operator -> GreaterOrEqual <1192..1194>
│           │       │   ├╼ LHS
│           │       │   │ └╼ BinaryOperatorExpression <1087..1191>
│           │       │   │   ├╼ Operator -> Minus <1187..1188>
│           │       │   │   ├╼ LHS
│           │       │   │   │ └╼ BinaryOperatorExpression <1087..1186>
│           │       │   │   │   ├╼ Operator -> Multiply <1179..1180>
│           │       │   │   │   ├╼ LHS
│           │       │   │   │   │ └╼ BinaryOperatorExpression <1087..1177>
│           │       │   │   │   │   ├╼ Operator -> Multiply <1120..1121>
│           │       │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │ └╼ BinaryOperatorExpression <1087..1118>
│           │       │   │   │   │   │   ├╼ Operator -> BitwiseAnd <1114..1115>
│           │       │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │ └╼ BinaryOperatorExpression <1087..1113>
│           │       │   │   │   │   │   │   ├╼ Operator -> ShiftLeft <1109..1111>
│           │       │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │ └╼ BinaryOperatorExpression <1087..1108>
│           │       │   │   │   │   │   │   │   ├╼ Operator -> Plus <1104..1105>
│           │       │   │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │   │ └╼ Identifier -> "g_GlobalVariable" <1087..1103>
│           │       │   │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │   │       └╼ Integer -> Unsigned(2), <1106..1108>
│           │       │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │       └╼ Integer -> Generic(2), <1112..1113>
│           │       │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │       └╼ Integer -> Generic(31), <1116..1118>
│           │       │   │   │   │   └╼ RHS
│           │       │   │   │   │     └╼ BinaryOperatorExpression <1125..1177>
│           │       │   │   │   │       ├╼ Operator -> Multiply <1174..1175>
│           │       │   │   │   │       ├╼ LHS
│           │       │   │   │   │       │ └╼ BinaryOperatorExpression <1125..1172>
│           │       │   │   │   │       │   ├╼ Operator -> Multiply <1168..1169>
│           │       │   │   │   │       │   ├╼ LHS
│           │       │   │   │   │       │   │ └╼ BinaryOperatorExpression <1125..1166>
│           │       │   │   │   │       │   │   ├╼ Operator -> Equals <1149..1151>
│           │       │   │   │   │       │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │ └╼ BinaryOperatorExpression <1125..1148>
│           │       │   │   │   │       │   │   │   ├╼ Operator -> Plus <1129..1130>
│           │       │   │   │   │       │   │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │   │ └╼ Constant
│           │       │   │   │   │       │   │   │   │   └╼ Integer -> SignedLong(34), <1125..1128>
│           │       │   │   │   │       │   │   │   └╼ RHS
│           │       │   │   │   │       │   │   │     └╼ Identifier -> "_g_AssumeABoolean" <1131..1148>
│           │       │   │   │   │       │   │   └╼ RHS
│           │       │   │   │   │       │   │     └╼ BinaryOperatorExpression <1152..1166>
│           │       │   │   │   │       │   │       ├╼ Operator -> ShiftRight <1162..1164>
│           │       │   │   │   │       │   │       ├╼ LHS
│           │       │   │   │   │       │   │       │ └╼ Identifier -> "g_NiceVar" <1152..1161>
│           │       │   │   │   │       │   │       └╼ RHS
│           │       │   │   │   │       │   │         └╼ Constant
│           │       │   │   │   │       │   │           └╼ Integer -> Generic(3), <1165..1166>
│           │       │   │   │   │       │   └╼ RHS
│           │       │   │   │   │       │     └╼ Constant
│           │       │   │   │   │       │       └╼ Integer -> Generic(34), <1170..1172>
│           │       │   │   │   │       └╼ RHS
│           │       │   │   │   │         └╼ Constant
│           │       │   │   │   │           └╼ Integer -> Generic(4), <1176..1177>
│           │       │   │   │   └╼ RHS
│           │       │   │   │     └╼ Constant
│           │       │   │   │       └╼ Float -> Float(23.6) <1181..1186>
│           │       │   │   └╼ RHS
│           │       │   │     └╼ Constant
│           │       │   │       └╼ Integer -> Generic(45), <1189..1191>
│           │       │   └╼ RHS
│           │       │     └╼ Constant
│           │       │       └╼ Integer -> Generic(25), <1195..1197>
│           │       └╼ RHS
│           │         └╼ Constant
│           │           └╼ Integer -> Generic(0), <1201..1202>
│           └╼ ElseExpression
│             └╼ Constant
│               └╼ Integer -> Generic(77), <1205..1207>
├╼ FunctionDefinition <1214..1239>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ ├╼ FunctionSpecifier -> Inline <1214..1220>
│ │ │ └╼ TypeSpecifier -> Float <1221..1226>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "AdityaG" <1227..1234>
│ │   └╼ FunctionParameters
│ │     └╼ Empty
│ └╼ FunctionBody
│   └╼ Empty
├╼ Declaration <1240..1295>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ TypeQualifier -> Const <1240..1245>
│ │ └╼ TypeSpecifier -> Int <1246..1249>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ FunctionDeclarator
│       ├╼ Identifier -> "NiceFunction" <1250..1262>
│       └╼ FunctionParameters
│         ├╼ FunctionParameter <1263..1279>
│         │ ├╼ DeclarationSpecifiers
│         │ │ ├╼ TypeQualifier -> Const <1263..1268>
│         │ │ └╼ TypeSpecifier -> Int <1269..1272>
│         │ └╼ DirectDeclarator -> "param1" <1273..1279>
│         └╼ FunctionParameter <1281..1293>
│           ├╼ DeclarationSpecifiers
│           │ └╼ TypeSpecifier -> Float <1281..1286>
│           └╼ DirectDeclarator -> "param2" <1287..1293>
└╼ Declaration <1296..1344>
  ├╼ DeclarationSpecifiers
  │ ├╼ TypeSpecifier -> Unsigned <1296..1304>
  │ └╼ TypeSpecifier -> Int <1305..1308>
  └╼ InitDeclaratorList
    └╼ InitDeclarator
      └╼ FunctionDeclarator
        ├╼ Identifier -> "GetSum" <1309..1315>
        └╼ FunctionParameters
          ├╼ FunctionParameter <1316..1329>
          │ ├╼ DeclarationSpecifiers
          │ │ ├╼ TypeSpecifier -> Unsigned <1316..1324>
          │ │ └╼ TypeSpecifier -> Int <1325..1328>
          │ └╼ DirectDeclarator -> None
          └╼ FunctionParameter <1330..1343>
            ├╼ DeclarationSpecifiers
            │ ├╼ TypeSpecifier -> Unsigned <1330..1338>
            │ └╼ TypeSpecifier -> Int <1339..1342>
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
