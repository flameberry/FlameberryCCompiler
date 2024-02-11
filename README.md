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
  if (_g_AssumeABoolean + 3 < sizeof(const float))
  {
    char ch = 'A';
    return g_NiceVar ? sizeof ch > 1 ? ch : 0 : 11;
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
  sizeof(((5 * 3)));
  sizeof((((i))))++;
  while (i < 10)
  {
    while (1)
      return 2;
    do
    {
      char character = 'Z' + 'D';
    } while (i == sizeof(5 - 3));

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
    case 2 * 32: return --g_GlobalVariable * g_NiceVar;
    default: break;
  }
  // This is a comment to try and mess with the tokenizer :D
  { return _g_AssumeABoolean ? 4 & 12 ^ 13 | 1 && 5 || (++g_GlobalVariable + 2u << 2 & 31) * (((34l + _g_AssumeABoolean == ++g_NiceVar >> 3) * 34) * 4) * 23.6f - 45 >= 25 != 0 : 77; }
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
├╼ FunctionDefinition <117..1307>
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
│   └╼ CompoundStatement <133..1303>
│     ├╼ IfStatement <197..565>
│     │ ├╼ IfExpression
│     │ │ └╼ BinaryOperatorExpression <201..244>
│     │ │   ├╼ Operator -> Less <223..224>
│     │ │   ├╼ LHS
│     │ │   │ └╼ BinaryOperatorExpression <201..222>
│     │ │   │   ├╼ Operator -> Plus <219..220>
│     │ │   │   ├╼ LHS
│     │ │   │   │ └╼ Identifier -> "_g_AssumeABoolean" <201..218>
│     │ │   │   └╼ RHS
│     │ │   │     └╼ Constant
│     │ │   │       └╼ Integer -> Generic(3), <221..222>
│     │ │   └╼ RHS
│     │ │     └╼ SizeofTypeExpression <225..244>
│     │ │       └╼ TypeName
│     │ │         └╼ SpecifierQualifiers
│     │ │           ├╼ Const <232..237>
│     │ │           └╼ Float <238..243>
│     │ ├╼ ThenStatement
│     │ │ └╼ CompoundStatement <249..320>
│     │ │   ├╼ Declaration <254..268>
│     │ │   │ ├╼ DeclarationSpecifiers
│     │ │   │ │ └╼ TypeSpecifier -> Char <254..258>
│     │ │   │ └╼ InitDeclaratorList
│     │ │   │   └╼ InitDeclarator
│     │ │   │     ├╼ DirectDeclarator -> "ch" <259..261>
│     │ │   │     └╼ Initializer
│     │ │   │       └╼ Constant
│     │ │   │         └╼ Character -> 'A'
│     │ │   └╼ ReturnStatement <273..320>
│     │ │     └╼ TernaryOperatorExpression <273..320>
│     │ │       ├╼ Condition
│     │ │       │ └╼ Identifier -> "g_NiceVar" <280..289>
│     │ │       ├╼ IfExpression
│     │ │       │ └╼ TernaryOperatorExpression <292..314>
│     │ │       │   ├╼ Condition
│     │ │       │   │ └╼ BinaryOperatorExpression <292..305>
│     │ │       │   │   ├╼ Operator -> Greater <302..303>
│     │ │       │   │   ├╼ LHS
│     │ │       │   │   │ └╼ SizeofValExpression <292..301>
│     │ │       │   │   │   └╼ Identifier -> "ch" <299..301>
│     │ │       │   │   └╼ RHS
│     │ │       │   │     └╼ Constant
│     │ │       │   │       └╼ Integer -> Generic(1), <304..305>
│     │ │       │   ├╼ IfExpression
│     │ │       │   │ └╼ Identifier -> "ch" <308..310>
│     │ │       │   └╼ ElseExpression
│     │ │       │     └╼ Constant
│     │ │       │       └╼ Integer -> Generic(0), <313..314>
│     │ │       └╼ ElseExpression
│     │ │         └╼ Constant
│     │ │           └╼ Integer -> Generic(11), <317..319>
│     │ └╼ ElseStatement
│     │   └╼ IfStatement <332..565>
│     │     ├╼ IfExpression
│     │     │ └╼ BinaryOperatorExpression <336..351>
│     │     │   ├╼ Operator -> LessOrEqual <346..348>
│     │     │   ├╼ LHS
│     │     │   │ └╼ Identifier -> "g_NiceVar" <336..345>
│     │     │   └╼ RHS
│     │     │     └╼ Constant
│     │     │       └╼ Integer -> Generic(44), <349..351>
│     │     ├╼ ThenStatement
│     │     │ └╼ ReturnStatement <357..368>
│     │     │   └╼ Constant
│     │     │     └╼ Integer -> SignedLong(69), <357..368>
│     │     └╼ ElseStatement
│     │       └╼ IfStatement <376..565>
│     │         ├╼ IfExpression
│     │         │ └╼ BinaryOperatorExpression <380..400>
│     │         │   ├╼ Operator -> Greater <397..398>
│     │         │   ├╼ LHS
│     │         │   │ └╼ Identifier -> "g_GlobalVariable" <380..396>
│     │         │   └╼ RHS
│     │         │     └╼ Constant
│     │         │       └╼ Integer -> Generic(3), <399..400>
│     │         ├╼ ThenStatement
│     │         │ └╼ ReturnStatement <469..481>
│     │         │   └╼ Constant
│     │         │     └╼ Integer -> UnsignedLong(12), <469..481>
│     │         └╼ ElseStatement
│     │           └╼ ReturnStatement <556..565>
│     │             └╼ Constant
│     │               └╼ Integer -> Generic(0), <556..565>
│     ├╼ Declaration <570..580>
│     │ ├╼ DeclarationSpecifiers
│     │ │ └╼ TypeSpecifier -> Int <570..573>
│     │ └╼ InitDeclaratorList
│     │   └╼ InitDeclarator
│     │     ├╼ DirectDeclarator -> "i" <574..575>
│     │     └╼ Initializer
│     │       └╼ Constant
│     │         └╼ Integer -> Generic(0), <578..579>
│     ├╼ ExpressionStatement <583..601>
│     │ └╼ Expression
│     │   └╼ SizeofValExpression <583..597>
│     │     └╼ BinaryOperatorExpression <592..597>
│     │       ├╼ Operator -> Multiply <594..595>
│     │       ├╼ LHS
│     │       │ └╼ Constant
│     │       │   └╼ Integer -> Generic(5), <592..593>
│     │       └╼ RHS
│     │         └╼ Constant
│     │           └╼ Integer -> Generic(3), <596..597>
│     ├╼ ExpressionStatement <604..622>
│     │ └╼ Expression
│     │   └╼ SizeofValExpression <604..621>
│     │     └╼ UnaryOperatorExpression <610..621>
│     │       ├╼ Operator -> PostIncrement <619..621>
│     │       └╼ Expression
│     │         └╼ Identifier -> "i" <614..615>
│     ├╼ WhileStatement <625..774>
│     │ ├╼ WhileExpression
│     │ │ └╼ BinaryOperatorExpression <632..638>
│     │ │   ├╼ Operator -> Less <634..635>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <632..633>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(10), <636..638>
│     │ └╼ WhileBlock
│     │   └╼ CompoundStatement <643..774>
│     │     ├╼ WhileStatement <648..673>
│     │     │ ├╼ WhileExpression
│     │     │ │ └╼ Constant
│     │     │ │   └╼ Integer -> Generic(1), <655..656>
│     │     │ └╼ WhileBlock
│     │     │   └╼ ReturnStatement <664..673>
│     │     │     └╼ Constant
│     │     │       └╼ Integer -> Generic(2), <664..673>
│     │     ├╼ DoWhileStatement <678..754>
│     │     │ ├╼ DoBlock
│     │     │ │ └╼ CompoundStatement <686..720>
│     │     │ │   └╼ Declaration <693..720>
│     │     │ │     ├╼ DeclarationSpecifiers
│     │     │ │     │ └╼ TypeSpecifier -> Char <693..697>
│     │     │ │     └╼ InitDeclaratorList
│     │     │ │       └╼ InitDeclarator
│     │     │ │         ├╼ DirectDeclarator -> "character" <698..707>
│     │     │ │         └╼ Initializer
│     │     │ │           └╼ BinaryOperatorExpression <710..719>
│     │     │ │             ├╼ Operator -> Plus <714..715>
│     │     │ │             ├╼ LHS
│     │     │ │             │ └╼ Constant
│     │     │ │             │   └╼ Character -> 'Z'
│     │     │ │             └╼ RHS
│     │     │ │               └╼ Constant
│     │     │ │                 └╼ Character -> 'D'
│     │     │ └╼ DoWhileExpression
│     │     │   └╼ BinaryOperatorExpression <734..751>
│     │     │     ├╼ Operator -> Equals <736..738>
│     │     │     ├╼ LHS
│     │     │     │ └╼ Identifier -> "i" <734..735>
│     │     │     └╼ RHS
│     │     │       └╼ SizeofValExpression <739..751>
│     │     │         └╼ BinaryOperatorExpression <746..751>
│     │     │           ├╼ Operator -> Minus <748..749>
│     │     │           ├╼ LHS
│     │     │           │ └╼ Constant
│     │     │           │   └╼ Integer -> Generic(5), <746..747>
│     │     │           └╼ RHS
│     │     │             └╼ Constant
│     │     │               └╼ Integer -> Generic(3), <750..751>
│     │     └╼ Declaration <760..774>
│     │       ├╼ DeclarationSpecifiers
│     │       │ └╼ TypeSpecifier -> Int <760..763>
│     │       └╼ InitDeclaratorList
│     │         └╼ InitDeclarator
│     │           ├╼ DirectDeclarator -> "j" <764..765>
│     │           └╼ Initializer
│     │             └╼ BinaryOperatorExpression <768..773>
│     │               ├╼ Operator -> Plus <770..771>
│     │               ├╼ LHS
│     │               │ └╼ Identifier -> "i" <768..769>
│     │               └╼ RHS
│     │                 └╼ Constant
│     │                   └╼ Integer -> Generic(1), <772..773>
│     ├╼ ForStatement <782..893>
│     │ ├╼ ForInitializer
│     │ │ └╼ Declaration <787..797>
│     │ │   ├╼ DeclarationSpecifiers
│     │ │   │ └╼ TypeSpecifier -> Int <787..790>
│     │ │   └╼ InitDeclaratorList
│     │ │     └╼ InitDeclarator
│     │ │       ├╼ DirectDeclarator -> "i" <791..792>
│     │ │       └╼ Initializer
│     │ │         └╼ Constant
│     │ │           └╼ Integer -> Generic(0), <795..796>
│     │ ├╼ ForCondition
│     │ │ └╼ BinaryOperatorExpression <798..804>
│     │ │   ├╼ Operator -> Less <800..801>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <798..799>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(12), <802..804>
│     │ ├╼ ForStepExpression
│     │ │ └╼ BinaryOperatorExpression <806..811>
│     │ │   ├╼ Operator -> Minus <808..809>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <806..807>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(1), <810..811>
│     │ └╼ ForBlock
│     │   └╼ CompoundStatement <816..893>
│     │     ├╼ ForStatement <821..845>
│     │     │ ├╼ ForInitializer
│     │     │ │ └╼ Empty
│     │     │ ├╼ ForCondition
│     │     │ │ └╼ Empty
│     │     │ ├╼ ForStepExpression
│     │     │ │ └╼ Empty
│     │     │ └╼ ForBlock
│     │     │   └╼ ContinueStatement <836..845>
│     │     ├╼ Declaration <851..868>
│     │     │ ├╼ DeclarationSpecifiers
│     │     │ │ └╼ TypeSpecifier -> Int <851..854>
│     │     │ └╼ InitDeclaratorList
│     │     │   └╼ InitDeclarator
│     │     │     ├╼ DirectDeclarator -> "counter" <855..862>
│     │     │     └╼ Initializer
│     │     │       └╼ Constant
│     │     │         └╼ Integer -> Generic(45), <865..867>
│     │     └╼ ReturnStatement <873..893>
│     │       └╼ BinaryOperatorExpression <873..893>
│     │         ├╼ Operator -> Multiply <883..884>
│     │         ├╼ LHS
│     │         │ └╼ Constant
│     │         │   └╼ Integer -> Generic(20), <880..882>
│     │         └╼ RHS
│     │           └╼ Identifier -> "counter" <885..892>
│     ├╼ ExpressionStatement <901..907>
│     │ └╼ Expression
│     │   └╼ BinaryOperatorExpression <901..906>
│     │     ├╼ Operator -> Multiply <903..904>
│     │     ├╼ LHS
│     │     │ └╼ Constant
│     │     │   └╼ Integer -> Generic(2), <901..902>
│     │     └╼ RHS
│     │       └╼ Constant
│     │         └╼ Integer -> Generic(2), <905..906>
│     ├╼ ExpressionStatement <910..911>
│     │ └╼ Expression
│     │   └╼ Empty
│     ├╼ GotoStatement <914..930>
│     │ └╼ Identifier -> "Checkpoint" <919..929>
│     ├╼ LabeledStatement <932..954>
│     │ ├╼ Identifier -> "Checkpoint" <932..942>
│     │ └╼ LabeledBlock
│     │   └╼ ReturnStatement <944..954>
│     │     └╼ Constant
│     │       └╼ Integer -> Generic(34), <944..954>
│     ├╼ SwitchStatement <958..1056>
│     │ ├╼ SwitchExpression
│     │ │ └╼ BinaryOperatorExpression <966..975>
│     │ │   ├╼ Operator -> Plus <972..973>
│     │ │   ├╼ LHS
│     │ │   │ └╼ BinaryOperatorExpression <966..971>
│     │ │   │   ├╼ Operator -> Multiply <968..969>
│     │ │   │   ├╼ LHS
│     │ │   │   │ └╼ Identifier -> "i" <966..967>
│     │ │   │   └╼ RHS
│     │ │   │     └╼ Identifier -> "i" <970..971>
│     │ │   └╼ RHS
│     │ │     └╼ Identifier -> "i" <974..975>
│     │ └╼ SwitchBlock
│     │   └╼ CompoundStatement <980..1056>
│     │     ├╼ CaseStatement <985..1036>
│     │     │ ├╼ CaseExpression
│     │     │ │ └╼ BinaryOperatorExpression <990..996>
│     │     │ │   ├╼ Operator -> Multiply <992..993>
│     │     │ │   ├╼ LHS
│     │     │ │   │ └╼ Constant
│     │     │ │   │   └╼ Integer -> Generic(2), <990..991>
│     │     │ │   └╼ RHS
│     │     │ │     └╼ Constant
│     │     │ │       └╼ Integer -> Generic(32), <994..996>
│     │     │ └╼ CaseBlock
│     │     │   └╼ ReturnStatement <998..1036>
│     │     │     └╼ BinaryOperatorExpression <998..1036>
│     │     │       ├╼ Operator -> Multiply <1024..1025>
│     │     │       ├╼ LHS
│     │     │       │ └╼ UnaryOperatorExpression <1005..1023>
│     │     │       │   ├╼ Operator -> PreDecrement <1005..1007>
│     │     │       │   └╼ Expression
│     │     │       │     └╼ Identifier -> "g_GlobalVariable" <1007..1023>
│     │     │       └╼ RHS
│     │     │         └╼ Identifier -> "g_NiceVar" <1026..1035>
│     │     └╼ DefaultStatement <1041..1056>
│     │       └╼ DefaultBlock
│     │         └╼ BreakStatement <1050..1056>
│     └╼ CompoundStatement <1125..1303>
│       └╼ ReturnStatement <1126..1303>
│         └╼ TernaryOperatorExpression <1126..1303>
│           ├╼ Condition
│           │ └╼ Identifier -> "_g_AssumeABoolean" <1133..1150>
│           ├╼ IfExpression
│           │ └╼ BinaryOperatorExpression <1153..1297>
│           │   ├╼ Operator -> LogicalOr <1174..1176>
│           │   ├╼ LHS
│           │   │ └╼ BinaryOperatorExpression <1153..1173>
│           │   │   ├╼ Operator -> LogicalAnd <1169..1171>
│           │   │   ├╼ LHS
│           │   │   │ └╼ BinaryOperatorExpression <1153..1168>
│           │   │   │   ├╼ Operator -> BitwiseOr <1165..1166>
│           │   │   │   ├╼ LHS
│           │   │   │   │ └╼ BinaryOperatorExpression <1153..1164>
│           │   │   │   │   ├╼ Operator -> BitwiseXor <1160..1161>
│           │   │   │   │   ├╼ LHS
│           │   │   │   │   │ └╼ BinaryOperatorExpression <1153..1159>
│           │   │   │   │   │   ├╼ Operator -> BitwiseAnd <1155..1156>
│           │   │   │   │   │   ├╼ LHS
│           │   │   │   │   │   │ └╼ Constant
│           │   │   │   │   │   │   └╼ Integer -> Generic(4), <1153..1154>
│           │   │   │   │   │   └╼ RHS
│           │   │   │   │   │     └╼ Constant
│           │   │   │   │   │       └╼ Integer -> Generic(12), <1157..1159>
│           │   │   │   │   └╼ RHS
│           │   │   │   │     └╼ Constant
│           │   │   │   │       └╼ Integer -> Generic(13), <1162..1164>
│           │   │   │   └╼ RHS
│           │   │   │     └╼ Constant
│           │   │   │       └╼ Integer -> Generic(1), <1167..1168>
│           │   │   └╼ RHS
│           │   │     └╼ Constant
│           │   │       └╼ Integer -> Generic(5), <1172..1173>
│           │   └╼ RHS
│           │     └╼ BinaryOperatorExpression <1178..1297>
│           │       ├╼ Operator -> NotEquals <1293..1295>
│           │       ├╼ LHS
│           │       │ └╼ BinaryOperatorExpression <1178..1292>
│           │       │   ├╼ Operator -> GreaterOrEqual <1287..1289>
│           │       │   ├╼ LHS
│           │       │   │ └╼ BinaryOperatorExpression <1178..1286>
│           │       │   │   ├╼ Operator -> Minus <1282..1283>
│           │       │   │   ├╼ LHS
│           │       │   │   │ └╼ BinaryOperatorExpression <1178..1281>
│           │       │   │   │   ├╼ Operator -> Multiply <1274..1275>
│           │       │   │   │   ├╼ LHS
│           │       │   │   │   │ └╼ BinaryOperatorExpression <1178..1272>
│           │       │   │   │   │   ├╼ Operator -> Multiply <1213..1214>
│           │       │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │ └╼ BinaryOperatorExpression <1178..1211>
│           │       │   │   │   │   │   ├╼ Operator -> BitwiseAnd <1207..1208>
│           │       │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │ └╼ BinaryOperatorExpression <1178..1206>
│           │       │   │   │   │   │   │   ├╼ Operator -> ShiftLeft <1202..1204>
│           │       │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │ └╼ BinaryOperatorExpression <1178..1201>
│           │       │   │   │   │   │   │   │   ├╼ Operator -> Plus <1197..1198>
│           │       │   │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │   │ └╼ UnaryOperatorExpression <1178..1196>
│           │       │   │   │   │   │   │   │   │   ├╼ Operator -> PreIncrement <1178..1180>
│           │       │   │   │   │   │   │   │   │   └╼ Expression
│           │       │   │   │   │   │   │   │   │     └╼ Identifier -> "g_GlobalVariable" <1180..1196>
│           │       │   │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │   │       └╼ Integer -> Unsigned(2), <1199..1201>
│           │       │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │       └╼ Integer -> Generic(2), <1205..1206>
│           │       │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │       └╼ Integer -> Generic(31), <1209..1211>
│           │       │   │   │   │   └╼ RHS
│           │       │   │   │   │     └╼ BinaryOperatorExpression <1218..1272>
│           │       │   │   │   │       ├╼ Operator -> Multiply <1269..1270>
│           │       │   │   │   │       ├╼ LHS
│           │       │   │   │   │       │ └╼ BinaryOperatorExpression <1218..1267>
│           │       │   │   │   │       │   ├╼ Operator -> Multiply <1263..1264>
│           │       │   │   │   │       │   ├╼ LHS
│           │       │   │   │   │       │   │ └╼ BinaryOperatorExpression <1218..1261>
│           │       │   │   │   │       │   │   ├╼ Operator -> Equals <1242..1244>
│           │       │   │   │   │       │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │ └╼ BinaryOperatorExpression <1218..1241>
│           │       │   │   │   │       │   │   │   ├╼ Operator -> Plus <1222..1223>
│           │       │   │   │   │       │   │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │   │ └╼ Constant
│           │       │   │   │   │       │   │   │   │   └╼ Integer -> SignedLong(34), <1218..1221>
│           │       │   │   │   │       │   │   │   └╼ RHS
│           │       │   │   │   │       │   │   │     └╼ Identifier -> "_g_AssumeABoolean" <1224..1241>
│           │       │   │   │   │       │   │   └╼ RHS
│           │       │   │   │   │       │   │     └╼ BinaryOperatorExpression <1245..1261>
│           │       │   │   │   │       │   │       ├╼ Operator -> ShiftRight <1257..1259>
│           │       │   │   │   │       │   │       ├╼ LHS
│           │       │   │   │   │       │   │       │ └╼ UnaryOperatorExpression <1245..1256>
│           │       │   │   │   │       │   │       │   ├╼ Operator -> PreIncrement <1245..1247>
│           │       │   │   │   │       │   │       │   └╼ Expression
│           │       │   │   │   │       │   │       │     └╼ Identifier -> "g_NiceVar" <1247..1256>
│           │       │   │   │   │       │   │       └╼ RHS
│           │       │   │   │   │       │   │         └╼ Constant
│           │       │   │   │   │       │   │           └╼ Integer -> Generic(3), <1260..1261>
│           │       │   │   │   │       │   └╼ RHS
│           │       │   │   │   │       │     └╼ Constant
│           │       │   │   │   │       │       └╼ Integer -> Generic(34), <1265..1267>
│           │       │   │   │   │       └╼ RHS
│           │       │   │   │   │         └╼ Constant
│           │       │   │   │   │           └╼ Integer -> Generic(4), <1271..1272>
│           │       │   │   │   └╼ RHS
│           │       │   │   │     └╼ Constant
│           │       │   │   │       └╼ Float -> Float(23.6) <1276..1281>
│           │       │   │   └╼ RHS
│           │       │   │     └╼ Constant
│           │       │   │       └╼ Integer -> Generic(45), <1284..1286>
│           │       │   └╼ RHS
│           │       │     └╼ Constant
│           │       │       └╼ Integer -> Generic(25), <1290..1292>
│           │       └╼ RHS
│           │         └╼ Constant
│           │           └╼ Integer -> Generic(0), <1296..1297>
│           └╼ ElseExpression
│             └╼ Constant
│               └╼ Integer -> Generic(77), <1300..1302>
├╼ FunctionDefinition <1309..1334>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ ├╼ FunctionSpecifier -> Inline <1309..1315>
│ │ │ └╼ TypeSpecifier -> Float <1316..1321>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "AdityaG" <1322..1329>
│ │   └╼ FunctionParameters
│ │     └╼ Empty
│ └╼ FunctionBody
│   └╼ Empty
├╼ Declaration <1335..1390>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ TypeQualifier -> Const <1335..1340>
│ │ └╼ TypeSpecifier -> Int <1341..1344>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ FunctionDeclarator
│       ├╼ Identifier -> "NiceFunction" <1345..1357>
│       └╼ FunctionParameters
│         ├╼ FunctionParameter <1358..1374>
│         │ ├╼ DeclarationSpecifiers
│         │ │ ├╼ TypeQualifier -> Const <1358..1363>
│         │ │ └╼ TypeSpecifier -> Int <1364..1367>
│         │ └╼ DirectDeclarator -> "param1" <1368..1374>
│         └╼ FunctionParameter <1376..1388>
│           ├╼ DeclarationSpecifiers
│           │ └╼ TypeSpecifier -> Float <1376..1381>
│           └╼ DirectDeclarator -> "param2" <1382..1388>
└╼ Declaration <1391..1439>
  ├╼ DeclarationSpecifiers
  │ ├╼ TypeSpecifier -> Unsigned <1391..1399>
  │ └╼ TypeSpecifier -> Int <1400..1403>
  └╼ InitDeclaratorList
    └╼ InitDeclarator
      └╼ FunctionDeclarator
        ├╼ Identifier -> "GetSum" <1404..1410>
        └╼ FunctionParameters
          ├╼ FunctionParameter <1411..1424>
          │ ├╼ DeclarationSpecifiers
          │ │ ├╼ TypeSpecifier -> Unsigned <1411..1419>
          │ │ └╼ TypeSpecifier -> Int <1420..1423>
          │ └╼ DirectDeclarator -> None
          └╼ FunctionParameter <1425..1438>
            ├╼ DeclarationSpecifiers
            │ ├╼ TypeSpecifier -> Unsigned <1425..1433>
            │ └╼ TypeSpecifier -> Int <1434..1437>
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
